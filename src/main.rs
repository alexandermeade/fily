use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::filyregex::Command;
use ratatui::{backend::CrosstermBackend, Frame, Terminal};
use std::io::{self};

//changed small thing

mod window;
mod tui;
mod appstate;
mod ui;
mod filemanager;
mod filyregex;
mod empty;

fn main() -> io::Result<()> {
    // Setup the terminal 
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    // Important variables
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut state:appstate::AppState_t = Box::new(appstate::AppState::new());
    state.evaluate_commands(vec![Command::Win(String::from("hello"))]);

    // App loop
    let res = run_app(&mut terminal, & mut state);
    //goal run: "(someFile[1-4].txt > cat > $temp) $temp > :win"
    //this should index each file of someFile 1 through 4 (if there are any missing indexes these
    //are ignored then it will cat each file into the variable temp then after the coupled
    //comppands are executed pipe the value of temp into the win terminal
    let resRegex = filyregex::execute_fily_regex(Some(String::from("./")), String::from("\"value\""));
   
 //   let mut input = String::new();
    
/*    while true {
        print!("\x1B[2J\x1B[1;1H");
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                filyregex::execute_fily_regex(Some(String::from("./")), String::from(input.clone()));
                input = String::new();
            }
            Err(error) => println!("error: {error}"),
        }
    }
*/

    res 
}



fn run_app(terminal:&mut Terminal<CrosstermBackend<io::Stdout>>, state:& mut appstate::AppState_t) -> io::Result<()>{
    loop { 
        terminal.draw(|f: &mut Frame| ui::ui_render(f, state))?;       
        if state.windowStates().len() <= 0 || state.requests_exit(){
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;                         
            return Ok(());
        }

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && state.windowStates().len() > 0 {
                let mut commands:Vec<filyregex::Command> = Vec::new();
                match state.curr_win() {
                    Some(win) => { 
                        let res = win.handle_input(key);
                            
                        match res {
                            Some(values) => commands = values,
                            None => {}
                        } 
                    },
                    None => {continue;}
                };

                if commands != vec![] {
                    state.evaluate_commands(commands);
                }
            }
        }
    }

}


