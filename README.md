## TODO

#### Features
- [x] - Sort by size
- [x] - Async subfolder calculation
- [x] - Delete to Trash bin
- [ ] - PageUp/PageDown / g/G navigation
- [x] - Open file with system app
- [x] - Debug slow parent navigation
- [ ] - Check folder permissions

#### Non-functional
- [ ] - Configure clippy
- [ ] - Colored first letters (keybindings)
- [ ] - Better list scrolling
- [ ] - Maybe auto-center cursor
- [ ] - Refactor unit tests for easier state awaiting

#### Performance
- [ ] - Indexing / caching / refreshing
- [ ] - Optimize folder sorting
- [ ] - Prevent from locking main thread, always process inputs
- [ ] - Improve MPSC messaging (make single receiver)
- [ ] - Review all variable clones, optimize

## Scripts
#### Test
```bash
cargo test -- --test-threads=1 --nocapture
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
