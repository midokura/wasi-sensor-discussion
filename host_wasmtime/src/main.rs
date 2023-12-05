use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::trace;
use wasmtime::component::*;
use wasmtime::Precompiled;
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::ambient_authority;
use wasmtime_wasi::preview2::DirPerms;
use wasmtime_wasi::preview2::FilePerms;
use wasmtime_wasi::preview2::Pollable;
use wasmtime_wasi::preview2::Subscribe;
use wasmtime_wasi::preview2::Table;
use wasmtime_wasi::preview2::WasiCtx;
use wasmtime_wasi::preview2::WasiCtxBuilder;
use wasmtime_wasi::preview2::WasiView;
use wasmtime_wasi::Dir;

mod dummy_device;
mod nokhwa;
mod pool;
mod traits;

use dummy_device::DummyDevice;
use nokhwa::NokhwaDevice;
use pool::SimplePool;
use traits::SensorDevice;

wasmtime::component::bindgen!({
    path: "../wit",
    tracing: true,
    with: {
        "wasi:buffer-pool/buffer-pool/pool": Pool,
        "wasi:sensor/sensor/device": Device,
        "wasi:io/poll": wasmtime_wasi::preview2::bindings::io::poll,
    },
});

trait WasiSensorView {
    fn table(&mut self) -> &mut Table;
    fn pools(&mut self) -> &mut HashMap<String, Arc<dyn traits::BufferPool + Send + Sync>>;
}

pub struct Pool {
    name: String,
    next_frame: Option<(u64, u64, Box<wasi::buffer_pool::buffer_pool::FrameData>)>,
    pool: Arc<dyn traits::BufferPool + Send + Sync>,
}

#[async_trait::async_trait]
impl Subscribe for Pool {
    async fn ready(&mut self) {
        if self.next_frame.is_some() {
            return;
        }
        let frame = self.pool.dequeue().await;
        self.next_frame = Some(frame);
    }
}

pub struct Device {
    device: Box<dyn SensorDevice + Send + Sync>,
}

impl<T: WasiSensorView> wasi::buffer_pool::data_types::Host for T {}
impl<T: WasiSensorView> wasi::buffer_pool::buffer_pool::Host for T {}

impl<T: WasiSensorView> wasi::buffer_pool::buffer_pool::HostMemory for T {
    fn address(
        &mut self,
        res: Resource<wasi::buffer_pool::buffer_pool::Memory>,
    ) -> wasmtime::Result<u64> {
        bail!("not implemented");
    }
    fn size(
        &mut self,
        res: Resource<wasi::buffer_pool::buffer_pool::Memory>,
    ) -> wasmtime::Result<wasi::buffer_pool::buffer_pool::Size> {
        bail!("not implemented");
    }
    fn invalidate(
        &mut self,
        res: Resource<wasi::buffer_pool::buffer_pool::Memory>,
    ) -> Result<Result<(), wasi::buffer_pool::buffer_pool::BufferError>> {
        bail!("not implemented");
    }
    fn drop(
        &mut self,
        res: Resource<wasi::buffer_pool::buffer_pool::Memory>,
    ) -> wasmtime::Result<()> {
        bail!("not implemented");
    }
}

impl<T: WasiSensorView> wasi::buffer_pool::buffer_pool::HostPool for T {
    fn create(
        &mut self,
        mode: wasi::buffer_pool::buffer_pool::BufferingMode,
        size: u32,
        buffer_num: u32,
        name: String,
    ) -> Result<
        Result<
            Resource<wasi::buffer_pool::buffer_pool::Pool>,
            wasi::buffer_pool::buffer_pool::BufferError,
        >,
    > {
        let pool = SimplePool::new(mode, size as usize, buffer_num as usize)?;
        let pool = Arc::new(pool);
        let idx = self.table().push_resource(Pool {
            name: name.clone(),
            next_frame: None,
            pool: pool.clone(),
        })?;
        self.pools().insert(name, pool);
        Ok(Ok(idx))
    }

