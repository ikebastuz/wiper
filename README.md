## TODO

#### Features
- [x] - Sort by size
- [x] - Async subfolder calculation
- [x] - Delete to Trash bin
- [ ] - PageUp/PageDown / g/G navigation
- [x] - Open file with system app
- [x] - Debug slow parent navigation
- [ ] - Check folder permissions
- [ ] - Show loading folder indicator if it is not calculated completely
- [ ] - File extension chart


#### Non-functional
- [x] - Lint with clippy
- [x] - Colored first letters (keybindings)
- [ ] - Better list scrolling
- [ ] - Maybe auto-center cursor
- [ ] - Refactor unit tests for easier state awaiting

#### Performance
- [ ] - Indexing / caching / refreshing
- [x] - Optimize folder sorting
- [ ] - Prevent from locking main thread, always process inputs
- [ ] - Review all variable clones, optimize
- [ ] - Check Mutex locks for performance improvements
- [ ] - Consider having threadpool instead of single extra thread

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
cargo clippy
```
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
