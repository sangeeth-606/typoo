// A Rust typing speed test application
use crossterm::{
    cursor::{MoveTo, Show, Hide},
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Print, Color, SetForegroundColor, ResetColor,},
    terminal::{self, Clear, ClearType},
};
use rand::seq::SliceRandom;
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::fs;
use std::path::Path;
use self_update::update::Release;

mod words;

fn check_for_updates() -> Result<Option<Release>, Box<dyn std::error::Error>> {
    let updater = self_update::backends::github::Update::configure()
        .repo_owner("sangeeth-606")
        .repo_name("typoo")
        .bin_name("typoo")
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?;
    
    let latest_release = updater.get_latest_release()?;
    let current_version = env!("CARGO_PKG_VERSION");
    
    if latest_release.version != current_version {
        Ok(Some(latest_release))
    } else {
        Ok(None)
    }
}

fn show_update_notification(latest_version: &str) -> std::io::Result<()> {
    let (width, height) = terminal::size()?;
    let center_x = width / 2;
    let center_y = height / 2;

    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(center_x - 20, center_y - 2),
        SetForegroundColor(Color::Yellow),
        Print("╔════════════════════════════════════════════════════════════╗"),
        MoveTo(center_x - 20, center_y - 1),
        Print("║                  New Version Available!                    ║"),
        MoveTo(center_x - 20, center_y),
        Print(format!("║ A new version ({}) is available!                    ║", latest_version)),
        MoveTo(center_x - 20, center_y + 1),
        Print("║ Would you like to update now? (Y/N)                       ║"),
        MoveTo(center_x - 20, center_y + 2),
        Print("╚════════════════════════════════════════════════════════════╝"),
        ResetColor
    )?;
    stdout().flush()?;
    Ok(())
}

fn load_highest_wpm() -> u32 {
    let path = Path::new(&std::env::var("HOME").unwrap_or(".".to_string())).join(".typo_highest_wpm");
    fs::read_to_string(path)
        .map(|s| s.trim().parse().unwrap_or(0))
        .unwrap_or(0)
}
//which mean's it return's nothing on succes but can return an error
fn save_highest_wpm(wpm: u32) -> std::io::Result<()> {
    let path = Path::new(&std::env::var("HOME").unwrap_or(".".to_string())).join(".typo_highest_wpm");
    fs::write(path, wpm.to_string())
}

