
extern crate serde;
extern crate serde_json;
extern crate measurements;

use measurements::*;
use measurements::test_utils::assert_almost_eq;

#[test]
fn create() -> () {
    let aperture = Aperture::calc_nth_aperture_number(3);
    assert_eq!(aperture.get_base_units_name(), "f");
    assert_almost_eq(aperture.as_base_units(), 2.8284271247461903);
}

#[test]
fn json() {
    let len = Aperture::from_base_units(2.828);
    let serialized = serde_json::to_string(&len).unwrap();
    assert_eq!(serialized, "{\"aperture_number\":2.828}");
    let deserialized: Aperture = serde_json::from_str(&serialized).unwrap();
    assert_eq!(deserialized, Aperture::from_base_units(2.828))
}
