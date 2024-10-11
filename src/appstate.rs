
use std::path;
use std::mem;
use std::io::{Error, ErrorKind};

use crate::window;

pub type AppState_t  = Box<AppState>;

pub struct AppState{
    pub counter:i32,
    pub currWindow: usize,
    pub windowStates: Vec<Box<window::WindowState>>,
}

impl AppState{

    pub fn new() -> AppState {
        let mut state = AppState {
            counter:0, 
            currWindow: 0,
            windowStates:Vec::new()
        };
        state
    }

    pub fn focus_left(&mut self) {
        
        if self.windowStates.len() <= 0 {
            return;
        }

        if (self.currWindow as i32) - 1 < 0{
            let index = (self.windowStates.len() as i32) - 1; 
            self.currWindow = if index < 0 {0} else {index as usize}; 
            return;
        }
        
        self.currWindow -= 1;
    }

    pub fn focus_right(&mut self) {

        if self.windowStates.len() <= 0 {
            return;
        }

        if self.currWindow + 1 > self.windowStates.len()-1{
            self.currWindow = 0;
            return;
        }

        self.currWindow += 1;

    }

    pub fn is_using_keyboard(&mut self) -> bool {
        if self.windowStates.len() <= 0 {
            return false;
        }
        self.windowStates[self.currWindow].is_using_keyboard() 
    }

    pub fn remove_win(&mut self, index:usize) {

        self.windowStates[self.currWindow].handle_quit();
        self.windowStates.remove(index);

        self.focus_left();
    }

    fn inc(& mut self) {
        self.counter += 1;
    }

}


