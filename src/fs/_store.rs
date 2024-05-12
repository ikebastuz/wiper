pub struct Store {}

pub trait DataStore {
    fn hello() -> bool;
    fn new() -> Self;
}

impl DataStore for Store {
    fn hello() -> bool {
        true
    }

    fn new() -> Store {
        Store {}
    }
}
