use glob::glob;
use crate::appstate;
use crate::window::Element;
use std::fs;
use std::path::Path;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Line},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};



pub fn ui_render(f: &mut Frame, appState:&mut appstate::AppState_t) {
    let size = f.size();
    
    let outterLayout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec! [
            Constraint::Percentage(2),
            Constraint::Percentage(96),
            Constraint::Percentage(2),
        ]).split(f.size());

    let mut innerConstraints = vec![];
    if appState.windowStates().len() <= 0 {
        let currDirLayout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Percentage(25),Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(outterLayout[1]);
        f.render_widget(Paragraph::new(String::from("press e to open the explorer"))
    .block(Block::bordered().title("Paragraph"))
    .style(Style::default())
    .alignment(Alignment::Center), currDirLayout[1]);
        return;
    }

    if appState.windowStates().len() <= 0 {
        return;
    }

    for i in 0..appState.windowStates().len() {
        innerConstraints.push(Constraint::Percentage(((100/appState.windowStates().len())).try_into().unwrap()));
    }

    let innerLayout = Layout::default() 
.direction(Direction::Horizontal)
        .constraints(innerConstraints)
        .split(outterLayout[1]);//.split(outerLayout[1]);
    
    

    let currDirLayout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(5),Constraint::Percentage(95)])
        .split(outterLayout[0]);



    let windowState = &appState.windowStates()[appState.curr_win_index() as usize];
    
    f.render_widget(Span::styled(format!("{}, window count {}", windowState.name(), appState.windowStates().len()), Style::default().bg(Color::Blue).fg(Color::Black)), outterLayout[2]); 
    


    for i in 0..appState.windowStates().len(){
        let filesOutter = Layout::default() 
             .direction(Direction::Horizontal)
             .constraints(vec![
                Constraint::Percentage(2),
                Constraint::Percentage(98)
             ]).split(innerLayout[i]);
        appState.windowStates()[i].render(f, &appState, filesOutter[1], i == appState.curr_win_index() as usize); 
    }

     

}

