use std::{io::IsTerminal, process};

use pancurses::endwin;
use terminal_rain_lightning::{parse_color_arg, simulate_rain};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if !std::io::stdout().is_terminal() || std::env::var("TERM").unwrap_or_default() == "dumb" {
        eprintln!("Error: This program requires a TTY with curses support.");
        process::exit(1);
    }

    let rain_color = parse_color_arg(&args, "--rain-color", "cyan");
    let lightning_color = parse_color_arg(&args, "--lightning-color", "yellow");

    let window = pancurses::initscr();
    if let Err(e) = simulate_rain(&window, &rain_color, &lightning_color) {
        endwin();
        eprintln!("Error: {e}");
        process::exit(1);
    }
    endwin();
}
