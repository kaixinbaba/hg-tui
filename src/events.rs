use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use crossterm::event::{Event, KeyCode, KeyEvent};

use lazy_static::lazy_static;

use crate::app::App;

use std::sync::{Arc, Mutex};

lazy_static! {
    pub static ref NOTIFY: (Sender<HGEvent>, Receiver<HGEvent>) = bounded(1);
}

#[derive(Debug, Clone)]
pub enum HGEvent {
    UserEvent(KeyEvent),

    NotifyEvent(Notify),
}

#[derive(Debug, Clone)]
pub enum Notify {
    Redraw,
    Quit,
}

pub fn handle_notify(moved_app: Arc<Mutex<App>>) {
    std::thread::spawn(move || {
        let notify_app = moved_app;

        let notify_recv = NOTIFY.1.clone();

        loop {

            if let Ok(HGEvent::NotifyEvent(notify)) = notify_recv.recv() {
                match notify {
                    Notify::Redraw => {
                        todo!()
                    }
                    Notify::Quit => {
                        todo!()
                    }
                }
            }
        }
    });
}

pub fn setup_key_handler() -> Receiver<HGEvent> {
    let (sender, receiver) = unbounded();
    std::thread::spawn(move || loop {
        if let Ok(Event::Key(event)) = crossterm::event::read() {
            sender.send(HGEvent::UserEvent(event)).unwrap();
        }
    });

    receiver
}
