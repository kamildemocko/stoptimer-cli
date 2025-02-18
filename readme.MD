# Stopwatch CLI

A simple, terminal-based stopwatch written in Rust.

## Features

- Precise time tracking with millisecond accuracy
- Split time recording
- Pause/resume functionality
- Clean terminal interface

## Installation

Clone the repository and build using Cargo:

```bash
git clone https://github.com/kamildemocko/stoptimer-cli.git
cd stopwatch-cli
cargo build --release
```

## Usage

Run the program:

```bash
cargo run
```

## Look

```
> 2:    0 : 00 : 08 : 107
> 1:    0 : 00 : 02 : 155

0 : 00 : 26 : 538

>> running [ <h> to help ]
```

### Controls

- `SPACE` - Pause/Resume the stopwatch
- `s` - Record a split time
- `h` - Show help
- `q` - Quit the program

## Dependencies

- chrono = "0.4.39"
- crossterm = "0.28.1"

## License

MIT License