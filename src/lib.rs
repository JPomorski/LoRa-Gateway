// translated to Rust based on code by www.mischianti.org

pub mod lora;
pub mod status;
pub mod enums;
pub mod utility;
pub mod uart;
pub mod mock;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
