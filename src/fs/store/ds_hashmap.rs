use crate::fs::DataStore;

pub struct DSHashmap {}

impl DataStore for DSHashmap {
    fn hello() -> bool {
        true
    }

    fn new() -> DSHashmap {
        DSHashmap {}
    }
}
