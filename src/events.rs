use crossterm::event::{KeyEvent, KeyCode};


#[derive(Debug, Clone)]
pub enum HGEvent {

    UserEvent(KeyEvent),

    NotifyEvent(Notify),

}

#[derive(Debug, Clone)]
pub enum Notify {

}
