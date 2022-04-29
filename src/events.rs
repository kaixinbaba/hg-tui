use crossbeam_channel::{bounded, select, unbounded, Receiver, Sender};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use lazy_static::lazy_static;

use crate::app::{App, AppMode};
use crate::draw;

use std::sync::{Arc, Mutex};
use std::time::Duration;

lazy_static! {
    pub static ref NOTIFY: (Sender<HGEvent>, Receiver<HGEvent>) = bounded(1024);
}

#[derive(Debug, Clone)]
pub enum HGEvent {
    UserEvent(KeyEvent),

    NotifyEvent(Notify),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Notify {
    /// 重绘界面
    Redraw,

    /// 退出应用
    Quit,

    /// 弹出窗口展示消息
    Message(Message),

    /// tick
    Tick,
}

#[derive(Debug, Clone, PartialEq)]
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
                        AppMode::Detail => {
                            handle_detail(key_modifier, key_code, &mut app);
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

pub fn show_help() {
    tips(
        r###"Ctrl j\k 切换 搜索\浏览 模式
搜索模式
输入 :help 获得帮助
输入 #{数字} 按期数搜索
输入 ${类别} 按类别搜索
其他按关键字搜索

浏览模式：
j\k 移动一行
d\u 移动五行
0 移动至首行
G 移动至末行
h/l 翻页
o/enter 查看详细"###
            .into(),
    );
}

pub fn tick() {
    NOTIFY.0.send(HGEvent::NotifyEvent(Notify::Tick)).unwrap();
}

/// 搜索模式
fn handle_search(key_modifier: KeyModifiers, key_code: KeyCode, app: &mut App) {
    match (key_modifier, key_code) {
        (KeyModifiers::CONTROL, KeyCode::Char('j')) | (_, KeyCode::Down) | (_, KeyCode::Esc) => {
            // switch to view
            app.switch_to_view();
            redraw();
        }
        (KeyModifiers::CONTROL, KeyCode::Char('h')) => {
            show_help();
        }
        (_, KeyCode::Char(char)) => {
            let mode = app.input.handle_char(char);
            app.statusline.set_mode(mode);
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
        (KeyModifiers::CONTROL, KeyCode::Char('h')) => {
            show_help();
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
        (_, KeyCode::Char('l')) => {
            app.next_page().unwrap();
            redraw();
        }
        (_, KeyCode::Char('h')) => {
            app.prev_page().unwrap();
            redraw();
        }
        (_, KeyCode::Char('o')) | (_, KeyCode::Enter) => {
            // 进入详情页
            app.display_detail().unwrap();
            redraw();
            // redraw();
        }
        _ => {}
    }
}

fn handle_popup(_: KeyModifiers, _: KeyCode, app: &mut App) {
    app.mode = AppMode::Search;
    redraw();
}

fn handle_detail(key_modifier: KeyModifiers, key_code: KeyCode, app: &mut App) {
    match (key_modifier, key_code) {
        (_, KeyCode::Char('o')) | (_, KeyCode::Esc) => {
            app.mode = AppMode::View;
            redraw();
        }
        _ => {}
    }
}

pub fn handle_notify(moved_app: Arc<Mutex<App>>) {
    // first draw
    redraw();

    std::thread::spawn(move || loop {
        tick();
        std::thread::sleep(Duration::from_secs(1));
    });

    let notify_app = moved_app;

    let notify_recv = NOTIFY.1.clone();

    loop {
        if let Ok(HGEvent::NotifyEvent(notify)) = notify_recv.recv() {
            match notify {
                Notify::Redraw | Notify::Tick => {
                    let mut app = notify_app.lock().unwrap();

                    draw::redraw(&mut app);
                }
                Notify::Message(msg) => {
                    let mut app = notify_app.lock().unwrap();
                    app.popup(msg);

                    draw::redraw(&mut app);
                }
                Notify::Quit => {
                    break;
                }
            }
        }
    }
}
