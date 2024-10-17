
use std::path;
use crate::filyregex::Command;

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
use crate::empty;



pub enum Element{
    FileManager(Box<filemanager::FileManager>),
    Empty(Box<empty::Empty>),
//    TextManager(Vec<String>),
    None,
}

impl From<empty::Empty> for Element {
    fn from(empty:empty::Empty) -> Self {
        return Element::Empty(Box::new(empty));
    }
}

impl From<filemanager::FileManager> for Element {
    fn from(fm:filemanager::FileManager) -> Self {
        return Element::FileManager(Box::new(fm));
    }
}

impl Clone for Element {
    fn clone(&self) -> Element {
        match &self {
            Element::FileManager(fm) => Element::FileManager(fm.clone()),
            Element::Empty(contents) => Element::Empty(contents.clone()),
 //           Element::TextManager(tm) => Element::TextManager(tm),
            Element::None => Element::None,
        
        } 
    }
}

#[derive(Clone)]
pub struct WindowState {
    windowName: String,
    elements:Element,
}


impl WindowState {
    pub fn name(&self) -> String{
        return self.windowName.clone();
    }

    pub fn elements(&self) -> &Element {
        return &self.elements;
    }

    pub fn using_keyboard(&self) -> bool {
        return match &self.elements {
            Element::FileManager(fm) => {
                fm.searching()
            },
            Element::Empty(empty) => {
                empty.searching()
            }
            Element::None => {false}
        };
    }

    pub fn pulling_info(&self)-> String {
        return match &self.elements {
            Element::FileManager(fm) => {
                fm.pulling_info()
            }
            Element::Empty(empty) => {
                empty.pulling_info()
            }
            Element::None => String::from(""),
        }
    }

    pub fn new(name:String, elements: Element) -> WindowState{
        WindowState {windowName:name, elements:elements}
    } 

    pub fn handle_quit(&mut self){
        
    } 
    pub fn is_using_keyboard(&mut self) -> bool{
        return match &self.elements {
            Element::FileManager(fm) => fm.searching(),
            Element::Empty(empty) => empty.searching(),
            Element::None => false
        }
    }
    pub fn handle_input(&mut self, key:KeyEvent) -> Option<Vec<Command>>{        
        match &mut self.elements {
            Element::FileManager(fm) => {
                fm.handle_input(key)
            },
            Element::Empty(emp) => {
                emp.handle_input(key)
            }
  //          Element::TextManager(text) => {},
            Element::None => {None}
        }
    }
    
    
    pub fn render(&self,  f: &mut Frame, appState:&  AppState, outter:Rect, isFocused: bool) { 
        match &self.elements {
            Element::FileManager(fm) => {fm.render(f, appState, outter, isFocused)},
            Element::Empty(emp) => {emp.render(f,appState,outter,isFocused)},
            Element::None => {}
        };  
    }

}
/*
pub fn from(win:&WindowState) -> WindowState{
        WindowState {windowName:String::from(&win.windowName), elements:win.elements.clone()}
    }

    pub fn from(element:Element) -> WindowState {
        WindowState {
            windowName: "empty".to_string(),
            elements: element.clone()
        }
    }
*/

impl From<&WindowState> for WindowState {
    fn from(win:&WindowState) -> WindowState{
        WindowState {windowName:String::from(&win.windowName), elements:win.elements.clone()}
    }
}



impl From<Element> for WindowState {
    fn from(element: Element) -> Self {
         WindowState {
            windowName: "empty".to_string(),
            elements: element.clone()
        }
    }   
}


