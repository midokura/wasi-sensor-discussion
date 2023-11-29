use anyhow::Error;
use anyhow::Result;
use std::sync::mpsc::sync_channel;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::sync::Mutex;
use std::time::Instant;

use super::*;
use traits::BufferPool;
use wasi::buffer_pool::buffer_pool::BufferError;
use wasi::buffer_pool::buffer_pool::BufferingMode;
use wasi::buffer_pool::buffer_pool::FrameData;

struct SimplePoolSequencer {
    sequence_number: u64,
    boottime: Instant,
    sender: SyncSender<(u64, u64, Box<FrameData>)>,
}

pub struct SimplePool {
    sequencer: Mutex<SimplePoolSequencer>,
    receiver: Mutex<Receiver<(u64, u64, Box<FrameData>)>>,
}

impl SimplePool {
    pub fn new(mode: BufferingMode, sz: usize, num: usize) -> Result<SimplePool, BufferError> {
        match mode {
            BufferingMode::BufferingDiscard => mode,
            _ => return Err(BufferError::NotSupported),
        };
        let (sender, receiver) = sync_channel(num);
        Ok(Self {
            sequencer: Mutex::new(SimplePoolSequencer {
                sequence_number: 0,
                boottime: Instant::now(),
                sender,
            }),
            receiver: Mutex::new(receiver),
        })
    }
}

impl BufferPool for SimplePool {
    fn enqueue(&self, frame: Box<FrameData>, timestamp: Option<u64>) -> Result<(), Error> {
        let mut seq = self.sequencer.lock().unwrap();
        let timestamp = match timestamp {
            Some(t) => t,
            _ => seq.boottime.elapsed().as_nanos() as u64,
        };
        let seqno = seq.sequence_number;
        seq.sequence_number += 1;
        seq.sender.try_send((seqno, timestamp, frame))?;
        Ok(())
    }
    fn dequeue(&self) -> (u64, u64, Box<FrameData>) {
        self.receiver.lock().unwrap().recv().unwrap()
    }
}
