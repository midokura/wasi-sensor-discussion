use super::wasi;
use fraction::Fraction;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::time::Instant;

use wasi::sensor::property::PropertyKey;
use wasi::sensor::property::PropertyValue;
use wasi::sensor::sensor::DeviceError;

use crate::traits::BufferPool;
use crate::traits::SensorDevice;
use crate::traits::SensorDeviceGroup;

#[derive(Clone)]
pub struct DummyDeviceConfig {
    pub width: u32,
    pub height: u32,
    pub frame_duration: Duration,
}

pub struct DummyDeviceGroup {}

pub struct DummyDevice {
    pub pool: Arc<Mutex<Option<Arc<dyn BufferPool + Send + Sync>>>>,
    pub config: Arc<Mutex<DummyDeviceConfig>>,
}

impl DummyDevice {
    pub fn new() -> Result<Self, DeviceError> {
        Ok(Self {
            pool: Arc::new(Mutex::new(None)),
            config: Arc::new(Mutex::new(DummyDeviceConfig {
                width: 640,
                height: 480,
                frame_duration: Duration::from_millis(33),
            })),
        })
    }
}

impl SensorDeviceGroup for DummyDeviceGroup {
    fn list_devices(&self) -> Result<Vec<String>, DeviceError> {
        Ok(vec!["dummy".to_string()])
    }
    fn open_device(&self, name: &str) -> Result<Box<dyn SensorDevice + Send + Sync>, DeviceError> {
        let dev = DummyDevice::new()?;
        Ok(Box::new(dev))
    }
}

fn generate_dummy_image(frame_no: u64, xsz: u32, ysz: u32) -> (u32, u32, Vec<u8>) {
    let mut img = image::RgbImage::new(xsz, ysz);
    let thresh = (xsz as f32 * (frame_no % 100) as f32 / 100 as f32) as u32;
    for (x, y, p) in img.enumerate_pixels_mut() {
        let r = (x as f32 / xsz as f32 * 255 as f32) as u8;
        let g = if x < thresh { 255 } else { 0 };
        let b = (y as f32 / ysz as f32 * 255 as f32) as u8;
        *p = image::Rgb([r, g, b]);
    }
    (img.width(), img.height(), img.into_raw())
}

impl SensorDevice for DummyDevice {
    fn start_streaming(
        &mut self,
        pool: Arc<dyn BufferPool + Send + Sync>,
    ) -> Result<(), DeviceError> {
        let _ = self.pool.lock().unwrap().insert(pool);
        let pool = self.pool.clone();
        let config_mutex = self.config.clone();
        thread::spawn(move || {
            let mut frame_no = 0;
            println!("DummyDevice thread started");
            let mut next_frame = Instant::now();
            while let Some(ref pool) = *pool.lock().unwrap() {
                let config_locked = config_mutex.lock().unwrap();
                let config = config_locked.clone();
                drop(config_locked);
                // dummy image data
                let (width, height, samples) =
                    generate_dummy_image(frame_no, config.width, config.height);
                frame_no += 1;
                let image = wasi::buffer_pool::data_types::Image {
                    dimension: wasi::buffer_pool::data_types::Dimension {
                        width,
                        height,
                        stride_bytes: width * 3,
                        pixel_format: wasi::buffer_pool::data_types::PixelFormat::Rgb24,
                        //pixel_format: wasi::buffer_pool::data_types::PixelFormat::Yuy2,
                    },
                    payload: samples,
                };
                let data = wasi::buffer_pool::buffer_pool::FrameData::ByValue(
                    wasi::buffer_pool::data_types::DataType::Image(image),
                );
                loop {
                    let now = Instant::now();
                    if now >= next_frame {
                        break;
                    }
                    thread::sleep(next_frame - now);
                }
                next_frame += config.frame_duration;
                match pool.try_enqueue(Box::new(data), None) {
                    Ok(_) => println!("DummyDevice generated frame enqueued"),
                    _ => println!("DummyDevice generated frame dropped"),
                }
            }
            println!("DummyDevice thread finished");
        });
        Ok(())
    }
    fn stop_streaming(&mut self) -> Result<(), DeviceError> {
        self.pool.lock().unwrap().take();
        Ok(())
    }
    fn set_property(&mut self, key: PropertyKey, value: PropertyValue) -> Result<(), DeviceError> {
        match key {
            PropertyKey::SamplingRate => {
                let PropertyValue::Fraction(frac) = value else {
                    return Err(DeviceError::InvalidArgument);
                };
                if frac.numerator == 0 || frac.denominator == 0 {
                    return Err(DeviceError::InvalidArgument);
                }
                let frame_duration_sec = frac.denominator as f32 / frac.numerator as f32;
                let mut config = self.config.lock().unwrap();
                config.frame_duration = Duration::from_secs_f32(frame_duration_sec);
            }
            _ => return Err(DeviceError::NotSupported),
        };
        Ok(())
    }
    fn get_property(&mut self, key: PropertyKey) -> Result<PropertyValue, DeviceError> {
        let value = match key {
            PropertyKey::SamplingRate => {
                let config = self.config.lock().unwrap();
                let frame_duration_sec = config.frame_duration.as_secs_f32();
                let frame_duration_sec = Fraction::from(frame_duration_sec);
                let Fraction::Rational(sign, ratio) = frame_duration_sec else {
                    return Err(DeviceError::OutOfRange);
                };
                if sign.is_negative() {
                    return Err(DeviceError::OutOfRange);
                }
                let ratio = ratio.recip();
                PropertyValue::Fraction(wasi::sensor::property::Fraction {
                    numerator: *ratio.numer() as u32,
                    denominator: *ratio.denom() as u32,
                })
            }
            _ => return Err(DeviceError::NotSupported),
        };
        Ok(value)
    }
}
