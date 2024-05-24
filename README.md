# Wiper - Disk cleanup tool

Wiper is a handy command-line tool made with Rust. It's perfect for anyone looking to quickly spot which folders are eating up all the disk space. Super easy to use, it gives you a clear visual breakdown of directory sizes, so you can clean things up without a hassle.

https://github.com/ikebastuz/wiper/assets/24222413/acf9384d-7f04-4f37-ac47-99b349e6ee29

## Features
- Fast and Efficient: Quickly scans directories and subdirectories to provide size metrics.  
- Cross-Platform: Works on Linux, Windows, and macOS.
- User-Friendly Output: Displays results in an easily understandable format.

## Usage
#### Run in current dir
`wiper`
#### Run in specific dir
`wiper [PATH]`

## Keybindings
- `jk/↓↑` - Navigate up/down
- `l/→/Enter` - Navigate into folder
- `h/←/Backspace` - Navigate to parent
- `d` - Delete file/folder. First hit - selects entry. Second hit - confirms deletion.
- `s` - Toggle sorting (`Title` / `Size`)
- `c` - Toggle coloring. When enabled - shows space usage with gradient
- `t` - Toggle trash. When enabled - removed content goes to Trash bin.
- `q` - Quit


## Installation

### MacOS
#### Homebrew
```
brew tap ikebastuz/wiper
brew install wiper
```

### Linux
#### AUR
```
paru -S wiper
```

## Build from source
```bash
git clone https://github.com/ikebastuz/wiper.git
cd wiper
cargo build --release
```

## Contributing
If you have any suggestions, improvements, or bug fixes, feel free to open an issue or submit a pull request.

## Why not [dua-cli](https://github.com/Byron/dua-cli)?
I started this project as part of my journey to learn Rust. I always missed having such a tool but had never heard of dua-cli. From my understanding, there are some differences:

#### Pros:
- Wiper allows navigating to the parent directory at any time
- Supports opening files with the default system app
- Simpler deletion flow

#### Cons:
- ~~It is 10-15 times slower because of manually implemented file traversal~~. Not anymore, rewritten with `jwalk`.
- Does not have filtering functionality
- Not capable of marking multiple entries for deletion

#### Subjective:
- The UI is more "elegant" :)
- Shows a "full" space-taken bar for the largest entry. So there will always be at least one entry with a full bar. If dua-cli has two large entries of similar size, they will be shown as approximately 50% bars.
