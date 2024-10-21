
use std::path;
use std::mem;
use std::io::{Error, ErrorKind};

use crate::empty::Empty;
use crate::window;
use crate::filyregex::Command;
use crate::filemanager;

pub type AppState_t  = Box<AppState>;
pub type WinStates = Vec<Box<window::WindowState>>; 

pub struct AppState{
    counter:i32,
    currWindow: usize,
    windowStates: WinStates,
    exit: bool,
}

impl AppState {

    pub fn new() -> AppState {
        let mut state = AppState {
            counter:0, 
            currWindow: 0,
            windowStates:Vec::new(),
            exit: false
        };
        state
    }

    pub fn requests_exit(&self) -> bool {
        return self.exit;
    }

    pub fn evaluate_commands(&mut self, commands:Vec<Command>) {

        //top level commands
        for command in commands {
            match command {
                Command::Win(contents) => {
                    self.push_win(window::WindowState::from(window::Element::from(Empty::new(contents))));
                },
                Command::Quit() => {          
                    self.remove_win(self.curr_win_index());
                },
                Command::Explorer() => {
                        let curr_dirs = filemanager::FileManager::get_curr_dir();  // Get the current directories
                        let file_manager = filemanager::FileManager::new();  // Create a FileManager
                        let element = window::Element::FileManager(Box::new(file_manager));  // Wrap it in an Elemen
                        let win = window::WindowState::new(String::from("balls"), element);
                        self.push_win(win);
 
                },
                Command::CopyWin() => {
                    let currWin = match self.curr_win() {
                        Some(win) => *win.clone(),
                        None => {continue;}
                    };
                    self.push_win(currWin);

                },
                Command::FocusLeft() => {self.focus_left();}
                Command::FocusRight() => {self.focus_right();}

                Command::RequestExit() => {self.exit = true;},
                Command::Unknown | _=> {}
            }
        }
    }

    pub fn curr_win_index(&self) -> usize {
        return self.currWindow;
    }
    pub fn windowStates(&self) -> &WinStates{
        return &self.windowStates;
    }

    pub fn push_win(&mut self, win:window::WindowState) {
        self.windowStates.push(Box::new(win));
    }

    pub fn is_pulling_keys(&self) -> bool {

        if self.windowStates.len() <= 0 {
            return false;
        }

        return self.windowStates[self.currWindow].using_keyboard();
    }

    pub fn curr_win(&mut self) -> Option<&mut Box<window::WindowState>> {
        if self.currWindow >= 0 && self.currWindow < self.windowStates.len() {
            return Some(&mut self.windowStates[self.currWindow]);
        }        
        return None;
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


