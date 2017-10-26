extern crate measurements;

use measurements::*;

#[test]
fn create() -> () {
    let meter = Length::from_metres(1.0);
    let millimeter = meter.as_millimeters();
    assert_eq!(1000.0, millimeter);
}