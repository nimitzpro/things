use std::num::Wrapping;

pub fn add(left: usize, right: usize) -> usize {
    (Wrapping(left) + Wrapping(right)).0 // + 23
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use std::usize::MAX;

    use super::*;

    #[test]
    fn it_works1() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        let result = add(MAX, 2);
        assert_eq!(result, usize::MIN+1);
    }

    #[test]
    fn it_works3() -> Result<(), String> {
        if 2 + 2 == add(2, 2) {
            Ok(())
        } else {
            Err(String::from("custom string can be return using Err instead of panic!"))
        }
    }

    const LARGER: Rectangle = Rectangle {
        width: 8,
        height: 7,
    };
    const SMALLER: Rectangle = Rectangle {
        width: 5,
        height: 1,
    };
    #[test]
    fn larger_can_hold_smaller() {
        assert!(LARGER.can_hold(&SMALLER));
    }
    
    #[test]
    fn smaller_cannot_hold_larger() {
        assert!(!SMALLER.can_hold(&LARGER));
    }
}
