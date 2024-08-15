use ratatui::crossterm::event::{EnableMouseCapture, DisableMouseCapture};
use ratatui::crossterm::execute;
use ratatui::crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen};
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    //setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(do_print) = res {
        if do_print {
            app.print_json()?;
        }
    } else if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}