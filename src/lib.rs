mod tests;

use std::ops::{Add,Sub,Div,Mul};

#[derive(Copy, Clone, Debug)]
pub struct Length {
    meters: f64
}

// Constants, metric
const METER_NANOMETER_FACTOR: f64 = 1000000000.0;
const METER_MICROMETER_FACTOR: f64 = 1000000.0;
const METER_MILLIMETER_FACTOR: f64 = 1000.0;
const METER_CENTIMETER_FACTOR: f64 = 100.0;
const METER_DECAMETER_FACTOR: f64 = 0.1;
const METER_HECTOMETER_FACTOR: f64 = 0.01;
const METER_KILOMETER_FACTOR: f64 = 0.001;

// Constants, imperial
const METER_INCH_FACTOR: f64 = 39.3700787402;
const METER_FEET_FACTOR: f64 = 3.28083989501;
const METER_YARD_FACTOR: f64 = 1.09361329834;
const METER_FURLONG_FACTOR: f64 = 0.0049709695379;
const METER_MILE_FACTOR: f64 = 0.000621371192237;

impl Length {
    // Inputs, metric
    pub fn from_meters(meters: f64) -> Length {
        Length { meters: meters }
    }
    
    fn from_nanometers(nanometers: f64) -> Length {
        Self::from_meters(nanometers / METER_NANOMETER_FACTOR)
    }
    
    fn from_micrometers(micrometers: f64) -> Length {
        Self::from_meters(micrometers / METER_MICROMETER_FACTOR)
    }
    
    fn from_millimeters(millimeters: f64) -> Length {
        Self::from_meters(millimeters / METER_MILLIMETER_FACTOR)
    }
    
    fn from_centimeters(centimeters: f64) -> Length {
        Self::from_meters(centimeters / METER_CENTIMETER_FACTOR)
    }
    
    fn from_decameters(decameters: f64) -> Length {
        Self::from_meters(decameters / METER_DECAMETER_FACTOR)
    }
    
    fn from_hectometers(hectometers: f64) -> Length {
        Self::from_meters(hectometers / METER_HECTOMETER_FACTOR)
    }
    
    fn from_kilometers(kilometers: f64) -> Length {
        Self::from_meters(kilometers / METER_KILOMETER_FACTOR)
    }
    
    // Inputs, imperial
    fn from_inches(inches: f64) -> Length {
        Self::from_meters(inches / METER_INCH_FACTOR)
    }
    
    fn from_feet(feet: f64) -> Length {
        Self::from_meters(feet / METER_FEET_FACTOR)
    }
    
    fn from_yards(yards: f64) -> Length {
        Self::from_meters(yards / METER_YARD_FACTOR)
    }
    
    fn from_furlongs(furlongs: f64) -> Length {
        Self::from_meters(furlongs / METER_FURLONG_FACTOR)
    }
    
    fn from_miles(miles: f64) -> Length {
        Self::from_meters(miles / METER_MILE_FACTOR)
    }
    
    // Outputs, metric
    fn as_nanometers(&self) -> f64 {
        self.meters * METER_NANOMETER_FACTOR
    }
    
    fn as_micrometers(&self) -> f64 {
        self.meters * METER_MICROMETER_FACTOR
    }
    
    fn as_millimeters(&self) -> f64 {
        self.meters * METER_MILLIMETER_FACTOR
    }
    
    fn as_centimeters(&self) -> f64 {
        self.meters * METER_CENTIMETER_FACTOR
    }
    
    fn as_meters(&self) -> f64 {
        self.meters
    }
    
    fn as_decameters(&self) -> f64 {
        self.meters * METER_DECAMETER_FACTOR
    }
    
    fn as_hectometer(&self) -> f64 {
        self.meters * METER_HECTOMETER_FACTOR
    }
    
    fn as_kilometers(&self) -> f64 {
        self.meters * METER_KILOMETER_FACTOR
    }
    
    // Outputs, imperial
    fn as_inches(&self) -> f64 {
        self.meters * METER_INCH_FACTOR
    }
    
    fn as_feet(&self) -> f64 {
        self.meters * METER_FEET_FACTOR
    }
    
    fn as_yards(&self) -> f64 {
        self.meters * METER_YARD_FACTOR
    }
    
    fn as_furlongs(&self) -> f64 {
        self.meters * METER_FURLONG_FACTOR
    }
    
    fn as_miles(&self) -> f64 {
        self.meters * METER_MILE_FACTOR
    }
}

impl Add for Length {
    type Output = Length;
    
    fn add(self, rhs: Length) -> Length {
        Length::from_meters(self.meters + rhs.meters)
    }
}

impl Sub for Length {
    type Output = Length;
    
    fn sub(self, rhs: Length) -> Length {
        Length::from_meters(self.meters - rhs.meters)
    }
}

impl Div for Length {
    type Output = Length;
    
    fn div(self, rhs: Length) -> Length {
        Length::from_meters(self.meters / rhs.meters)
    }
}

impl Mul for Length {
    type Output = Length;
    
    fn mul(self, rhs: Length) -> Length {
        Length::from_meters(self.meters * rhs.meters)
    }
}