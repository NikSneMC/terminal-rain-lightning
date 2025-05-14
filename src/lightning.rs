use std::cmp::{max, min};
use std::time::Instant;

use pancurses::Window;
use rand::Rng;

use crate::constants::{
    FORK_CHANCE, FORK_HORIZONTAL_SPREAD, LIGHTNING_BRANCH_CHANCE, LIGHTNING_GROWTH_DELAY,
    LIGHTNING_MAX_BRANCHES, SEGMENT_LIFESPAN,
};

struct LightningSegment {
    y: i32,
    x: i32,
    created: Instant,
}

pub struct LightningBolt {
    segments: Vec<LightningSegment>,
    last_growth: Instant,
    is_growing: bool,
    max_y: i32,
    max_x: i32,
    target_length: usize,
}

impl LightningBolt {
    pub fn new(rng: &mut impl Rng, start_row: i32, start_col: i32, max_y: i32, max_x: i32) -> Self {
        let target_length = rng.random_range((max_y / 2)..=(max_y - 2)) as usize;
        LightningBolt {
            segments: vec![LightningSegment {
                y: start_row,
                x: start_col,
                created: Instant::now(),
            }],
            last_growth: Instant::now(),
            is_growing: true,
            max_y,
            max_x,
            target_length,
        }
    }

    pub fn update(
        &mut self,
        rng: &mut impl Rng,
        rand_f32_iter: &mut impl Iterator<Item = f32>,
    ) -> bool {
        let now = Instant::now();
        if self.is_growing && now.duration_since(self.last_growth) >= LIGHTNING_GROWTH_DELAY {
            self.last_growth = now;
            let mut added = false;
            let LightningSegment {
                y: last_y,
                x: last_x,
                ..
            } = self.segments.last().unwrap();
            let last_x = *last_x;
            let last_y = *last_y;

            if self.segments.len() < self.target_length && last_y < self.max_y - 1 {
                let mut branches = 1;
                if rand_f32_iter.next().unwrap() < LIGHTNING_BRANCH_CHANCE {
                    branches = rng.random_range(1..=LIGHTNING_MAX_BRANCHES + 1);
                }

                let mut current_x = last_x;
                let next_primary_x = current_x;
                for _ in 0..branches {
                    let offset = rng.random_range(-2..=2);
                    let next_x = max(0, min(self.max_x - 1, current_x + offset));
                    let next_y = min(self.max_y - 1, last_y + 1);
                    self.segments.push(LightningSegment {
                        y: next_y,
                        x: next_x,
                        created: now,
                    });
                    current_x = next_x;
                    added = true;
                }

                if rand_f32_iter.next().unwrap() < FORK_CHANCE {
                    let fork_offset =
                        rng.random_range(-FORK_HORIZONTAL_SPREAD..=FORK_HORIZONTAL_SPREAD);
                    let fork_x = max(0, min(self.max_x - 1, last_x + fork_offset));
                    let fork_y = min(self.max_y - 1, last_y + 1);
                    if fork_x != next_primary_x {
                        self.segments.push(LightningSegment {
                            y: fork_y,
                            x: fork_x,
                            created: now,
                        });
                        added = true;
                    }
                }
            }

            if !added || self.segments.len() >= self.target_length || last_y >= self.max_y - 1 {
                self.is_growing = false;
            }
        }

        let all_expired = self
            .segments
            .iter()
            .all(|s| now.duration_since(s.created) > SEGMENT_LIFESPAN);
        !all_expired
    }

    pub fn draw(&self, window: &Window) {
        let now = Instant::now();
        for segment in &self.segments {
            let age = now.duration_since(segment.created);
            if age > SEGMENT_LIFESPAN {
                continue;
            }
            let norm_age = age.as_secs_f32() / SEGMENT_LIFESPAN.as_secs_f32();
            let char = match norm_age {
                a if a < 0.33 => '*',
                a if a < 0.66 => '+',
                _ => '#',
            };
            if segment.y < window.get_max_y() && segment.x < window.get_max_x() {
                window.mvaddch(segment.y, segment.x, char);
            }
        }
    }
}
