pub struct Store {}

pub trait WiperStore {
    fn hello() -> bool;
    fn new() -> Self;
}

impl WiperStore for Store {
    fn hello() -> bool {
        true
    }

    fn new() -> Store {
        Store {}
    }
}
