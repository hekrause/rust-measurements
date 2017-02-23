use super::measurement::*;

/// The `Temperature` struct can be used to deal with temperatures in a common way.
///
/// # Example
///
/// ```
/// use measurements::Temperature;
///
/// let boiling_water = Temperature::from_celsius(100.0);
/// let fahrenheit = boiling_water.as_fahrenheit();
/// println!("Boiling water measures at {} degrees fahrenheit.", fahrenheit);
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Temperature {
    kelvin: f64
}

impl Temperature {
    pub fn from_kelvin(kelvin: f64) -> Self {
        Temperature { kelvin: kelvin }
    }

    pub fn from_celsius(celsius: f64) -> Self {
        Self::from_kelvin(celsius + 273.15)
    }

    pub fn from_fahrenheit(fahrenheit: f64) -> Self {
        Self::from_kelvin((fahrenheit - 32.0) / 1.8 + 273.15)
    }

    pub fn from_rankine(rankine: f64) -> Self {
        Self::from_kelvin((rankine - 491.67) / 1.8 + 273.15)
    }

    pub fn as_kelvin(&self) -> f64 {
        self.kelvin
    }

    pub fn as_celsius(&self) -> f64 {
        self.kelvin - 273.15
    }

    pub fn as_fahrenheit(&self) -> f64 {
        (self.kelvin - 273.15) * 1.8 + 32.0
    }

    pub fn as_rankine(&self) -> f64 {
        (self.kelvin - 273.15) * 1.8 + 491.67
    }
}

impl Measurement for Temperature {
    fn get_base_units(&self) -> f64 {
        self.kelvin
    }

    fn from_base_units(units: f64) -> Self {
        Self::from_kelvin(units)
    }
}

implement_measurement! { Temperature }

impl ::std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "{:.1} \u{00B0}C", self.as_celsius())
    }
}
