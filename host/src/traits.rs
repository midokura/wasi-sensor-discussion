use anyhow::Error;
use std::sync::Arc;

use super::*;
use wasi::buffer_pool::buffer_pool::FrameData;
use wasi::sensor::property::PropertyKey;
use wasi::sensor::property::PropertyValue;
use wasi::sensor::sensor::DeviceError;

pub trait BufferPool {
    fn enqueue(&self, frame: Box<FrameData>, timestamp: Option<u64>) -> Result<(), Error>;
    fn dequeue(&self) -> (u64, u64, Box<FrameData>);
}

pub trait SensorDevice {
    fn start_streaming(
        &mut self,
        pool: Arc<dyn BufferPool + Send + Sync>,
    ) -> Result<(), DeviceError>;
    fn stop_streaming(&mut self) -> Result<(), DeviceError>;
    fn set_property(&mut self, key: PropertyKey, value: PropertyValue) -> Result<(), DeviceError>;
    fn get_property(&mut self, key: PropertyKey) -> Result<PropertyValue, DeviceError>;
}
