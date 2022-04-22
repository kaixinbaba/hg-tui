use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use lazy_static::lazy_static;

use crate::app::App;
use crate::draw;

use std::time::Duration;
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
}


pub fn handle_key_event(moved_app: Arc<Mutex<App>>) {
    let (sender, receiver) = unbounded();
    std::thread::spawn(move || loop {
        if let Ok(Event::Key(event)) = crossterm::event::read() {
            sender.send(HGEvent::UserEvent(event)).unwrap();
        }
    });
    let event_app = moved_app;
    loop {
        if let Ok(HGEvent::UserEvent(key_event)) = receiver.recv() {
            match (key_event.modifiers, key_event.code) {

                (KeyModifiers::CONTROL, KeyCode::Char('c')) | (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                    break;
                }
                (_, KeyCode::Char(char)) => {
                    let mut app = event_app.lock().unwrap();
                    app.handle_char(char);
                }
                (_, KeyCode::Enter) => {
                    let mut app = event_app.lock().unwrap();
                    app.search();
                }
                (_, KeyCode::Backspace) => {
                    let mut app = event_app.lock().unwrap();
                    app.remove_char();

                }
                _ => {}

            }
        }
    }

}


pub fn handle_notify(moved_app: Arc<Mutex<App>>) {
    let redraw_tx = NOTIFY.0.clone();
    redraw_tx.send(HGEvent::NotifyEvent(Notify::Redraw)).unwrap();


    std::thread::spawn(move || {
        let notify_app = moved_app;

        let notify_recv = NOTIFY.1.clone();

        loop {

            if let Ok(HGEvent::NotifyEvent(notify)) = notify_recv.recv() {
                match notify {
                    Notify::Redraw => {

                        let mut app = notify_app.lock().unwrap();

                        draw::redraw(&mut app);
                    }
                }
            }
        }
    });
}
