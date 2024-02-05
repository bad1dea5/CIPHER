//
//
//

use crossterm::event::{Event as CrosstermEvent, EventStream, KeyEvent, KeyEventKind, MouseEvent};
use futures::{FutureExt, StreamExt};
use std::{
    io::{Error, ErrorKind},
    time::Duration
};
use tokio::{
    sync::mpsc,
    task::JoinHandle,
    time,
};
use crate::app::Result;

//
//
//
#[derive(Debug, Clone, Copy)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16)
}

//
//
//
#[allow(dead_code)]
pub struct EventHandler {
    sender: mpsc::UnboundedSender<Event>,
    receiver: mpsc::UnboundedReceiver<Event>,
    worker: JoinHandle<()>,
}

//
//
//
impl EventHandler {
    pub fn new(tick_rate: u64) -> Result<Self> {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::unbounded_channel();
        let emit = sender.clone();
        let worker = tokio::spawn(async move {
            let mut stream = EventStream::new();
            let mut tick = time::interval(tick_rate);

            loop {
                let tick_delay = tick.tick();
                let event = stream.next().fuse();

                tokio::select! {
                    Some(Ok(evt)) = event => {
                        match evt {
                            CrosstermEvent::Key(key) => {
                                if key.kind == KeyEventKind::Press {
                                    emit.send(Event::Key(key)).unwrap()
                                }
                            }
                            CrosstermEvent::Mouse(mouse) => {
                                emit.send(Event::Mouse(mouse)).unwrap()
                            }
                            CrosstermEvent::Resize(x, y) => {
                                emit.send(Event::Resize(x, y)).unwrap()
                            }
                            CrosstermEvent::Paste(_) => {
                            }
                            CrosstermEvent::FocusLost => {
                            }
                            CrosstermEvent::FocusGained => {
                            }
                        }
                    }
                    _ = tick_delay => {
                        emit.send(Event::Tick).unwrap();
                    }
                }
            }
        });

        Ok(Self { sender, receiver, worker })
    }

    pub async fn next(&mut self) -> Result<Event> {
        self.receiver.recv().await.ok_or(
            Box::new(Error::new(ErrorKind::Other, "An io error occurred"))
        )
    }
}