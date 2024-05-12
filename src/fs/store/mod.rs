mod ds_hashmap;
pub use ds_hashmap::DSHashmap;

pub trait DataStore {
    fn hello() -> bool;
    fn new() -> Self;
}
