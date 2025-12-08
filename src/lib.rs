use reqwest::{blocking::Client, header::COOKIE};

#[allow(clippy::missing_panics_doc)]
pub fn fetch_aoc_input(day: u8) -> String {
    let session =
        std::env::var("AOC_SESSION").expect("AOC_SESSION environment variable must be set");

    let url = format!("https://adventofcode.com/2025/day/{day}/input");

    let response = Client::new()
        .get(&url)
        .header(COOKIE, format!("session={session}"))
        .send()
        .expect("Failed to fetch input");

    response
        .text()
        .expect("Failed to read response text")
        .trim()
        .to_string()
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Pos2D {
    pub x: i32,
    pub y: i32,
}

impl Pos2D {
    #[must_use]
    pub fn neighbours(&self) -> Vec<Self> {
        vec![
            Self {
                x: self.x - 1,
                y: self.y - 1,
            },
            Self {
                x: self.x,
                y: self.y - 1,
            },
            Self {
                x: self.x + 1,
                y: self.y - 1,
            },
            Self {
                x: self.x - 1,
                y: self.y,
            },
            Self {
                x: self.x + 1,
                y: self.y,
            },
            Self {
                x: self.x - 1,
                y: self.y + 1,
            },
            Self {
                x: self.x,
                y: self.y + 1,
            },
            Self {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }

    #[must_use]
    pub const fn left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    #[must_use]
    pub const fn right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    #[must_use]
    pub const fn below(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
        }
    }
}
