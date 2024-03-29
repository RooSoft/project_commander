/// Terminal events.
#[derive(Clone, Copy, Debug)]
pub enum Event {
    /// Terminal tick.
    Tick,
    /// Key press.
    Key(KeyEvent),
    /// Mouse click/scroll.
    Mouse(MouseEvent),
    /// Terminal resize.
    Resize(u16, u16),
}

use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    #[allow(dead_code)]
    sender: mpsc::Sender<Event>,
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,
    /// Event handler thread.
    #[allow(dead_code)]
    handler: thread::JoinHandle<()>,
}

use color_eyre::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};

// -- snip --

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`].
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = Self::create_handler(sender.clone(), tick_rate);

        Self {
            sender,
            receiver,
            handler,
        }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if
    /// there is no data available and it's possible for more data to be sent.
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }

    fn create_handler(sender: mpsc::Sender<Event>, tick_rate: Duration) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = Self::create_timeout(tick_rate, last_tick);

                Self::poll_event(&sender, timeout);

                if last_tick.elapsed() >= tick_rate {
                    sender.send(Event::Tick).expect("failed to send tick event");
                    last_tick = Instant::now();
                }
            }
        })
    }

    fn poll_event(sender: &mpsc::Sender<Event>, timeout: Duration) {
        if event::poll(timeout).expect("unable to poll for event") {
            match event::read().expect("unable to read event") {
                CrosstermEvent::Key(e) => {
                    if e.kind == event::KeyEventKind::Press {
                        sender.send(Event::Key(e))
                    } else {
                        Ok(()) // ignore KeyEventKind::Release on windows
                    }
                }
                CrosstermEvent::Mouse(e) => sender.send(Event::Mouse(e)),
                CrosstermEvent::Resize(w, h) => sender.send(Event::Resize(w, h)),
                _ => unimplemented!(),
            }
            .expect("failed to send terminal event")
        }
    }

    fn create_timeout(tick_rate: Duration, last_tick: Instant) -> Duration {
        if let Some(timeout) = tick_rate.checked_sub(last_tick.elapsed()) {
            timeout
        } else {
            tick_rate
        }
    }
}
