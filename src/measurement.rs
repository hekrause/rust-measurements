
pub trait Measurement {
    fn get_appropriate_units(&self) -> (&'static str, f64) {
        (self.get_base_units_name(), self.as_base_units())
    }

    fn pick_appropriate_units(&self, list: &[(&'static str, f64)]) -> (&'static str, f64) {
        for &(unit, ref scale) in list.iter().rev() {
            let value = self.as_base_units() / scale;
            if value.abs() > 1.0 {
                return (unit, value);
            }
        }
        (list[0].0, self.as_base_units() / list[0].1)
    }

    fn get_base_units_name(&self) -> &'static str;
    fn as_base_units(&self) -> f64;
    fn from_base_units(units: f64) -> Self;
}

#[macro_export]
macro_rules! implement_display {
    ($($t:ty)*) => ($(

        impl ::std::fmt::Display for $t {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let (unit, value) = self.get_appropriate_units();
                value.fmt(f)?;      // Value
                write!(f, "\u{00A0}{}", unit)
            }
        }
    )*)
}

#[macro_export]
macro_rules! implement_measurement {
    ($($t:ty)*) => ($(

        implement_display!( $t );

        impl ::std::ops::Add for $t {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self::from_base_units(self.as_base_units() + rhs.as_base_units())
            }
        }

        impl ::std::ops::Sub for $t {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self::from_base_units(self.as_base_units() - rhs.as_base_units())
            }
        }

        impl ::std::ops::Div<$t> for $t {
            type Output = f64;

            fn div(self, rhs: Self) -> f64 {
                self.as_base_units() / rhs.as_base_units()
            }
        }

        impl ::std::ops::Div<f64> for $t {
            type Output = Self;

            fn div(self, rhs: f64) -> Self {
                Self::from_base_units(self.as_base_units() / rhs)
            }
        }

        impl ::std::ops::Mul<f64> for $t {
            type Output = Self;

            fn mul(self, rhs: f64) -> Self {
                Self::from_base_units(self.as_base_units() * rhs)
            }
        }

        impl ::std::ops::Mul<$t> for f64 {
            type Output = $t;

            fn mul(self, rhs: $t) -> $t {
                rhs * self
            }
        }

        impl ::std::cmp::Eq for $t { }
        impl ::std::cmp::PartialEq for $t {
            fn eq(&self, other: &Self) -> bool {
                self.as_base_units() == other.as_base_units()
            }
        }

        impl ::std::cmp::PartialOrd for $t {
            fn partial_cmp(&self, other: &Self) -> Option<::std::cmp::Ordering> {
                self.as_base_units().partial_cmp(&other.as_base_units())
            }
        }
    )*)
}
