## TODO

#### Features
- [x] - Sort by size
- [x] - Async subfolder calculation
- [x] - Delete to Trash bin
- [ ] - PageUp/PageDown / g/G navigation
- [x] - Open file with system app
- [x] - Debug slow parent navigation
- [ ] - Check folder permissions
- [x] - Show loading folder indicator if it is not calculated completely
- [x] - File extension chart


#### Non-functional
- [x] - Lint with clippy
- [x] - Colored first letters (keybindings)
- [ ] - Better list scrolling (maybe auto-center cursor)
- [ ] - Refactor unit tests for easier state awaiting

#### Performance
- [ ] - Indexing / caching / refreshing
- [x] - Optimize folder sorting
- [ ] - Prevent from locking main thread, always process inputs
- [ ] - Review all variable clones, optimize

## Scripts
#### Run
```bash
cargo run
```
```bash
cargo run -- [PATH]
```
#### Lint
```bash
cargo clippy --all-targets -- -D warnings
```
#### Test
```bash
cargo test
```
#### Build
```bash
RUSTFLAGS="-Z threads=8" cargo +nightly build --release
```

## Install from homebrew
```bash
brew tap ikebastuz/wiper
brew install wiper
```
