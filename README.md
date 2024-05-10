## TODO
- [x] - Sort by size
- [x] - Async subfolder calculation
- [ ] - Better list scrolling
- [ ] - Maybe auto-center cursor
- [ ] - Indexing / caching / refreshing
- [x] - Delete to Trash bin
- [ ] - PageUp/PageDown / g/G navigation
- [x] - Open file with system app
- [ ] - Debug slow parent navigation


- [ ] - clippy

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
