use std::time::Duration;

pub const UPDATE_INTERVAL: Duration = Duration::from_millis(15);
pub const RAIN_CHARS: [char; 3] = ['|', '.', '`'];
pub const LIGHTNING_CHANCE: f32 = 0.005;
pub const LIGHTNING_GROWTH_DELAY: Duration = Duration::from_micros(2000);
pub const LIGHTNING_MAX_BRANCHES: usize = 2;
pub const LIGHTNING_BRANCH_CHANCE: f32 = 0.3;
pub const FORK_CHANCE: f32 = 0.15;
pub const FORK_HORIZONTAL_SPREAD: i32 = 3;
pub const SEGMENT_LIFESPAN: Duration = Duration::from_millis(800);

#[derive(Clone)]
pub struct ColorMapEntry(pub &'static str, pub i16);

pub const COLOR_MAP: &[ColorMapEntry] = &[
    ColorMapEntry("black", pancurses::COLOR_BLACK),
    ColorMapEntry("red", pancurses::COLOR_RED),
    ColorMapEntry("green", pancurses::COLOR_GREEN),
    ColorMapEntry("yellow", pancurses::COLOR_YELLOW),
    ColorMapEntry("blue", pancurses::COLOR_BLUE),
    ColorMapEntry("magenta", pancurses::COLOR_MAGENTA),
    ColorMapEntry("cyan", pancurses::COLOR_CYAN),
    ColorMapEntry("white", pancurses::COLOR_WHITE),
];
