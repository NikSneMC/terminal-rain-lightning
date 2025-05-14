use std::time::Instant;

use constants::{COLOR_MAP, LIGHTNING_CHANCE, RAIN_CHARS, UPDATE_INTERVAL};
use lightning::LightningBolt;
use pancurses::{
    Attribute, Attributes, ColorPair, Input, Window, curs_set, has_colors, init_pair, start_color,
    use_default_colors,
};
use rand::Rng;

mod constants;
mod lightning;

#[derive(Clone)]
struct Raindrop {
    x: i32,
    y: f32,
    speed: f32,
    char: char,
}

fn setup_colors(
    rain_color: &str,
    lightning_color: &str,
) -> (Option<Attributes>, Option<Attributes>) {
    if !has_colors() {
        return (None, None);
    }
    start_color();
    use_default_colors();
    let rain_fg = COLOR_MAP
        .iter()
        .find(|entry| entry.0 == rain_color)
        .map(|entry| entry.1)
        .unwrap_or(pancurses::COLOR_CYAN);
    let lightning_fg = COLOR_MAP
        .iter()
        .find(|entry| entry.0 == lightning_color)
        .map(|entry| entry.1)
        .unwrap_or(pancurses::COLOR_YELLOW);
    init_pair(1, rain_fg, -1);
    init_pair(2, lightning_fg, -1);
    (
        Some(ColorPair(1) | Attribute::Normal),
        Some(ColorPair(2) | Attribute::Bold),
    )
}

pub fn simulate_rain(
    window: &Window,
    rain_color: &str,
    lightning_color: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    curs_set(0);
    window.nodelay(true);
    window.timeout(0);

    let (rain_attr, lightning_attr) = setup_colors(rain_color, lightning_color);
    let mut raindrops = Vec::new();
    let mut active_bolts = Vec::new();
    let mut is_thunderstorm = false;
    let mut last_update = Instant::now();

    let mut rng = rand::rng();
    let mut rand_f32_iter = rand::rng().random_iter::<f32>();

    loop {
        match window.getch() {
            Some(Input::Character('q')) | Some(Input::Character('Q')) => {
                break;
            }
            Some(Input::Character('t')) | Some(Input::Character('T')) => {
                is_thunderstorm = !is_thunderstorm;
                window.clear();
            }
            Some(Input::KeyResize) => {
                window.clear();
                raindrops.clear();
                active_bolts.clear();
            }
            _ => {}
        }

        let now = Instant::now();
        if now.duration_since(last_update) < UPDATE_INTERVAL {
            std::thread::sleep(UPDATE_INTERVAL - now.duration_since(last_update));
        }
        last_update = Instant::now();

        let (rows, cols) = (window.get_max_y(), window.get_max_x());

        if is_thunderstorm
            && active_bolts.len() < 3
            && rand_f32_iter.next().unwrap() < LIGHTNING_CHANCE
        {
            let start_col = rng.random_range(cols / 4..=3 * cols / 4);
            let start_row = rng.random_range(0..=rows / 5);
            active_bolts.push(LightningBolt::new(
                &mut rng, start_row, start_col, rows, cols,
            ));
        }

        active_bolts.retain_mut(|bolt| bolt.update(&mut rng, &mut rand_f32_iter));

        let randomeration_chance = if is_thunderstorm { 0.5 } else { 0.3 };
        let max_new_drops = if is_thunderstorm { cols / 8 } else { cols / 15 };
        let (min_speed, max_speed) = (0.3, if is_thunderstorm { 1.0 } else { 0.6 });

        if rand_f32_iter.next().unwrap() < randomeration_chance {
            let num_drops = rng.random_range(1..=max_new_drops);
            for _ in 0..num_drops {
                raindrops.push(Raindrop {
                    x: rng.random_range(0..cols),
                    y: 0.0,
                    speed: rng.random_range(min_speed..=max_speed),
                    char: RAIN_CHARS[rng.random_range(0..RAIN_CHARS.len())],
                });
            }
        }

        raindrops.retain_mut(|drop| {
            drop.y += drop.speed;
            drop.y < rows as f32
        });

        window.clear();

        if let Some(lightning_pair) = lightning_attr {
            window.attrset(lightning_pair);
            for bolt in &active_bolts {
                bolt.draw(window);
            }
        }

        if let Some(rain_pair) = rain_attr {
            let rain_attr_with_style = if is_thunderstorm {
                rain_pair | Attribute::Bold
            } else {
                rain_pair | Attribute::Normal
            };
            window.attrset(rain_attr_with_style);
            for drop in &raindrops {
                let y = drop.y as i32;
                if y < rows {
                    window.mvaddch(y, drop.x, drop.char);
                }
            }
        }

        window.refresh();
    }

    Ok(())
}

pub fn parse_color_arg(args: &[String], flag: &str, default: &str) -> String {
    let mut result = default.to_string();
    let mut i = 0;
    while i < args.len() {
        if args[i] == flag {
            if i + 1 < args.len() {
                result = args[i + 1].clone();
            }
            break;
        }
        i += 1;
    }
    result
}
