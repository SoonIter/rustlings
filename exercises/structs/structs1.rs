// structs1.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a hint.

use std::fmt::Display;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // Instantiate a classic c struct!
        struct Color {
            red: i32,
            green: i32,
            blue: i32,
        }
        impl Color {
            fn new(red: i32, green: i32, blue: i32) -> Color {
                Color { red, green, blue }
            }
        }
        let green = Color::new(0, 255, 0);
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // Instantiate a tuple struct!
        struct Color(i32, i32, i32);
        let green = (0, 255, 0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        #[derive(Debug)]
        struct UnitLikeStruct;

        impl Display for UnitLikeStruct {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "UnitLikeStruct")
            }
        }

        // Instantiate a unit-like struct!
        let unit_like_struct = UnitLikeStruct;
        let message = format!("{:?}s are fun!", unit_like_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
