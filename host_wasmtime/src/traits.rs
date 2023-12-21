use anyhow::Error;
use std::sync::Arc;

use super::*;
use wasi::buffer_pool::buffer_pool::FrameData;
use wasi::buffer_pool::buffer_pool::PoolStatistics;
use wasi::sensor::property::PropertyKey;
use wasi::sensor::property::PropertyValue;
use wasi::sensor::sensor::DeviceError;

#[async_trait::async_trait]
pub trait BufferPool {
    fn try_enqueue(&self, frame: Box<FrameData>, timestamp: Option<u64>) -> Result<(), Error>;
    fn try_dequeue(&self) -> Option<(u64, u64, Box<FrameData>)>;
    async fn dequeue(&self) -> (u64, u64, Box<FrameData>);
    fn get_statistics(&self) -> Result<PoolStatistics, Error>;
}

pub trait SensorDeviceGroup {
    fn list_devices(&self) -> Result<Vec<String>, DeviceError>;
    fn open_device(&self, name: &str) -> Result<Box<dyn SensorDevice + Send + Sync>, DeviceError>;
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
