use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use lazy_static::lazy_static;

use crate::app::{App, AppMode};
use crate::draw;

use std::sync::{Arc, Mutex};
use std::time::Duration;

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
    Message(String),
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
                    },
                }
            }
        }
    });
}

fn redraw() {
    NOTIFY.0.send(HGEvent::NotifyEvent(Notify::Redraw)).unwrap();
}

fn quit() {
    NOTIFY.0.send(HGEvent::NotifyEvent(Notify::Quit)).unwrap();
}

fn msg(msg: String) {
    NOTIFY
        .0
        .send(HGEvent::NotifyEvent(Notify::Message(msg)))
        .unwrap();
}

/// 搜索模式
fn handle_search(key_modifier: KeyModifiers, key_code: KeyCode, app: &mut App) {
    match (key_modifier, key_code) {
        (KeyModifiers::CONTROL, KeyCode::Char('j')) | (_, KeyCode::Down) => {
            // switch to view
            app.switch_to_view();
            redraw();
        }
        (_, KeyCode::Char(char)) => {
            app.input.handle_char(char);
            redraw();
        }
        (_, KeyCode::Enter) => {
            app.search().unwrap();
            redraw();
        }
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
            // move to next
            app.content.next();
            redraw();
        }
        (_, KeyCode::Char('k')) => {
            // move to prev
            app.content.prev();
            redraw();
        }
        _ => {}
    }
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
                    todo!()
                }
                Notify::Quit => {
                    break;
                }
            }
        }
    }
}
