use std::path::PathBuf;
use std::env;
use std::io; 
use std::fs;
use std::fs::metadata;
use crate::appstate::AppState;

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
    pub currDir:String,
    pub dirs:Box<Vec<String>>, 
    pub fileIndex:usize,
}


impl FileManager {
    pub fn new() -> FileManager{
        let dir = FileManager::getCurrDir();
        FileManager {currDir:String::from(&dir), dirs:Box::new(FileManager::getCurrDirs(String::from(&dir))), fileIndex: 0}
    }

    pub fn getCurrDir() -> String{
         String::from(env::current_dir()
            .unwrap()
            .as_path()
            .to_str()
            .unwrap()
        )
    }

    pub fn isDir(path:String) -> bool {
        metadata(path).unwrap().is_dir()
    }


    pub fn back(path:String) -> String{
        let mut newPath = path.clone();
        newPath.pop();
        let pieces = newPath.split('\\');
    
        let mut piecesJoin:Vec<String> = pieces.map(|p| format!("{}\\", p)).collect();    
    
        if piecesJoin.len() < 2{
            return path;
        }
    
        piecesJoin.remove(piecesJoin.len()-1);
        return piecesJoin.into_iter().collect();
    
    }


    pub fn getCurrDirs(dir:String) -> Vec<String> {

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


    pub fn handleInput(&mut self, key:KeyEvent) {
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
            KeyCode::Backspace => {
                
                let currDir = FileManager::back(String::from(&self.currDir));  
                self.currDir = currDir;
                self.fileIndex = 0;
                if FileManager::isDir(self.currDir.clone()){

                    self.dirs = Box::new(FileManager::getCurrDirs(String::from(&self.currDir))); 
                }

            },
            KeyCode::Enter if self.fileIndex < self.dirs.len() => {
                

                self.currDir = String::from(&self.dirs[self.fileIndex]); 
                
                if(FileManager::isDir(self.currDir.clone())){

                    self.dirs = Box::new(FileManager::getCurrDirs(String::from(&self.currDir))); 
                }
                self.fileIndex = 0;

            }
            _ => {}
        }
    }

    pub fn render(&self,  f: &mut Frame, appState:&  AppState, outter:Rect, isFocused: bool) {
    
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
            .border_style(Style::default().fg(if isFocused {Color::Blue} else {Color::White})),
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
                .style(Style::default().fg(if !FileManager::isDir(currDir.clone()) {Color::Red} else {Color::Blue}))
                .alignment(Alignment::Center);
            f.render_widget(p, filesBounds[c]);

        }

    }

}