fn main() -> std::io::Result<()> {
    // Check for updates
    if let Ok(Some(release)) = check_for_updates() {
        show_update_notification(&release.version)?;
        
      
        loop {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('y') | KeyCode::Char('Y') => {
                            // Update the application
                            if let Err(e) = self_update::backends::github::Update::configure()
                                .repo_owner("sangeeth-606")
                                .repo_name("typoo")
                                .bin_name("typoo")
                                .show_download_progress(true)
                                .current_version(env!("CARGO_PKG_VERSION"))
                                .build()
                                .and_then(|updater| updater.update()) {
                                eprintln!("Failed to update: {}", e);
                            }
                            return Ok(());
                        },
                        KeyCode::Char('n') | KeyCode::Char('N') => break,
                        _ => {}
                    }
                }
            }
        }
    }

    // Initialize terminal
    terminal::enable_raw_mode()?;
    execute!(stdout(), Hide, Clear(ClearType::All))?;

    // Load highest WPM
    let mut highest_wpm = load_highest_wpm();

    // Show welcome screen and wait for Enter
    let (width, height) = terminal::size()?;
    let center_x = width / 2;
    let center_y = height / 2;

    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveTo(center_x - 10, center_y - 4),
        SetForegroundColor(Color::Cyan),
        Print("╔════════════════════════════════════════╗"),
        MoveTo(center_x - 10, center_y - 3),
        Print("║            Welcome to Typoo!           ║"),
        MoveTo(center_x - 10, center_y - 2),
        Print("╚════════════════════════════════════════╝"),
        MoveTo(center_x - 10, center_y),
        SetForegroundColor(Color::Yellow),
        Print(format!("Highest WPM: {}", highest_wpm)),
        MoveTo(center_x - 10, center_y + 2),
        SetForegroundColor(Color::Green),
        Print("Press Enter to Start"),
        MoveTo(center_x - 10, center_y + 3),
        SetForegroundColor(Color::Red),
        Print("Press Esc to Exit"),
        ResetColor
    )?;
    stdout().flush()?;

    // Wait for Enter key or Esc key
    loop {
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => break,
                    KeyCode::Esc => return Ok(()),
                    _ => {}
                }
            }
        }
    }

    // Main game loop
    loop {
        // Show cursor during the test
        execute!(stdout(), Show)?;

        // Game state/logic
        let mut words: Vec<&str> = words::WORDS.to_vec();
        words.shuffle(&mut rand::thread_rng());
        let mut current_word_idx = 0;
        let mut user_input = String::new();
        let mut correct_words = 0;
        let start_time = Instant::now();
        let duration = Duration::from_secs(30);

        // Main loop
        let _result: std::io::Result<()> = loop {
            // Check if time is up
            let elapsed = start_time.elapsed();
            if elapsed >= duration {
                break Ok(());
            }
            let seconds_left = (duration - elapsed).as_secs();

            // Render UI
            let (width, _) = terminal::size()?;
            let center_x = width / 2;

            queue!(
                stdout(),
                Clear(ClearType::All),
                MoveTo(0, 0),
                SetForegroundColor(Color::Yellow),
                Print("╔════════════════════════════════════════════════════════════════════════════════╗"),
                MoveTo(0, 1),
                Print(format!("║ Timer: {}s", seconds_left)),
                MoveTo(width - 25, 1),
                Print(format!("Highest WPM: {} ║", highest_wpm)),
                MoveTo(0, 2),
                Print("╚════════════════════════════════════════════════════════════════════════════════╝"),
                MoveTo(center_x - 20, 4),
                SetForegroundColor(Color::Cyan),
                Print("Type the following:"),
                MoveTo(center_x - 20, 5),
                ResetColor
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

            // Move cursor to the end of the typed input on the words line
            queue!(stdout(), MoveTo(center_x - 20 + user_input.chars().count() as u16, 5))?;
            stdout().flush()?;

            // Handle input (non-terminal freezing1)
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

        // Hide cursor after the test
        execute!(stdout(), Hide)?;

        // Calculate WPM
        let wpm = (correct_words as f32 / 0.5) as u32; 

        // Update highest WPM of the current user 
        if wpm > highest_wpm {
            highest_wpm = wpm;
            save_highest_wpm(wpm)?;
        }

        // Show final screen
        let (width, height) = terminal::size()?;
        let center_x = width / 2;
        let center_y = height / 2;

        execute!(
            stdout(),
            Clear(ClearType::All),
            MoveTo(center_x - 15, center_y - 2),
            SetForegroundColor(Color::Cyan),
            Print("╔════════════════════════════════════════════════════╗"),
            MoveTo(center_x - 15, center_y - 1),
            Print("║                Test Complete!                      ║"),
            MoveTo(center_x - 15, center_y),
            Print(format!("║ Your WPM: {:<30} ║", wpm)),
            MoveTo(center_x - 15, center_y + 1),
            Print(format!("║ Highest WPM: {:<27} ║", highest_wpm)),
            MoveTo(center_x - 15, center_y + 2),
            Print("╚════════════════════════════════════════════════════╝"),
            MoveTo(center_x - 15, center_y + 4),
            SetForegroundColor(Color::Green),
            Print("Press Enter to Start New Test"),
            MoveTo(center_x - 15, center_y + 5),
            SetForegroundColor(Color::Red),
            Print("Press Esc to Exit"),
            ResetColor
        )?;
        stdout().flush()?;

        // Wait for Enter key or Esc key
        loop {
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Enter => break,
                        KeyCode::Esc => {
                            // Cleanup
                            execute!(stdout(), Show)?;
                            terminal::disable_raw_mode()?;
                            return Ok(());
                        },
                        _ => {}
                    }
                }
            }
        }
    }
}