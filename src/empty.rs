use crate::filyregex;
use std::path::PathBuf;
use std::env;
use std::io; 
use std::fs;
use std::fs::metadata;
use crate::appstate::AppState;
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
pub struct Empty {
    contents:String,
    lineIndex:usize,
    isSearching:bool,
    currRegex: String
}


impl Empty {
    
    pub fn new(contents:String) -> Empty { 
        Empty {
            contents,
            lineIndex:0,
            isSearching: false,
            currRegex: String::from("")
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


    pub fn pulling_info(&self) -> String {
        self.currRegex.clone()
    }


    pub fn handle_input(&mut self, key:KeyEvent) -> Option<Vec<Command>> {
        //sub commands
        match key.code {
            KeyCode::Up  => {
                if (self.lineIndex as i32) - 1 >= 0 {
                    self.lineIndex -= 1;
                } 
            },
            KeyCode::Down  => {            
                
                if self.lineIndex +1 < self.contents.split("\n").collect::<Vec<_>>().len() { 
                    self.lineIndex += 1;
                }

            },
            KeyCode::Backspace if !self.isSearching => {

            },
            KeyCode::Backspace if self.isSearching => {
                self.currRegex.pop();
            },

            KeyCode::Enter if self.isSearching => {
                let res = self.execute_fily_regex();
                self.isSearching = false;
                self.currRegex = String::from("");
                return Some(res);
            }   

            KeyCode::Char('s') if !self.isSearching => {
                self.isSearching = true;
                
            }
            
            KeyCode::Char(':') if !self.isSearching => {
                self.currRegex.push_str(":");
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
        filyregex::execute_fily_regex(None, self.currRegex.clone())
    }

    pub fn render(&self,  f: &mut Frame, _appState:&AppState, outter:Rect, isFocused: bool) {
    
        let contents = self.contents.split("\n").into_iter().collect::<Vec<_>>();
        
        let mut constraints = vec![];
        let pad = 4; 
        
        let mut bot = 0; 

        for i in 0..=contents.len()  {
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
            .title("window view"),
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

        let start = if (self.lineIndex as i32) - (bot as i32) <= 9 {0} else {self.lineIndex - bot};

        for i in start..contents.len() {
            c += 1;
            if c * pad >= 100 || c >= filesBounds.len(){
                break;
            }

            let currLine = contents[i].clone(); 

            if  filesBounds[c].y > outter.height {
                break;
            }

            let p = Paragraph::new(contents[i].clone())
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Center);
            f.render_widget(p, filesBounds[c]);

        }

    }

}


