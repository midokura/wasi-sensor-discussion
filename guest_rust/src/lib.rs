use anyhow::bail;
use anyhow::ensure;
use anyhow::Context;
use anyhow::Result;
use image::flat::FlatSamples;
use image::flat::SampleLayout;
use image::ImageBuffer;
use image::Rgb;

use fraction::Fraction;
use getopts::Options;
use std::env;

wit_bindgen::generate!({
    world: "sensing",
    path: "../wit",
    exports:{
        world: T,
        "wasi:sensor/interface": T,
    },
});

struct T;

use crate::exports::wasi::sensor::interface::Guest;

// a naive implementation. maybe it's better to use fix point arithmetic.
// https://hk.interaction-lab.org/firewire/yuv.html
fn yuv_to_rgb(y: i32, u: i32, v: i32) -> Vec<u8> {
    let y = 1.164 * ((y as f32) - 16.0);
    let u = u as f32 - 128.0;
    let v = v as f32 - 128.0;
    let r = y + 1.596 * v;
    let g = y - 0.293 * u - 0.813 * v;
    let b = y + 2.018 * u;
    let r = r.clamp(0.0, 255.0) as u8;
    let g = g.clamp(0.0, 255.0) as u8;
    let b = b.clamp(0.0, 255.0) as u8;
    vec![r, g, b]
}

fn convert_yuy2_to_rgb(width: u32, height: u32, stride: u32, yuv: &Vec<u8>) -> Vec<u8> {
    let size = (3 * width * height) as usize;
    let mut vec = Vec::with_capacity(size);
    yuv.as_slice()
        .chunks_exact(stride as usize)
        .for_each(|row| {
            row.chunks_exact(4).take(width as usize).for_each(|px| {
                let y1 = i32::from(px[0]);
                let u = i32::from(px[1]);
                let y2 = i32::from(px[2]);
                let v = i32::from(px[3]);
                let rgb = yuv_to_rgb(y1, u, v);
                vec.extend(rgb);
                let rgb = yuv_to_rgb(y2, u, v);
                vec.extend(rgb);
            })
        });
    vec
}

fn process_pixel_image(image: &wasi::buffer_pool::data_types::Image) -> Result<()> {
    let dimension = &image.dimension;
    let payload = &image.payload;
    println!(
        "guest: received a frame: dimension {:?} payload len {}",
        dimension,
        payload.len()
    );
    let channels = 3;
    let height_stride = dimension.stride_bytes;

    // convert to rgb
    let converted;
    let (height_stride, payload) = match dimension.pixel_format {
        wasi::buffer_pool::data_types::PixelFormat::Rgb24 => (height_stride, payload),
        wasi::buffer_pool::data_types::PixelFormat::Yuy2 => {
            converted = convert_yuy2_to_rgb(
                dimension.width,
                dimension.height,
                dimension.stride_bytes,
                payload,
            );
            (dimension.width * channels, &converted)
        }
        _ => {
            println!(
                "guest: dropping a frame with unimplemented format {:?}",
                dimension.pixel_format
            );
            return Ok(());
        }
    };

    let layout = SampleLayout {
        channels: channels as u8,
        channel_stride: 1,
        width: dimension.width,
        width_stride: channels as usize,
        height: dimension.height,
        height_stride: height_stride as usize,
    };
    println!("layout {:?} payload len {}", layout, payload.len());
    let flat = FlatSamples {
        samples: &payload[..],
        layout: layout,
        color_hint: None,
    };

    let buffer: ImageBuffer<Rgb<u8>, &[u8]> = flat.try_into_buffer().unwrap();
    let unixtime_ns = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("unix time")
        .as_nanos();
    buffer.save(format!("{}.jpg", unixtime_ns))?;
    Ok(())
}

fn process_frame(frame: &wasi::buffer_pool::buffer_pool::FrameInfo) -> Result<()> {
    println!(
        "got a frame with sequence number {} timestamp {}",
        frame.sequence_number, frame.timestamp
    );
    for (i, ref data) in frame.data.iter().enumerate() {
        println!("frame-data {}", i);
        match data {
            wasi::buffer_pool::buffer_pool::FrameData::ByValue(ref data) => match data {
                wasi::buffer_pool::data_types::DataType::Image(ref image) => {
                    process_pixel_image(image)
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
        .context("data type")?
    }
    Ok(())
}

fn main2() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optopt("", "pool", "pool name", "POOL");
    opts.optopt("", "sensor", "device name", "DEVICE");
    opts.optopt("", "sampling-rate", "samples per sec", "RATE");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            println!("{}", opts.usage(&format!("guest: {}", f.to_string())));
            return Err(f.into());
        }
    };
    let pool_name = matches.opt_str("pool").unwrap_or("my-pool".to_string());
    let device_name = matches.opt_str("sensor").unwrap_or("dummy".to_string());
    let sampling_rate = matches.opt_get::<f32>("sampling-rate")?;
    println!("pool name {}", pool_name);
    println!("sensor device name {}", device_name);
    println!("sampling rate {:?}", sampling_rate);
    let frame_size_in_bytes = 0; // XXX
    let number_of_frames = 16;
    let pool = wasi::buffer_pool::buffer_pool::Pool::create(
        wasi::buffer_pool::buffer_pool::BufferingMode::BufferingDiscard,
        frame_size_in_bytes,
        number_of_frames,
        &pool_name,
    )?;

    let device_names = wasi::sensor::sensor::Device::list_names()?;
    println!("available devices: {:?}", device_names);

    let sensor = wasi::sensor::sensor::Device::open(&device_name)?;
    println!("opened sensor {:?}", sensor);

    let value = sensor.get_property(wasi::sensor::property::PropertyKey::SamplingRate)?;
    println!("sensor default sampling rate {:?}", value);

    if let Some(rate) = sampling_rate {
        let rate = Fraction::from(rate);
        let Fraction::Rational(sign, ratio) = rate else {
            bail!("failed to process sampling rate {}", rate);
        };
        ensure!(sign.is_positive(), "negative sampling rate {}", rate);
        let value =
            wasi::sensor::property::PropertyValue::Fraction(wasi::sensor::property::Fraction {
                numerator: *ratio.numer() as u32,
                denominator: *ratio.denom() as u32,
            });
        println!("setting sampling rate to {:?}", value);
        sensor.set_property(wasi::sensor::property::PropertyKey::SamplingRate, &value)?;
        // confirm the result
        let value = sensor.get_property(wasi::sensor::property::PropertyKey::SamplingRate)?;
        println!("sensor sampling rate {:?}", value);
    }

    println!("starting sensor {:?}", sensor);
    sensor.start(&pool_name)?;
    let poll = pool.subscribe();
    let mut n = 0;
    while n < 60 {
        poll.block();
        let frames = pool.read_frames(1)?;
        assert!(frames.len() == 1);
        for ref frame in &frames {
            process_frame(frame)?;
            n += 1;
        }
    }
    let stats = pool.get_statistics()?;
    println!("pool statistics: {:?}", stats);
    Ok(())
}

impl Guest for T {
    fn main() -> Result<(), ()> {
        println!("Hello, world!");
        main2().expect("main2");
        Ok(())
    }
}
