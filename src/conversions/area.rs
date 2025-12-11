//! Area conversion functions organized by unit type
//! All conversions use square metres as the base unit for accuracy and consistency

/// Square metres conversion functions
pub mod square_metres {
    /// Converts square metres to square millimetres.
    /// # Arguments
    /// * `value` - The area in square metres to convert
    pub fn to_square_millimetres(value: f64) -> f64 {
        value * 1_000_000.0
    }

    /// Converts square metres to square centimetres.
    /// # Arguments
    /// * `value` - The area in square metres to convert
    pub fn to_square_centimetres(value: f64) -> f64 {
        value * 10_000.0
    }

    /// Converts square metres to square kilometres.
    /// # Arguments
    /// * `value` - The area in square metres to convert
    pub fn to_square_kilometres(value: f64) -> f64 {
        value / 1_000_000.0
    }

    /// Converts square metres to square inches.
    /// # Arguments
    /// * `value` - The area in square metres to convert
    pub fn to_square_inches(value: f64) -> f64 {
        value * 1550.0031
    }

    /// Converts square metres to square feet.
    /// # Arguments
    /// * `value` - The area in square metres to convert
    pub fn to_square_feet(value: f64) -> f64 {
        value * 10.763910417
    }

    /// Converts square metres to square yards.
    /// # Arguments
    /// * `value` - The area in square metres to convert
    pub fn to_square_yards(value: f64) -> f64 {
        value * 1.1959900463
    }

    /// Converts square metres to acres.
    /// # Arguments
    /// * `value` - The area in square metres to convert
    pub fn to_acres(value: f64) -> f64 {
        value / 4046.8564224
    }

    /// Converts square metres to hectares.
    /// # Arguments
    /// * `value` - The area in square metres to convert
    pub fn to_hectares(value: f64) -> f64 {
        value / 10_000.0
    }

    /// Converts square metres to square miles.
    /// # Arguments
    /// * `value` - The area in square metres to convert
    pub fn to_square_miles(value: f64) -> f64 {
        value / 2_589_988.110336
    }
}

/// Square millimetres conversion functions
pub mod square_millimetres {
    /// Converts square millimetres to square metres.
    /// # Arguments
    /// * `value` - The area in square millimetres to convert
    pub fn to_square_metres(value: f64) -> f64 {
        value / 1_000_000.0
    }

    /// Converts square millimetres to square centimetres.
    /// # Arguments
    /// * `value` - The area in square millimetres to convert
    pub fn to_square_centimetres(value: f64) -> f64 {
        value / 100.0
    }

    /// Converts square millimetres to square kilometres.
    /// # Arguments
    /// * `value` - The area in square millimetres to convert
    pub fn to_square_kilometres(value: f64) -> f64 {
        super::square_metres::to_square_kilometres(to_square_metres(value))
    }

    /// Converts square millimetres to square inches.
    /// # Arguments
    /// * `value` - The area in square millimetres to convert
    pub fn to_square_inches(value: f64) -> f64 {
        super::square_metres::to_square_inches(to_square_metres(value))
    }

    /// Converts square millimetres to square feet.
    /// # Arguments
    /// * `value` - The area in square millimetres to convert
    pub fn to_square_feet(value: f64) -> f64 {
        super::square_metres::to_square_feet(to_square_metres(value))
    }

    /// Converts square millimetres to square yards.
    /// # Arguments
    /// * `value` - The area in square millimetres to convert
    pub fn to_square_yards(value: f64) -> f64 {
        super::square_metres::to_square_yards(to_square_metres(value))
    }

    /// Converts square millimetres to acres.
    /// # Arguments
    /// * `value` - The area in square millimetres to convert
    pub fn to_acres(value: f64) -> f64 {
        super::square_metres::to_acres(to_square_metres(value))
    }

    /// Converts square millimetres to hectares.
    /// # Arguments
    /// * `value` - The area in square millimetres to convert
    pub fn to_hectares(value: f64) -> f64 {
        super::square_metres::to_hectares(to_square_metres(value))
    }

    /// Converts square millimetres to square miles.
    /// # Arguments
    /// * `value` - The area in square millimetres to convert
    pub fn to_square_miles(value: f64) -> f64 {
        super::square_metres::to_square_miles(to_square_metres(value))
    }
}

