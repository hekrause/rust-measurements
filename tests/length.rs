
extern crate serde;
extern crate serde_json;
extern crate measurements;

use measurements::*;

#[test]
fn create() -> () {}

#[test]
fn json() {
    let len = Length::from_base_units(1.0);
    let serialized = serde_json::to_string(&len).unwrap();
    assert_eq!(serialized, "{\"meters\":1.0}");
    //let deserialized: Length = serde_json::from_str(&serialized).unwrap();
    //assert_eq!(deserialized, Length::from_base_units(1.0));
}
