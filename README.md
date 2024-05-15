# Wiper - Disk cleanup tool

Wiper is a handy command-line tool made with Rust. It's perfect for anyone looking to quickly spot which folders are eating up all the disk space. Super easy to use, it gives you a clear visual breakdown of directory sizes, so you can clean things up without a hassle.



https://github.com/ikebastuz/wiper/assets/24222413/c61a6ad8-0a4b-4588-bdef-2aa39ae721f4



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

## Build from source
```bash
git clone https://github.com/ikebastuz/wiper.git
cd wiper
cargo build --release
```

## Contributing
If you have any suggestions, improvements, or bug fixes, feel free to open an issue or submit a pull request.
Current list of TODOs you can find [here](src/README.md)