/// Square centimetres conversion functions
pub mod square_centimetres {
    /// Converts square centimetres to square metres.
    /// # Arguments
    /// * `value` - The area in square centimetres to convert
    pub fn to_square_metres(value: f64) -> f64 {
        value / 10_000.0
    }

    /// Converts square centimetres to square millimetres.
    /// # Arguments
    /// * `value` - The area in square centimetres to convert
    pub fn to_square_millimetres(value: f64) -> f64 {
        value * 100.0
    }

    /// Converts square centimetres to square kilometres.
    /// # Arguments
    /// * `value` - The area in square centimetres to convert
    pub fn to_square_kilometres(value: f64) -> f64 {
        super::square_metres::to_square_kilometres(to_square_metres(value))
    }

    /// Converts square centimetres to square inches.
    /// # Arguments
    /// * `value` - The area in square centimetres to convert
    pub fn to_square_inches(value: f64) -> f64 {
        value * 0.15500031
    }

    /// Converts square centimetres to square feet.
    /// # Arguments
    /// * `value` - The area in square centimetres to convert
    pub fn to_square_feet(value: f64) -> f64 {
        super::square_metres::to_square_feet(to_square_metres(value))
    }

    /// Converts square centimetres to square yards.
    /// # Arguments
    /// * `value` - The area in square centimetres to convert
    pub fn to_square_yards(value: f64) -> f64 {
        super::square_metres::to_square_yards(to_square_metres(value))
    }

    /// Converts square centimetres to acres.
    /// # Arguments
    /// * `value` - The area in square centimetres to convert
    pub fn to_acres(value: f64) -> f64 {
        super::square_metres::to_acres(to_square_metres(value))
    }

    /// Converts square centimetres to hectares.
    /// # Arguments
    /// * `value` - The area in square centimetres to convert
    pub fn to_hectares(value: f64) -> f64 {
        super::square_metres::to_hectares(to_square_metres(value))
    }

    /// Converts square centimetres to square miles.
    /// # Arguments
    /// * `value` - The area in square centimetres to convert
    pub fn to_square_miles(value: f64) -> f64 {
        super::square_metres::to_square_miles(to_square_metres(value))
    }
}

/// Square kilometres conversion functions
pub mod square_kilometres {
    /// Converts square kilometres to square metres.
    /// # Arguments
    /// * `value` - The area in square kilometres to convert
    pub fn to_square_metres(value: f64) -> f64 {
        value * 1_000_000.0
    }

    /// Converts square kilometres to square millimetres.
    /// # Arguments
    /// * `value` - The area in square kilometres to convert
    pub fn to_square_millimetres(value: f64) -> f64 {
        super::square_metres::to_square_millimetres(to_square_metres(value))
    }

    /// Converts square kilometres to square centimetres.
    /// # Arguments
    /// * `value` - The area in square kilometres to convert
    pub fn to_square_centimetres(value: f64) -> f64 {
        super::square_metres::to_square_centimetres(to_square_metres(value))
    }

    /// Converts square kilometres to square inches.
    /// # Arguments
    /// * `value` - The area in square kilometres to convert
    pub fn to_square_inches(value: f64) -> f64 {
        super::square_metres::to_square_inches(to_square_metres(value))
    }

    /// Converts square kilometres to square feet.
    /// # Arguments
    /// * `value` - The area in square kilometres to convert
    pub fn to_square_feet(value: f64) -> f64 {
        super::square_metres::to_square_feet(to_square_metres(value))
    }

    /// Converts square kilometres to square yards.
    /// # Arguments
    /// * `value` - The area in square kilometres to convert
    pub fn to_square_yards(value: f64) -> f64 {
        super::square_metres::to_square_yards(to_square_metres(value))
    }

    /// Converts square kilometres to acres.
    /// # Arguments
    /// * `value` - The area in square kilometres to convert
    pub fn to_acres(value: f64) -> f64 {
        value * 247.10538147
    }

    /// Converts square kilometres to hectares.
    /// # Arguments
    /// * `value` - The area in square kilometres to convert
    pub fn to_hectares(value: f64) -> f64 {
        value * 100.0
    }

    /// Converts square kilometres to square miles.
    /// # Arguments
    /// * `value` - The area in square kilometres to convert
    pub fn to_square_miles(value: f64) -> f64 {
        value * 0.3861021585
    }
}

