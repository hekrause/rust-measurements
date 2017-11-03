extern crate measurements;

use measurements::*;

#[test]
fn create() -> () {
    let meter1 = Length::from_metres(2.0);
    let meter2 = Length::from_metres(5.0);

    let meter = meter1 * meter2;
}