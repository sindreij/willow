#![feature(arbitrary_self_types)]

#[macro_use]
pub mod utils;
pub mod attributes;
pub mod cmd;
pub mod events;
pub mod html;
mod program;
mod render;

pub use self::cmd::Cmd;
pub use self::{program::Program, utils::log};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
