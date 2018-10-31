#![feature(arbitrary_self_types)]

#[macro_use]
mod utils;
pub mod attributes;
pub mod events;
pub mod html;
mod program;
mod render;

pub use self::{program::Program, utils::log};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