/// Square inches conversion functions
pub mod square_inches {
    /// Converts square inches to square metres.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_square_metres(value: f64) -> f64 {
        value / 1550.0031
    }

    /// Converts square inches to square millimetres.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_square_millimetres(value: f64) -> f64 {
        super::square_metres::to_square_millimetres(to_square_metres(value))
    }

    /// Converts square inches to square centimetres.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_square_centimetres(value: f64) -> f64 {
        value / 0.15500031
    }

    /// Converts square inches to square kilometres.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_square_kilometres(value: f64) -> f64 {
        super::square_metres::to_square_kilometres(to_square_metres(value))
    }

    /// Converts square inches to square feet.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_square_feet(value: f64) -> f64 {
        value / 144.0
    }

    /// Converts square inches to square yards.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_square_yards(value: f64) -> f64 {
        value / 1296.0
    }

    /// Converts square inches to acres.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_acres(value: f64) -> f64 {
        super::square_metres::to_acres(to_square_metres(value))
    }

    /// Converts square inches to hectares.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_hectares(value: f64) -> f64 {
        super::square_metres::to_hectares(to_square_metres(value))
    }

    /// Converts square inches to square miles.
    /// # Arguments
    /// * `value` - The area in square inches to convert
    pub fn to_square_miles(value: f64) -> f64 {
        super::square_metres::to_square_miles(to_square_metres(value))
    }
}

/// Square feet conversion functions
pub mod square_feet {
    /// Converts square feet to square metres.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_square_metres(value: f64) -> f64 {
        value / 10.763910417
    }

    /// Converts square feet to square millimetres.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_square_millimetres(value: f64) -> f64 {
        super::square_metres::to_square_millimetres(to_square_metres(value))
    }

    /// Converts square feet to square centimetres.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_square_centimetres(value: f64) -> f64 {
        super::square_metres::to_square_centimetres(to_square_metres(value))
    }

    /// Converts square feet to square kilometres.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_square_kilometres(value: f64) -> f64 {
        super::square_metres::to_square_kilometres(to_square_metres(value))
    }

    /// Converts square feet to square inches.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_square_inches(value: f64) -> f64 {
        value * 144.0
    }

    /// Converts square feet to square yards.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_square_yards(value: f64) -> f64 {
        value / 9.0
    }

    /// Converts square feet to acres.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_acres(value: f64) -> f64 {
        value / 43560.0
    }

    /// Converts square feet to hectares.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_hectares(value: f64) -> f64 {
        super::square_metres::to_hectares(to_square_metres(value))
    }

    /// Converts square feet to square miles.
    /// # Arguments
    /// * `value` - The area in square feet to convert
    pub fn to_square_miles(value: f64) -> f64 {
        value / 27878400.0
    }
}

/// Square yards conversion functions
pub mod square_yards {
    /// Converts square yards to square metres.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_square_metres(value: f64) -> f64 {
        value / 1.1959900463
    }

    /// Converts square yards to square millimetres.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_square_millimetres(value: f64) -> f64 {
        super::square_metres::to_square_millimetres(to_square_metres(value))
    }

    /// Converts square yards to square centimetres.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_square_centimetres(value: f64) -> f64 {
        super::square_metres::to_square_centimetres(to_square_metres(value))
    }

    /// Converts square yards to square kilometres.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_square_kilometres(value: f64) -> f64 {
        super::square_metres::to_square_kilometres(to_square_metres(value))
    }

    /// Converts square yards to square inches.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_square_inches(value: f64) -> f64 {
        value * 1296.0
    }

    /// Converts square yards to square feet.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_square_feet(value: f64) -> f64 {
        value * 9.0
    }

    /// Converts square yards to acres.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_acres(value: f64) -> f64 {
        value / 4840.0
    }

    /// Converts square yards to hectares.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_hectares(value: f64) -> f64 {
        super::square_metres::to_hectares(to_square_metres(value))
    }

    /// Converts square yards to square miles.
    /// # Arguments
    /// * `value` - The area in square yards to convert
    pub fn to_square_miles(value: f64) -> f64 {
        value / 3097600.0
    }
}