    fn read_frames(
        &mut self,
        res: Resource<wasi::buffer_pool::buffer_pool::Pool>,
        max_results: u32,
    ) -> Result<
        Result<
            Vec<wasi::buffer_pool::buffer_pool::FrameInfo>,
            wasi::buffer_pool::buffer_pool::BufferError,
        >,
    > {
        let pool = self.table().get_resource_mut(&res)?;
        if max_results == 0 {
            return Ok(Ok(vec![]));
        }
        let Some((sequence_number, timestamp, data)) = pool.pool.try_dequeue() else {
            return Ok(Ok(vec![]));
        };
        let frame = wasi::buffer_pool::buffer_pool::FrameInfo {
            sequence_number: sequence_number,
            timestamp: timestamp,
            data: vec![*data],
        };
        Ok(Ok(vec![frame]))
    }

    fn subscribe(
        &mut self,
        res: Resource<wasi::buffer_pool::buffer_pool::Pool>,
    ) -> Result<Resource<Pollable>> {
        wasmtime_wasi::preview2::subscribe(self.table(), res)
    }

    fn get_statistics(
        &mut self,
        res: Resource<wasi::buffer_pool::buffer_pool::Pool>,
    ) -> Result<
        Result<
            wasi::buffer_pool::buffer_pool::PoolStatistics,
            wasi::buffer_pool::buffer_pool::BufferError,
        >,
    > {
        let pool = self.table().get_resource(&res)?;
        let stats = pool.pool.get_statistics()?;
        Ok(Ok(stats))
    }

    fn drop(
        &mut self,
        res: Resource<wasi::buffer_pool::buffer_pool::Pool>,
    ) -> wasmtime::Result<()> {
        let pool = self.table().get_resource(&res)?;
        let name = pool.name.clone();
        self.table().delete_resource(res)?;
        self.pools().remove(&name);
        Ok(())
    }
}

impl<T: WasiSensorView> wasi::sensor::sensor::HostDevice for T {
    fn open(
        &mut self,
        device_name: String,
    ) -> Result<Result<Resource<wasi::sensor::sensor::Device>, wasi::sensor::sensor::DeviceError>>
    {
        trace!("opening a device {}", device_name);
        let device_impl: Box<dyn SensorDevice + Send + Sync> = match &*device_name {
            "dummy" => Box::new(match DummyDevice::new() {
                Err(e) => return Ok(Err(e)),
                Ok(i) => i,
            }),
            "nokhwa" => Box::new(match NokhwaDevice::new() {
                Err(e) => return Ok(Err(e)),
                Ok(i) => i,
            }),
            _ => return Ok(Err(wasi::sensor::sensor::DeviceError::NotFound)),
        };
        let device = Device {
            device: device_impl,
        };
        let idx = self.table().push_resource(device)?;
        Ok(Ok(idx))
    }

    fn list_names(&mut self) -> Result<Result<Vec<String>, wasi::sensor::sensor::DeviceError>> {
        Ok(Ok(vec!["dummy".to_string(), "nokhwa".to_string()]))
    }

    fn start(
        &mut self,
        res: Resource<wasi::sensor::sensor::Device>,
        buffer_pool: String,
    ) -> Result<Result<(), wasi::sensor::sensor::DeviceError>> {
        let pool = match self.pools().get(&buffer_pool) {
            Some(pool) => pool,
            _ => return Ok(Err(wasi::sensor::sensor::DeviceError::NotFound)),
        };
        let pool = Arc::clone(pool);
        let device = self.table().get_resource_mut(&res)?;
        Ok(device.device.start_streaming(pool))
    }
    fn stop(
        &mut self,
        res: Resource<wasi::sensor::sensor::Device>,
    ) -> Result<Result<(), wasi::sensor::sensor::DeviceError>> {
        Ok(Err(wasi::sensor::sensor::DeviceError::NotSupported))
    }
    fn set_property(
        &mut self,
        res: Resource<wasi::sensor::sensor::Device>,
        key: wasi::sensor::property::PropertyKey,
        value: wasi::sensor::property::PropertyValue,
    ) -> Result<Result<(), wasi::sensor::sensor::DeviceError>> {
        let device = self.table().get_resource_mut(&res)?;
        Ok(device.device.set_property(key, value))
    }
    fn get_property(
        &mut self,
        res: Resource<wasi::sensor::sensor::Device>,
        key: wasi::sensor::property::PropertyKey,
    ) -> Result<Result<wasi::sensor::property::PropertyValue, wasi::sensor::sensor::DeviceError>>
    {
        let device = self.table().get_resource_mut(&res)?;
        Ok(device.device.get_property(key))
    }
    fn drop(&mut self, res: Resource<wasi::sensor::sensor::Device>) -> wasmtime::Result<()> {
        trace!("dropping {:?}", res);
        self.table().delete_resource(res)?;
        Ok(())
    }
}

