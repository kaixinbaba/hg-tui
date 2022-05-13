use crossbeam_channel::{bounded, unbounded, Receiver, Sender};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use lazy_static::lazy_static;

use crate::app::{App, AppMode};
use crate::app_global::HG_INFO;
use crate::draw;

use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Duration;

lazy_static! {
    pub static ref NOTIFY: (Sender<HGEvent>, Receiver<HGEvent>) = bounded(1024);
    pub static ref GG_COMBINE: AtomicBool = AtomicBool::new(false);
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

pub fn handle_key_event(event_app: Arc<Mutex<App>>) {
    let (sender, receiver) = unbounded();
    sender
        .send(HGEvent::UserEvent(KeyEvent {
            code: KeyCode::Char('#'),
            modifiers: KeyModifiers::NONE,
        }))
        .unwrap();
    let max_volume = HG_INFO.max_volume.to_string();

    for c in max_volume.chars() {
        sender
            .send(HGEvent::UserEvent(KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::NONE,
            }))
            .unwrap();
    }
    sender
        .send(HGEvent::UserEvent(KeyEvent {
            code: KeyCode::Enter,
            modifiers: KeyModifiers::NONE,
        }))
        .unwrap();

    std::thread::spawn(move || loop {
        if let Ok(Event::Key(event)) = crossterm::event::read() {
            sender.send(HGEvent::UserEvent(event)).unwrap();
        }
    });
    std::thread::spawn(move || loop {
        if let Ok(HGEvent::UserEvent(key_event)) = receiver.recv() {
            let mut app = event_app.lock().unwrap();
            match (key_event.modifiers, key_event.code) {
                (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                    quit();
                    break;
                }
                (_, KeyCode::Char('q')) if app.mode != AppMode::Search => {
                    quit();
                    break;
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
        r###"CTRL j/k 切换 浏览/搜索 模式
搜索模式
Ctrl+h 获得帮助
输入 #{数字} 按期数搜索
输入 ${类别} 按类别搜索
其他按关键字搜索

浏览模式：
k/j 上/下 移动一行
u/d 上/下 移动五行
gg 移动至首行
G  移动至末行
h/l 前/后 翻页
o 查看（关闭）详细
s 帮 HG 点个小星星吧
ENTER 打开 GitHub 页面
q 退出应用"###
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
        (_, KeyCode::Enter) => match app.search(None) {
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
        (_, KeyCode::Char('g')) => {
            if GG_COMBINE.load(std::sync::atomic::Ordering::Relaxed) {
                app.content.first();
                redraw();
                GG_COMBINE.store(false, std::sync::atomic::Ordering::Relaxed);
            } else {
                GG_COMBINE.store(true, std::sync::atomic::Ordering::Relaxed);
            }
        }
        (key_modifier, key_code) => {
            GG_COMBINE.store(false, std::sync::atomic::Ordering::Relaxed);
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
                (_, KeyCode::Char('G')) => {
                    app.content.last();
                    redraw();
                }
                (_, KeyCode::Char('s')) => {
                    app.open_browser(Some("https://github.com/521xueweihan/HelloGitHub"))
                        .unwrap();
                }
                (_, KeyCode::Char('l')) => {
                    app.next_page().unwrap();
                    redraw();
                }
                (_, KeyCode::Char('h')) => {
                    app.prev_page().unwrap();
                    redraw();
                }
                (_, KeyCode::Char('o')) => {
                    // 进入详情页
                    app.display_detail().unwrap();
                    redraw();
                }
                (_, KeyCode::Enter) => {
                    // 浏览器打开项目地址
                    app.open_browser(None).unwrap();
                }
                _ => {}
            }
        }
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
        (_, KeyCode::Enter) => {
            // 浏览器打开项目地址
            app.open_browser(None).unwrap();
        }
        _ => {}
    }
}

pub fn handle_notify(notify_app: Arc<Mutex<App>>) {
    // first draw
    redraw();

    if notify_app.lock().unwrap().show_help {
        show_help();
    }

    std::thread::spawn(move || loop {
        tick();
        std::thread::sleep(Duration::from_secs(1));
    });

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
