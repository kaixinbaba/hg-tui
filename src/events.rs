use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use lazy_static::lazy_static;

use crate::app::{App, AppMode};
use crate::draw;

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
    /// 重绘界面
    Redraw,

    /// 退出应用
    Quit,

    /// 弹出窗口展示消息
    Message(Message),
}

#[derive(Debug, Clone)]
pub enum Message {
    Error(String),

    Warn(String),

    Tips(String),
}

impl Default for Message {
    fn default() -> Self {
        Message::Error(String::default())
    }
}

pub fn handle_key_event(moved_app: Arc<Mutex<App>>) {
    let (sender, receiver) = unbounded();
    std::thread::spawn(move || loop {
        if let Ok(Event::Key(event)) = crossterm::event::read() {
            sender.send(HGEvent::UserEvent(event)).unwrap();
        }
    });
    std::thread::spawn(move || {
        let event_app = moved_app;
        loop {
            if let Ok(HGEvent::UserEvent(key_event)) = receiver.recv() {
                let mut app = event_app.lock().unwrap();
                match (key_event.modifiers, key_event.code) {
                    (KeyModifiers::CONTROL, KeyCode::Char('c'))
                    | (KeyModifiers::CONTROL, KeyCode::Char('d')) => {
                        quit();
                    }
                    (key_modifier, key_code) => match app.mode {
                        AppMode::Search => {
                            handle_search(key_modifier, key_code, &mut app);
                        }

                        AppMode::View => {
                            handle_view(key_modifier, key_code, &mut app);
                        }

                        AppMode::Popup => {
                            handle_popup(key_modifier, key_code, &mut app);
                        }
                    },
                }
            }
        }
    });
}

pub fn redraw() {
    NOTIFY.0.send(HGEvent::NotifyEvent(Notify::Redraw)).unwrap();
}

pub fn quit() {
    NOTIFY.0.send(HGEvent::NotifyEvent(Notify::Quit)).unwrap();
}

pub fn err(msg: String) {
    NOTIFY
        .0
        .send(HGEvent::NotifyEvent(Notify::Message(Message::Error(msg))))
        .unwrap();
}

pub fn warn(msg: String) {
    NOTIFY
        .0
        .send(HGEvent::NotifyEvent(Notify::Message(Message::Warn(msg))))
        .unwrap();
}

pub fn tips(msg: String) {
    NOTIFY
        .0
        .send(HGEvent::NotifyEvent(Notify::Message(Message::Tips(msg))))
        .unwrap();
}

/// 搜索模式
fn handle_search(key_modifier: KeyModifiers, key_code: KeyCode, app: &mut App) {
    match (key_modifier, key_code) {
        (KeyModifiers::CONTROL, KeyCode::Char('j')) | (_, KeyCode::Down) | (_, KeyCode::Esc) => {
            // switch to view
            app.switch_to_view();
            redraw();
        }
        (_, KeyCode::Char(char)) => {
            app.input.handle_char(char);
            redraw();
        }
        (_, KeyCode::Enter) => match app.search() {
            Ok(_) => redraw(),
            Err(e) => {
                err(e.to_string());
                redraw();
            }
        },
        (_, KeyCode::Backspace) => {
            app.input.handle_backspace();
            redraw();
        }
        _ => {}
    }
}

/// 浏览模式
fn handle_view(key_modifier: KeyModifiers, key_code: KeyCode, app: &mut App) {
    match (key_modifier, key_code) {
        (KeyModifiers::CONTROL, KeyCode::Char('k')) | (_, KeyCode::Up) => {
            // switch to view
            app.switch_to_search();
            redraw();
        }
        (_, KeyCode::Char('j')) => {
            app.content.next(1);
            redraw();
        }
        (_, KeyCode::Char('k')) => {
            app.content.prev(1);
            redraw();
        }
        (_, KeyCode::Char('d')) => {
            app.content.next(5);
            redraw();
        }
        (_, KeyCode::Char('u')) => {
            app.content.prev(5);
            redraw();
        }
        (_, KeyCode::Char('0')) => {
            app.content.first();
            redraw();
        }
        (_, KeyCode::Char('G')) => {
            app.content.last();
            redraw();
        }
        _ => {}
    }
}

fn handle_popup(_: KeyModifiers, _: KeyCode, app: &mut App) {
    app.mode = AppMode::Search;
    redraw();
}

pub fn handle_notify(moved_app: Arc<Mutex<App>>) {
    // first draw
    redraw();

    let notify_app = moved_app;

    let notify_recv = NOTIFY.1.clone();

    loop {
        if let Ok(HGEvent::NotifyEvent(notify)) = notify_recv.recv() {
            match notify {
                Notify::Redraw => {
                    let mut app = notify_app.lock().unwrap();

                    draw::redraw(&mut app);
                }
                Notify::Message(msg) => {
                    let mut app = notify_app.lock().unwrap();

                    app.popup(msg);
                }
                Notify::Quit => {
                    break;
                }
            }
        }
    }
}