/// Acres conversion functions
pub mod acres {
    /// Converts acres to square metres.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_metres(value: f64) -> f64 {
        value * 4046.8564224
    }

    /// Converts acres to square millimetres.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_millimetres(value: f64) -> f64 {
        super::square_metres::to_square_millimetres(to_square_metres(value))
    }

    /// Converts acres to square centimetres.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_centimetres(value: f64) -> f64 {
        super::square_metres::to_square_centimetres(to_square_metres(value))
    }

    /// Converts acres to square kilometres.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_kilometres(value: f64) -> f64 {
        value / 247.10538147
    }

    /// Converts acres to square inches.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_inches(value: f64) -> f64 {
        super::square_metres::to_square_inches(to_square_metres(value))
    }

    /// Converts acres to square feet.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_feet(value: f64) -> f64 {
        value * 43560.0
    }

    /// Converts acres to square yards.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_yards(value: f64) -> f64 {
        value * 4840.0
    }

    /// Converts acres to hectares.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_hectares(value: f64) -> f64 {
        value * 0.40468564224
    }

    /// Converts acres to square miles.
    /// # Arguments
    /// * `value` - The area in acres to convert
    pub fn to_square_miles(value: f64) -> f64 {
        value / 640.0
    }
}

/// Hectares conversion functions
pub mod hectares {
    /// Converts hectares to square metres.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_metres(value: f64) -> f64 {
        value * 10_000.0
    }

    /// Converts hectares to square millimetres.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_millimetres(value: f64) -> f64 {
        super::square_metres::to_square_millimetres(to_square_metres(value))
    }

    /// Converts hectares to square centimetres.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_centimetres(value: f64) -> f64 {
        super::square_metres::to_square_centimetres(to_square_metres(value))
    }

    /// Converts hectares to square kilometres.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_kilometres(value: f64) -> f64 {
        value / 100.0
    }

    /// Converts hectares to square inches.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_inches(value: f64) -> f64 {
        super::square_metres::to_square_inches(to_square_metres(value))
    }

    /// Converts hectares to square feet.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_feet(value: f64) -> f64 {
        super::square_metres::to_square_feet(to_square_metres(value))
    }

    /// Converts hectares to square yards.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_yards(value: f64) -> f64 {
        super::square_metres::to_square_yards(to_square_metres(value))
    }

    /// Converts hectares to acres.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_acres(value: f64) -> f64 {
        value / 0.40468564224
    }

    /// Converts hectares to square miles.
    /// # Arguments
    /// * `value` - The area in hectares to convert
    pub fn to_square_miles(value: f64) -> f64 {
        super::square_metres::to_square_miles(to_square_metres(value))
    }
}

/// Square miles conversion functions
pub mod square_miles {
    /// Converts square miles to square metres.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_square_metres(value: f64) -> f64 {
        value * 2_589_988.110336
    }

    /// Converts square miles to square millimetres.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_square_millimetres(value: f64) -> f64 {
        super::square_metres::to_square_millimetres(to_square_metres(value))
    }

    /// Converts square miles to square centimetres.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_square_centimetres(value: f64) -> f64 {
        super::square_metres::to_square_centimetres(to_square_metres(value))
    }

    /// Converts square miles to square kilometres.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_square_kilometres(value: f64) -> f64 {
        value / 0.3861021585
    }

    /// Converts square miles to square inches.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_square_inches(value: f64) -> f64 {
        super::square_metres::to_square_inches(to_square_metres(value))
    }

    /// Converts square miles to square feet.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_square_feet(value: f64) -> f64 {
        value * 27878400.0
    }

    /// Converts square miles to square yards.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_square_yards(value: f64) -> f64 {
        value * 3097600.0
    }

    /// Converts square miles to acres.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_acres(value: f64) -> f64 {
        value * 640.0
    }

    /// Converts square miles to hectares.
    /// # Arguments
    /// * `value` - The area in square miles to convert
    pub fn to_hectares(value: f64) -> f64 {
        super::square_metres::to_hectares(to_square_metres(value))
    }
}

// Legacy function wrappers for backward compatibility
pub fn square_metres_to_square_millimetres(square_metres: f64) -> f64 {
    square_metres::to_square_millimetres(square_metres)
}

pub fn square_millimetres_to_square_metres(square_millimetres: f64) -> f64 {
    square_millimetres::to_square_metres(square_millimetres)
}

pub fn square_metres_to_square_centimetres(square_metres: f64) -> f64 {
    square_metres::to_square_centimetres(square_metres)
}

pub fn square_centimetres_to_square_metres(square_centimetres: f64) -> f64 {
    square_centimetres::to_square_metres(square_centimetres)
}

pub fn square_metres_to_square_kilometres(square_metres: f64) -> f64 {
    square_metres::to_square_kilometres(square_metres)
}