impl<T: WasiSensorView> wasi::sensor::sensor::Host for T {}

impl<T: WasiSensorView> wasi::sensor::property::Host for T {}

struct State {
    wasi: WasiCtx,
    table: Table,
    pools: HashMap<String, Arc<dyn traits::BufferPool + Send + Sync>>,
}

impl WasiView for State {
    fn table(&self) -> &Table {
        &self.table
    }
    fn table_mut(&mut self) -> &mut Table {
        &mut self.table
    }
    fn ctx(&self) -> &wasmtime_wasi::preview2::WasiCtx {
        &self.wasi
    }
    fn ctx_mut(&mut self) -> &mut wasmtime_wasi::preview2::WasiCtx {
        &mut self.wasi
    }
}

impl WasiSensorView for State {
    fn table(&mut self) -> &mut Table {
        &mut self.table
    }
    fn pools(&mut self) -> &mut HashMap<String, Arc<dyn traits::BufferPool + Send + Sync>> {
        &mut self.pools
    }
}

fn main() -> Result<()> {
    println!("start");
    tracing_subscriber::fmt::init();

    let mut config = Config::new();
    config.wasm_component_model(true);
    config.wasm_backtrace_details(wasmtime::WasmBacktraceDetails::Enable);
    let engine = Engine::new(&config)?;

    let args = std::env::args().collect::<Vec<_>>();
    let args = args.iter().map(|x| &**x).collect::<Vec<&str>>();
    println!("args: {:?}", args);
    let filename = &args[1];
    let guest_args = &args[1..];

    // Note: precompiled modules should have a configuration matching
    // the host. as we enable backtrace details above, you should
    // compile them as:
    // WASMTIME_BACKTRACE_DETAILS=1 wasmtime compile --wasm component-model guest-component.wasm
    let component = match engine.detect_precompiled_file(filename) {
        Ok(Some(Precompiled::Component)) => {
            println!("load a precompiled component");
            unsafe { Component::deserialize_file(&engine, filename) }?
        }
        _ => {
            println!("load a component");
            Component::from_file(&engine, filename)?
        }
    };
    println!("loaded");

    let dir = Dir::open_ambient_dir(".", ambient_authority())?;
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdio()
        .args(guest_args)
        .preopened_dir(dir, DirPerms::all(), FilePerms::all(), ".")
        .build();

    println!("prepare a linker");
    let mut linker = Linker::new(&engine);
    println!("add sensing");
    Sensing::add_to_linker(&mut linker, |s: &mut State| s)?;
    println!("add wasi");
    wasmtime_wasi::preview2::command::sync::add_to_linker(&mut linker)?;

    println!("prepare a store");
    let mut store = Store::new(
        &engine,
        State {
            wasi: wasi_ctx,
            table: Table::new(),
            pools: HashMap::new(),
        },
    );

    println!("instantiate");
    let (bindings, _) = Sensing::instantiate(&mut store, &component, &linker)?;

    println!("calling the entry point");
    let result = bindings.wasi_sensor_interface().call_main(&mut store)?;

    println!("done with result {:?}", result);
    println!(
        "table dump (including 3 entries for stdio): {:#?}",
        store.data().table
    );
    result.or(Err(anyhow!("guest failed")))
}
