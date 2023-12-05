use anyhow::Error;
use anyhow::Result;
use std::sync::Mutex;
use std::time::Instant;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use super::*;
use traits::BufferPool;
use wasi::buffer_pool::buffer_pool::BufferError;
use wasi::buffer_pool::buffer_pool::BufferingMode;
use wasi::buffer_pool::buffer_pool::FrameData;
use wasi::buffer_pool::buffer_pool::PoolStatistics;

struct SimplePoolSequencer {
    sequence_number: u64,
    boottime: Instant,
    sender: Sender<(u64, u64, Box<FrameData>)>,

    /* stats */
    enqueued: u64,
    dropped: u64,
}

struct SimplePoolReceiver {
    receiver: Receiver<(u64, u64, Box<FrameData>)>,

    /* stats */
    dequeued: u64,
}

pub struct SimplePool {
    sequencer: Mutex<SimplePoolSequencer>,
    receiver: Mutex<SimplePoolReceiver>,
}

impl SimplePool {
    pub fn new(mode: BufferingMode, sz: usize, num: usize) -> Result<SimplePool, BufferError> {
        match mode {
            BufferingMode::BufferingDiscard => mode,
            _ => return Err(BufferError::NotSupported),
        };
        let (sender, receiver) = channel(num);
        Ok(Self {
            sequencer: Mutex::new(SimplePoolSequencer {
                sequence_number: 0,
                boottime: Instant::now(),
                sender,
                enqueued: 0,
                dropped: 0,
            }),
            receiver: Mutex::new(SimplePoolReceiver {
                receiver,
                dequeued: 0,
            }),
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
        let result = seq.sender.try_send((seqno, timestamp, frame));
        match result {
            Ok(_) => seq.enqueued += 1,
            Err(e) => {
                seq.dropped += 1;
                return Err(e.into());
            }
        }
        Ok(())
    }
    fn dequeue(&self) -> (u64, u64, Box<FrameData>) {
        let mut receiver = self.receiver.lock().unwrap();
        receiver.dequeued += 1;
        receiver.receiver.blocking_recv().unwrap()
    }
    fn get_statistics(&self) -> Result<PoolStatistics, Error> {
        let seq = self.sequencer.lock().unwrap();
        let enqueued = seq.enqueued;
        let dropped = seq.dropped;
        drop(seq);
        let receiver = self.receiver.lock().unwrap();
        let dequeued = receiver.dequeued;
        drop(receiver);
        Ok(PoolStatistics {
            enqueued: enqueued,
            dequeued: dequeued,
            dropped: dropped,
        })
    }
}
