extern crate measurements;

use measurements::*;
use measurements::test_utils::assert_almost_eq;

#[test]
fn create() -> () {
    let aperture = Aperture::calc_nth_aperture_number(3);
    assert_eq!(aperture.get_base_units_name(), "f");
    assert_almost_eq(aperture.as_base_units(), 2.8284271247461903);
}
