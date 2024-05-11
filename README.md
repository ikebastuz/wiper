## TODO
- [x] - Sort by size
- [x] - Async subfolder calculation
- [ ] - Indexing / caching / refreshing
- [x] - Delete to Trash bin
- [ ] - PageUp/PageDown / g/G navigation
- [x] - Open file with system app
- [x] - Debug slow parent navigation
- [ ] - Optimize folder sorting
- [ ] - Check folder permissions
- [ ] - Prevent from locking main thread, always process inputs
- [ ] - Improve MPSC messaging (make single receiver)


#### Non-functional
- [ ] - Configure clippy
- [ ] - Colored first letters (keybindings)
- [ ] - Better list scrolling
- [ ] - Maybe auto-center cursor

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