pub fn square_kilometres_to_square_metres(square_kilometres: f64) -> f64 {
    square_kilometres::to_square_metres(square_kilometres)
}

pub fn square_metres_to_square_feet(square_metres: f64) -> f64 {
    square_metres::to_square_feet(square_metres)
}

pub fn square_feet_to_square_metres(square_feet: f64) -> f64 {
    square_feet::to_square_metres(square_feet)
}

pub fn square_metres_to_acres(square_metres: f64) -> f64 {
    square_metres::to_acres(square_metres)
}

pub fn acres_to_square_metres(acres: f64) -> f64 {
    acres::to_square_metres(acres)
}

/// General area conversion function that accepts string unit names
///
/// Converts an area value from one unit to another using string identifiers.
/// This function is case-insensitive and supports common abbreviations.
///
/// # Arguments
///
/// * `value` - The numeric value to convert
/// * `from_unit` - The source unit (e.g., "m²", "cm²", "km²", "ft²", "in²", "ac", "ha")
/// * `to_unit` - The target unit using the same abbreviations
///
/// # Returns
/// * `Ok(f64)` - The converted value
/// * `Err(String)` - Error message if the conversion is not supported
///
/// # Examples
///
/// ```rust
/// use conversions_rs::convert_area;
///
/// let square_centimetres = convert_area(2.5, "m²", "cm²").unwrap();
/// assert_eq!(square_centimetres, 25000.0);
///
/// let acres = convert_area(10000.0, "m²", "ac").unwrap();
/// assert!((acres - 2.471).abs() < 0.001);
/// ```
pub fn convert_area(value: f64, from_unit: &str, to_unit: &str) -> Result<f64, String> {
    let from_unit = from_unit.to_lowercase();
    let to_unit = to_unit.to_lowercase();

    // Convert input to square metres first
    let square_metres = match from_unit.as_str() {
        "m²" | "m2" | "sq_m" | "square_metres" => value,
        "mm²" | "mm2" | "sq_mm" | "square_millimetres" => {
            square_millimetres::to_square_metres(value)
        }
        "cm²" | "cm2" | "sq_cm" | "square_centimetres" => {
            square_centimetres::to_square_metres(value)
        }
        "km²" | "km2" | "sq_km" | "square_kilometres" => {
            square_kilometres::to_square_metres(value)
        }
        "in²" | "in2" | "sq_in" | "square_inches" => square_inches::to_square_metres(value),
        "ft²" | "ft2" | "sq_ft" | "square_feet" => square_feet::to_square_metres(value),
        "yd²" | "yd2" | "sq_yd" | "square_yards" => square_yards::to_square_metres(value),
        "ac" | "acre" | "acres" => acres::to_square_metres(value),
        "ha" | "hectare" | "hectares" => hectares::to_square_metres(value),
        "mi²" | "mi2" | "sq_mi" | "square_miles" => square_miles::to_square_metres(value),
        _ => return Err(format!("Unsupported area unit: {}", from_unit)),
    };

    // Convert square metres to target unit
    let result = match to_unit.as_str() {
        "m²" | "m2" | "sq_m" | "square_metres" => square_metres,
        "mm²" | "mm2" | "sq_mm" | "square_millimetres" => {
            square_metres::to_square_millimetres(square_metres)
        }
        "cm²" | "cm2" | "sq_cm" | "square_centimetres" => {
            square_metres::to_square_centimetres(square_metres)
        }
        "km²" | "km2" | "sq_km" | "square_kilometres" => {
            square_metres::to_square_kilometres(square_metres)
        }
        "in²" | "in2" | "sq_in" | "square_inches" => {
            square_metres::to_square_inches(square_metres)
        }
        "ft²" | "ft2" | "sq_ft" | "square_feet" => square_metres::to_square_feet(square_metres),
        "yd²" | "yd2" | "sq_yd" | "square_yards" => square_metres::to_square_yards(square_metres),
        "ac" | "acre" | "acres" => square_metres::to_acres(square_metres),
        "ha" | "hectare" | "hectares" => square_metres::to_hectares(square_metres),
        "mi²" | "mi2" | "sq_mi" | "square_miles" => square_metres::to_square_miles(square_metres),
        _ => return Err(format!("Unsupported area unit: {}", to_unit)),
    };

    Ok(result)
}
