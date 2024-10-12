use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{backend::CrosstermBackend, Frame, Terminal};
use std::io::{self};

//changed small thing

mod window;
mod tui;
mod appstate;
mod ui;
mod filemanager;

fn main() -> io::Result<()> {
    // Setup the terminal 
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // Important variables
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut state:appstate::AppState_t = Box::new(appstate::AppState::new());

    // App loop
    run_app(&mut terminal, & mut state)
}

fn run_app(terminal:&mut Terminal<CrosstermBackend<io::Stdout>>, state:& mut appstate::AppState_t) -> io::Result<()>{
    loop {
       
        terminal.draw(|f: &mut Frame| ui::ui_render(f, state))?;       

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(']') if !state.is_using_keyboard() => {
                        state.focus_right();                        
                    }
                    KeyCode::Char('[') if !state.is_using_keyboard() => {
                        state.focus_left();
                    }
                    KeyCode::Char('e') if !state.is_using_keyboard() => {
                        let curr_dirs = filemanager::FileManager::get_curr_dir();  // Get the current directories
                        let file_manager = filemanager::FileManager::new();  // Create a FileManager
                        let element = window::Element::FileManager(Box::new(file_manager));  // Wrap it in an Elemen
                        let win = window::WindowState::new(String::from("files"), element);
                        state.windowStates.push(Box::new(win));
                    },
                    KeyCode::Char('c') if !state.is_using_keyboard() => {
                        let currWin = &state.windowStates[state.currWindow];
                        
                        state.windowStates.push(Box::new(window::WindowState::from(&currWin)));
                    },
                    KeyCode::Char('q') if !state.is_using_keyboard() => {
                        if state.windowStates.len() <= 0 {
                            disable_raw_mode()?;
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            return Ok(());

                        }
                         
                        state.remove_win(state.currWindow);

                    },
                    _ => if state.windowStates.len() > 0 {
                        let windows = &mut state.windowStates; 
                        let index = state.currWindow as usize;
                        windows[index].handle_input(key);
                                                              
                    }
    /*                KeyCode::Left => state.move_horizontal(-1),
                    KeyCode::Right => state.move_horizontal(1),
                    KeyCode::Up => state.move_vertical(-1),
                    KeyCode::Down => state.move_vertical(1),
                    KeyCode::Enter | KeyCode::Char(' ') => state.select_case(),
                    KeyCode::Char('r') => state.reload_game(),
                    _ => {}*/
                }
            }
            continue;
        }
    }
    

}


