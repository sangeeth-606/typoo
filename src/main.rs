// A Rust typing speed test application
use crossterm::{
    cursor::{MoveTo, Show, Hide},
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Print, Color, SetForegroundColor, ResetColor},
    terminal::{self, Clear, ClearType},
};
use rand::seq::SliceRandom;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::fs;
use std::path::Path;

mod words;

fn load_highest_wpm() -> u32 {
    let path = Path::new(&std::env::var("HOME").unwrap_or(".".to_string())).join(".typo_highest_wpm");
    fs::read_to_string(path)
        .map(|s| s.trim().parse().unwrap_or(0))
        .unwrap_or(0)
}

fn save_highest_wpm(wpm: u32) -> std::io::Result<()> {
    let path = Path::new(&std::env::var("HOME").unwrap_or(".".to_string())).join(".typo_highest_wpm");
    fs::write(path, wpm.to_string())
}

fn main() -> std::io::Result<()> {
    // Initialize terminal
    terminal::enable_raw_mode()?;
    execute!(stdout(), Hide, Clear(ClearType::All))?;

    // Load highest WPM
    let mut highest_wpm = load_highest_wpm();

    // Game state
    let mut words: Vec<&str> = words::WORDS.to_vec();
    words.shuffle(&mut rand::thread_rng());
    let mut current_word_idx = 0;
    let mut user_input = String::new();
    let mut correct_words = 0;
    let start_time = Instant::now();
    let duration = Duration::from_secs(30);

    // Main loop
    let result = loop {
        // Check if time is up
        let elapsed = start_time.elapsed();
        if elapsed >= duration {
            break Ok(());
        }
        let seconds_left = (duration - elapsed).as_secs();

        // Render UI
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

        // Display words with better formatting
        let current_word = words[current_word_idx];
        
        // Display current word with character-by-character highlighting
        for (i, c) in current_word.chars().enumerate() {
            if i < user_input.len() {
                if user_input.chars().nth(i) == Some(c) {
                    queue!(stdout(), SetForegroundColor(Color::Green), Print(c), ResetColor)?;
                } else {
                    queue!(stdout(), SetForegroundColor(Color::Red), Print(c), ResetColor)?;
                }
            } else {
                queue!(stdout(), SetForegroundColor(Color::White), Print(c), ResetColor)?;
            }
        }
        
        // Add space after current word
        queue!(stdout(), Print(" "))?;

        // Display next 3 words
        for i in (current_word_idx + 1)..(current_word_idx + 4).min(words.len()) {
            queue!(stdout(), SetForegroundColor(Color::DarkGrey), Print(words[i]), ResetColor)?;
            queue!(stdout(), Print(" "))?;
        }

        // Move to input line
        queue!(
            stdout(),
            MoveTo(0, 5),
            Print("Your input: "),
            Print(&user_input)
        )?;
        stdout().flush()?;

        // Handle input (non-blocking)
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char(' ') => {
                        if user_input.trim() == words[current_word_idx] {
                            correct_words += 1;
                        }
                        user_input.clear();
                        current_word_idx += 1;
                        if current_word_idx >= words.len() {
                            words.shuffle(&mut rand::thread_rng());
                            current_word_idx = 0;
                        }
                    },
                    KeyCode::Char(c) => user_input.push(c),
                    KeyCode::Backspace => { user_input.pop(); },
                    KeyCode::Enter => {
                        if user_input.trim() == words[current_word_idx] {
                            correct_words += 1;
                        }
                        user_input.clear();
                        current_word_idx += 1;
                        if current_word_idx >= words.len() {
                            words.shuffle(&mut rand::thread_rng());
                            current_word_idx = 0;
                        }
                    },
                    KeyCode::Esc => break Ok(()),
                    _ => {}
                }
            }
        }
    };

    // Calculate WPM
    let wpm = (correct_words as f32 / 0.5) as u32; 

    // Update highest WPM of the current user 
    if wpm > highest_wpm {
        highest_wpm = wpm;
        save_highest_wpm(wpm)?;
    }

    // Show final screen
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(0, 0),
        Print(format!("Test Complete!\nYour WPM: {}\nHighest WPM: {}", wpm, highest_wpm))
    )?;

    // Cleanup
    execute!(stdout(), Show)?;
    terminal::disable_raw_mode()?;
    result
}