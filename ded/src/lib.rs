#![feature(arbitrary_self_types)]

#[macro_use]
mod utils;
pub mod html;
mod program;
mod render;

pub use self::{utils::log, program::Program};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
