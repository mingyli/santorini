#![feature(option_unwrap_none)]

pub mod input;
pub mod phase;
pub mod player;
pub mod state;
pub mod tower;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
