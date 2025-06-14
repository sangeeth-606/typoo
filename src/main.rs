use crossterm::{
    cursor::{MoveTo, Show, Hide},
    event::{self, Event, KeyCode},
    execute, queue,
    style::Print,
    terminal::{self, Clear, ClearType},
};
use rand::seq::SliceRandom;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::{fs, path::Path};

use crate::words::WORDS;

mod words;

fn main() -> std::io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), Hide, Clear(ClearType::All))?;
    
    let mut highest_wpm = load_highest_wpm();

    //game state
    let mut words: Vec<&str> = WORDS.to_vec();
    words.shuffle(&mut rand::thread_rng());
    let mut current_word_idx =0;
    let mut user_input = String::new();
    let mut correct_words = 0;
    let start_time = Instant::now();
    let duration = Duration::from_secs(30);

    //main loop
    loop{
        //check if time is done?
        let elapsed = start_time.elapsed();
        if elapsed >=duration{
            break;
        }
        let seconds_left= (duration - elapsed).as_secs();

        //now render the ui
        queue!(
            stdout(),
            Clear(ClearType::All),
            MoveTo(0, 0),
            Print(format!("Timer: {}s", seconds_left)),
            MoveTo(20, 0),
            Print(format!("Highest WPM: {}", highest_wpm)),
            MoveTo(0, 2),
            Print("Type the following:"),
            MoveTo(0, 3),
        )?;
    }
}
