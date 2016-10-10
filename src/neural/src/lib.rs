extern crate dependencies;
// extern crate rustc_serialize;
#[macro_use]
extern crate log;

pub use self::dependencies::rustc_serialize;

pub mod evolution;
pub mod network;
