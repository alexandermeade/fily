use std::path::PathBuf;
use std::env;
use std::io; 
use std::fs;
use std::fs::metadata;
use crate::appstate::AppState;
use crate::filyregex;
use crate::filyregex::Command;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};


use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

#[derive(Clone)]
pub struct FileManager{
    currDir:String,
    dirs:Box<Vec<String>>, 
    fileIndex:usize,
    currRegex:String,
    isSearching:bool
}


impl FileManager {
    
    pub fn new() -> FileManager{
        let dir = FileManager::get_curr_dir();

        FileManager {
            currDir:String::from(&dir), 
            dirs:Box::new(FileManager::get_curr_dirs(String::from(&dir))), 
            fileIndex: 0, 
            currRegex: String::from(""), 
            isSearching: false
        }

    }

    pub fn searching(&self) -> bool {
        return self.isSearching;
    }

    pub fn get_curr_dir() -> String{
         String::from(env::current_dir()
            .unwrap()
            .as_path()
            .to_str()
            .unwrap()
        )
    }

    pub fn is_dir(path:String) -> bool {
        metadata(path).unwrap().is_dir()
    }

    pub fn pulling_info(&self) -> String {
        self.currRegex.clone()
    }
    pub fn back(path:String) -> String{
        let key = '/';
        let mut newPath = path.clone();
        newPath.pop();
        let pieces = newPath.split(key);
    
        let mut piecesJoin:Vec<String> = pieces.map(|p| format!("{}{}", p, key)).collect();    
    
        if piecesJoin.len() < 2{
            return path;
        }
    
        piecesJoin.remove(piecesJoin.len()-1);
        return piecesJoin.into_iter().collect(); 
    }


    pub fn get_curr_dirs(dir:String) -> Vec<String> {

        let mut dirs:Vec<String> = Vec::new();


        match fs::read_dir(dir) {
            Ok(entries) => {
                for entry in entries {
                    match entry {
                        Ok(entry) => dirs.push(String::from(entry.path().to_str().unwrap())),
                        Err(e) =>eprintln!("Error: {}", e),
                    }

                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
        dirs
    }
    
    pub fn handle_input(&mut self, key:KeyEvent) -> Option<Vec<Command>>{
        match key.code {
            KeyCode::Up  => {
                if (self.fileIndex as i32) - 1 >= 0 {
                    self.fileIndex -= 1;
                } 
            },

            KeyCode::Down  => {            
                
                if self.fileIndex +1 < self.dirs.len() { 
                    self.fileIndex += 1;
                }

            },
            KeyCode::Backspace if !self.isSearching => {
                
                let currDir = FileManager::back(String::from(&self.currDir));  
                self.currDir = currDir;
                self.fileIndex = 0;
                
                if FileManager::is_dir(self.currDir.clone()){
                    self.dirs = Box::new(FileManager::get_curr_dirs(String::from(&self.currDir))); 
                }

            },
            KeyCode::Backspace if self.isSearching => {
                self.currRegex.pop();
            },

            KeyCode::Enter if self.fileIndex < self.dirs.len() && !self.isSearching => {
                

                self.currDir = String::from(&self.dirs[self.fileIndex]); 
                
                if FileManager::is_dir(self.currDir.clone()){

                    self.dirs = Box::new(FileManager::get_curr_dirs(String::from(&self.currDir))); 
                }
                self.fileIndex = 0;

            }
            KeyCode::Enter if self.isSearching => {
                let res = self.execute_fily_regex();
                self.isSearching = false;
                self.currRegex = String::from("");
                return Some(res);
            }

            KeyCode::Char('s') if !self.isSearching => {
                self.isSearching = true;
                
            }
            KeyCode::Esc if self.isSearching => {
                self.isSearching = false;
            }
            key => {
                let c:String = match key {
                    KeyCode::Char(key) => {
                        String::from(key)
                    },
                    _ => {String::from("")},
                };
                if self.isSearching {
                    self.currRegex.push_str(&c);
                    return None;
                }
            }
        }
        None
    }

    fn execute_fily_regex(&self) -> Vec<filyregex::Command> {
        filyregex::execute_fily_regex(Some(self.currDir.clone()), self.currRegex.clone())

    }

    pub fn render(&self,  f: &mut Frame, _appState:&AppState, outter:Rect, isFocused: bool) {
    
        let mut constraints = vec![];
        let pad = 4; 
        
        let mut bot = 0; 

        for i in 0..=self.dirs.len()  {
            if i * pad >= 25 && bot == 0 {
                bot = i;
            }
        
            if i * pad >= 100 {
                break;
            }
            constraints.push(Constraint::Percentage(5));
        }


         
         
        f.render_widget(
        Block::new()
            .border_type(BorderType::Rounded)    
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if isFocused {Color::Blue} else {Color::White}))
            .title(format!("{}", self.currDir)),
        outter);
        
        
    

        let filesInner = Layout::default() 
         .direction(Direction::Horizontal)
         .constraints(vec![
            Constraint::Percentage(2),
            Constraint::Percentage(98),
         ]).split(outter);

        let filesBounds = Layout::default() 
             .direction(Direction::Vertical)
             .constraints(constraints)
             .split(filesInner[1]);

    
        
        let mut c:usize = 0;

        let start = if (self.fileIndex as i32) - (bot as i32) <= 9 {0} else {self.fileIndex - bot};

        for i in start..self.dirs.len() {
            c += 1;
            if c * pad >= 100 || c >= filesBounds.len(){
                break;
            }

            let currDir = self.dirs[i].clone(); 
   
            if i == self.fileIndex {
                let p = Paragraph::new(currDir.clone())
                .style(Style::default().bg(Color::Blue).fg(Color::Red))
                .alignment(Alignment::Center);
                f.render_widget(p, filesBounds[c]);
                continue;
            }

            if  filesBounds[c].y > outter.height {
                break;
            }

            let p = Paragraph::new(currDir.clone())
                .style(Style::default().fg(if !FileManager::is_dir(currDir.clone()) {Color::Red} else {Color::Blue}))
                .alignment(Alignment::Center);
            f.render_widget(p, filesBounds[c]);

        }

    }

}


