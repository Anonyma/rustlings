// structs1.rs
// Address all the TODOs to make the tests pass!
// Execute `rustlings hint structs1` or use the `hint` watch subcommand for a hint.

struct ColorClassicStruct {
    // ADDED
    name: String,
    hex: String,
}

// ADDED
struct ColorTupleStruct(String, String);

#[derive(Debug)]
struct UnitLikeStruct;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_c_structs() {
        // ADDED
        let green = ColorClassicStruct{name: "green".to_string(), hex: "#00FF00".to_string()};

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // ADDED
        let green = ColorTupleStruct(String::from("green"), String::from("#00FF00"));

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // ADDED
        let unit_struct = UnitStruct;
        let message = format!("{:?}s are fun!", unit_struct);

        assert_eq!(message, "UnitLikeStructs are fun!");
    }
}
