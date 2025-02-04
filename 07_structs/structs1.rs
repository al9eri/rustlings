struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // Instantiate a regular struct.
        let green = ColorRegularStruct { red: 0, green: 255, blue: 0 };
        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // Instantiate a tuple struct.
        let color = ColorTupleStruct(255, 0, 0);
        assert_eq!(color.0, 255);
        assert_eq!(color.1, 0);
        assert_eq!(color.2, 0);
    }

    #[test]
    fn unit_structs() {
        // Instantiate a unit struct.
        let unit = UnitStruct;
        let message = format!("{:?}s are fun!", unit);
        assert_eq!(message, "UnitStructs are fun!");
    }
}