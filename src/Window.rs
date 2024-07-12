
use std::path;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crossterm::event::KeyEvent;
use crate::filemanager;
use crate::appstate::AppState;

pub enum Element{
    FileManager(Box<filemanager::FileManager>),
//    TextManager(Vec<String>),
    None,
}

impl Clone for Element {
    fn clone(&self) -> Element {
        match &self {
            Element::FileManager(fm) => Element::FileManager(fm.clone()),
 //           Element::TextManager(tm) => Element::TextManager(tm),
            Element::None => Element::None,
        } 
    }
}

pub struct WindowState {
    pub windowName: String,
    pub elements:Element,
}


impl WindowState {
    pub fn new(name:String, elements: Element) -> WindowState{
        WindowState {windowName:name, elements:elements}
    }
    pub fn from(win:&WindowState) -> WindowState{
        WindowState {windowName:String::from(&win.windowName), elements:win.elements.clone()}
    }
    pub fn handle_quit(&mut self){
        
    } 
    pub fn is_using_keyboard(&mut self) -> bool{
        return match &self.elements {
            Element::FileManager(fm) => fm.isSearching,
            Element::None => false
        }
    }
    pub fn handle_input(&mut self, key:KeyEvent) {        
        match &mut self.elements {
            Element::FileManager(fm) => {
                fm.handle_input(key);
            },
  //          Element::TextManager(text) => {},
            Element::None => {}
        }
    }
    
    
    pub fn render(&self,  f: &mut Frame, appState:&  AppState, outter:Rect, isFocused: bool) { 
        match &self.elements {
            Element::FileManager(fm) => {fm.render(f, appState, outter, isFocused)},
    //        Element::TextManager(text) => {},
            Element::None => {}
        };  
    }

}


