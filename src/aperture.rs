
use super::measurement::*;

#[derive(Copy, Clone, Debug)]
pub struct Aperture {
    aperture_number: f64,
}

impl Aperture {
    pub fn from_aperture_number(aperture_number: f64) -> Self {
        Aperture { aperture_number: aperture_number }
    }

    pub fn calc_nth_aperture_number(nth: u64) -> Self {
        Aperture { aperture_number: (nth-1) as f64 * 2.0_f64.sqrt() }
    }
}

impl Measurement for Aperture {
    fn as_base_units(&self) -> f64 {
        self.aperture_number
    }

    fn from_base_units(aperture_number: f64) -> Self {
        Self::from_aperture_number(aperture_number)
    }

    fn get_base_units_name(&self) -> &'static str {
        "f"
    }
}

implement_measurement! { Aperture }