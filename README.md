# 🚀 Typoo

<div align="center">

![Typoo Banner](https://img.shields.io/badge/Typoo-Terminal%20Typing%20Test-blue)
![License](https://img.shields.io/badge/license-Apache%202.0-green)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange)

A blazing fast, terminal-based typing speed test application written in Rust.

</div>

## 📸 Examples

### Welcome Screen
```
╔════════════════════════════════════════╗
║            Welcome to Typoo!           ║
╚════════════════════════════════════════╝
Highest WPM: 85
Press Enter to Start
Press Esc to Exit
```
*Note: "Welcome to Typoo!" appears in cyan, "Highest WPM" in yellow, "Press Enter" in green, and "Press Esc" in red*

### Typing Test Screen
```
╔════════════════════════════════════════════════════════════════════════════════╗
║ Timer: 25s                                                      Highest WPM: 85 ║
╚════════════════════════════════════════════════════════════════════════════════╝

Type the following:
apple banana cherry date elder

Your input: appl
```
*Note: Timer and Highest WPM are in yellow, "Type the following" is in cyan, correctly typed characters are in green, incorrect characters are in red, and upcoming words are in dark grey*

### Results Screen
```
╔════════════════════════════════════════════════════╗
║                Test Complete!                      ║
║ Your WPM: 75                                      ║
║ Highest WPM: 85                                   ║
╚════════════════════════════════════════════════════╝
Press Enter to Start New Test
Press Esc to Exit
```
*Note: "Test Complete!" is in cyan, WPM scores are in yellow, "Press Enter" is in green, and "Press Esc" is in red*

### Update Notification
```
╔════════════════════════════════════════════════════════════╗
║                  New Version Available!                    ║
║ A new version (1.1.0) is available!                        ║
║ Would you like to update now? (Y/N)                        ║
╚════════════════════════════════════════════════════════════╝
```
*Note: The entire update notification box appears in yellow*

## ✨ Features

- 🎯 Real-time typing accuracy feedback
- ⚡ Character-by-character highlighting
- 🏆 Persistent high score tracking
- 🔄 Auto-update functionality
- 🎨 Beautiful terminal UI with color-coded feedback
- ⏱️ 30-second timed tests
- 🎮 Simple and intuitive controls

## 📋 Prerequisites

- Rust 1.70 or higher
- A terminal emulator that supports ANSI escape codes

## 🛠️ Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/sangeeth-606/typoo.git

# Navigate to the project directory
cd typoo

# Build the project
cargo build --release

# Install the binary
cargo install --path .
```

### Using Cargo

```bash
cargo install typoo
```

## 🎮 Usage

1. Launch Typoo:
```bash
typoo
```

2. Controls:
- Press `Enter` to start a new test
- Type the words as they appear
- Press `Space` or `Enter` to move to the next word
- Press `Esc` to exit

## 🎯 How It Works

Typoo presents you with a series of words to type within a 30-second timeframe. Your typing speed is measured in Words Per Minute (WPM), and your accuracy is tracked in real-time with color-coded feedback:

- 🟢 Green: Correct characters
- 🔴 Red: Incorrect characters
- ⚪ White: Characters yet to be typed

Your highest WPM score is automatically saved and displayed during each session.

## 🔄 Auto-Update

Typoo includes an auto-update feature that checks for new versions when you launch the application. If a new version is available, you'll be prompted to update.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [crossterm](https://github.com/crossterm-rs/crossterm) for terminal manipulation
- Inspired by [monkeytype](https://monkeytype.com/)

## 📧 Contact

Project Link: [https://github.com/sangeeth-606/typoo](https://github.com/sangeeth-606/typoo)

---

<div align="center">
Made with ❤️ by zape
</div> 