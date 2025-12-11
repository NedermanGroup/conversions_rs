//! Length conversion functions organized by unit type
//! All conversions use metres as the base unit for accuracy and consistency

/// Metre conversion functions
pub mod metres {
    /// Converts metres to feet.
    /// # Arguments
    /// * `value` - The length in metres to convert
    pub fn to_feet(value: f64) -> f64 {
        value * 3.28084
    }

    /// Converts metres to inches.
    /// # Arguments
    /// * `value` - The length in metres to convert
    pub fn to_inches(value: f64) -> f64 {
        value * 39.3701
    }

    /// Converts metres to kilometres.
    /// # Arguments
    /// * `value` - The length in metres to convert
    pub fn to_kilometres(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts metres to centimetres.
    /// # Arguments
    /// * `value` - The length in metres to convert
    pub fn to_centimetres(value: f64) -> f64 {
        value * 100.0
    }

    /// Converts metres to millimetres.
    /// # Arguments
    /// * `value` - The length in metres to convert
    pub fn to_millimetres(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts metres to yards.
    /// # Arguments
    /// * `value` - The length in metres to convert
    pub fn to_yards(value: f64) -> f64 {
        value * 1.09361
    }

    /// Converts metres to miles.
    /// # Arguments
    /// * `value` - The length in metres to convert
    pub fn to_miles(value: f64) -> f64 {
        value / 1609.34
    }
}

/// Feet conversion functions
pub mod feet {
    /// Converts feet to metres.
    /// # Arguments
    /// * `value` - The length in feet to convert
    pub fn to_metres(value: f64) -> f64 {
        value / 3.28084
    }

    /// Converts feet to inches.
    /// # Arguments
    /// * `value` - The length in feet to convert
    pub fn to_inches(value: f64) -> f64 {
        value * 12.0
    }

    /// Converts feet to yards.
    /// # Arguments
    /// * `value` - The length in feet to convert
    pub fn to_yards(value: f64) -> f64 {
        value / 3.0
    }

    /// Converts feet to miles.
    /// # Arguments
    /// * `value` - The length in feet to convert
    pub fn to_miles(value: f64) -> f64 {
        value / 5280.0
    }

    /// Converts feet to centimetres.
    /// # Arguments
    /// * `value` - The length in feet to convert
    pub fn to_centimetres(value: f64) -> f64 {
        super::metres::to_centimetres(to_metres(value))
    }

    /// Converts feet to millimetres.
    /// # Arguments
    /// * `value` - The length in feet to convert
    pub fn to_millimetres(value: f64) -> f64 {
        super::metres::to_millimetres(to_metres(value))
    }

    /// Converts feet to kilometres.
    /// # Arguments
    /// * `value` - The length in feet to convert
    pub fn to_kilometres(value: f64) -> f64 {
        super::metres::to_kilometres(to_metres(value))
    }
}

/// Inches conversion functions
pub mod inches {
    /// Converts inches to metres.
    /// # Arguments
    /// * `value` - The length in inches to convert
    pub fn to_metres(value: f64) -> f64 {
        value / 39.3701
    }

    /// Converts inches to feet.
    /// # Arguments
    /// * `value` - The length in inches to convert
    pub fn to_feet(value: f64) -> f64 {
        value / 12.0
    }

    /// Converts inches to yards.
    /// # Arguments
    /// * `value` - The length in inches to convert
    pub fn to_yards(value: f64) -> f64 {
        value / 36.0
    }

    /// Converts inches to centimetres.
    /// # Arguments
    /// * `value` - The length in inches to convert
    pub fn to_centimetres(value: f64) -> f64 {
        value * 2.54
    }

    /// Converts inches to millimetres.
    /// # Arguments
    /// * `value` - The length in inches to convert
    pub fn to_millimetres(value: f64) -> f64 {
        value * 25.4
    }

    /// Converts inches to kilometres.
    /// # Arguments
    /// * `value` - The length in inches to convert
    pub fn to_kilometres(value: f64) -> f64 {
        super::metres::to_kilometres(to_metres(value))
    }

    /// Converts inches to miles.
    /// # Arguments
    /// * `value` - The length in inches to convert
    pub fn to_miles(value: f64) -> f64 {
        super::metres::to_miles(to_metres(value))
    }
}

/// Kilometres conversion functions
pub mod kilometres {
    /// Converts kilometres to metres.
    /// # Arguments
    /// * `value` - The length in kilometres to convert
    pub fn to_metres(value: f64) -> f64 {
        value * 1000.0
    }

    /// Converts kilometres to feet.
    /// # Arguments
    /// * `value` - The length in kilometres to convert
    pub fn to_feet(value: f64) -> f64 {
        super::metres::to_feet(to_metres(value))
    }

    /// Converts kilometres to inches.
    /// # Arguments
    /// * `value` - The length in kilometres to convert
    pub fn to_inches(value: f64) -> f64 {
        super::metres::to_inches(to_metres(value))
    }

    /// Converts kilometres to yards.
    /// # Arguments
    /// * `value` - The length in kilometres to convert
    pub fn to_yards(value: f64) -> f64 {
        super::metres::to_yards(to_metres(value))
    }

    /// Converts kilometres to miles.
    /// # Arguments
    /// * `value` - The length in kilometres to convert
    pub fn to_miles(value: f64) -> f64 {
        value / 1.60934
    }

    /// Converts kilometres to centimetres.
    /// # Arguments
    /// * `value` - The length in kilometres to convert
    pub fn to_centimetres(value: f64) -> f64 {
        value * 100000.0
    }

    /// Converts kilometres to millimetres.
    /// # Arguments
    /// * `value` - The length in kilometres to convert
    pub fn to_millimetres(value: f64) -> f64 {
        value * 1000000.0
    }
}

/// Centimetres conversion functions
pub mod centimetres {
    /// Converts centimetres to metres.
    /// # Arguments
    /// * `value` - The length in centimetres to convert
    pub fn to_metres(value: f64) -> f64 {
        value / 100.0
    }

    /// Converts centimetres to feet.
    /// # Arguments
    /// * `value` - The length in centimetres to convert
    pub fn to_feet(value: f64) -> f64 {
        super::metres::to_feet(to_metres(value))
    }

    /// Converts centimetres to inches.
    /// # Arguments
    /// * `value` - The length in centimetres to convert
    pub fn to_inches(value: f64) -> f64 {
        value / 2.54
    }

    /// Converts centimetres to yards.
    /// # Arguments
    /// * `value` - The length in centimetres to convert
    pub fn to_yards(value: f64) -> f64 {
        super::metres::to_yards(to_metres(value))
    }

    /// Converts centimetres to miles.
    /// # Arguments
    /// * `value` - The length in centimetres to convert
    pub fn to_miles(value: f64) -> f64 {
        super::metres::to_miles(to_metres(value))
    }

    /// Converts centimetres to kilometres.
    /// # Arguments
    /// * `value` - The length in centimetres to convert
    pub fn to_kilometres(value: f64) -> f64 {
        value / 100000.0
    }

    /// Converts centimetres to millimetres.
    /// # Arguments
    /// * `value` - The length in centimetres to convert
    pub fn to_millimetres(value: f64) -> f64 {
        value * 10.0
    }
}

/// Millimetres conversion functions
pub mod millimetres {
    /// Converts millimetres to metres.
    /// # Arguments
    /// * `value` - The length in millimetres to convert
    pub fn to_metres(value: f64) -> f64 {
        value / 1000.0
    }

    /// Converts millimetres to feet.
    /// # Arguments
    /// * `value` - The length in millimetres to convert
    pub fn to_feet(value: f64) -> f64 {
        super::metres::to_feet(to_metres(value))
    }

    /// Converts millimetres to inches.
    /// # Arguments
    /// * `value` - The length in millimetres to convert
    pub fn to_inches(value: f64) -> f64 {
        value / 25.4
    }

    /// Converts millimetres to yards.
    /// # Arguments
    /// * `value` - The length in millimetres to convert
    pub fn to_yards(value: f64) -> f64 {
        super::metres::to_yards(to_metres(value))
    }

    /// Converts millimetres to miles.
    /// # Arguments
    /// * `value` - The length in millimetres to convert
    pub fn to_miles(value: f64) -> f64 {
        super::metres::to_miles(to_metres(value))
    }

    /// Converts millimetres to kilometres.
    /// # Arguments
    /// * `value` - The length in millimetres to convert
    pub fn to_kilometres(value: f64) -> f64 {
        value / 1000000.0
    }

    /// Converts millimetres to centimetres.
    /// # Arguments
    /// * `value` - The length in millimetres to convert
    pub fn to_centimetres(value: f64) -> f64 {
        value / 10.0
    }
}

/// Yards conversion functions
pub mod yards {
    /// Converts yards to metres.
    /// # Arguments
    /// * `value` - The length in yards to convert
    pub fn to_metres(value: f64) -> f64 {
        value / 1.09361
    }

    /// Converts yards to feet.
    /// # Arguments
    /// * `value` - The length in yards to convert
    pub fn to_feet(value: f64) -> f64 {
        value * 3.0
    }

    /// Converts yards to inches.
    /// # Arguments
    /// * `value` - The length in yards to convert
    pub fn to_inches(value: f64) -> f64 {
        value * 36.0
    }

    /// Converts yards to miles.
    /// # Arguments
    /// * `value` - The length in yards to convert
    pub fn to_miles(value: f64) -> f64 {
        value / 1760.0
    }

    /// Converts yards to kilometres.
    /// # Arguments
    /// * `value` - The length in yards to convert
    pub fn to_kilometres(value: f64) -> f64 {
        super::metres::to_kilometres(to_metres(value))
    }

    /// Converts yards to centimetres.
    /// # Arguments
    /// * `value` - The length in yards to convert
    pub fn to_centimetres(value: f64) -> f64 {
        super::metres::to_centimetres(to_metres(value))
    }

    /// Converts yards to millimetres.
    /// # Arguments
    /// * `value` - The length in yards to convert
    pub fn to_millimetres(value: f64) -> f64 {
        super::metres::to_millimetres(to_metres(value))
    }
}

/// Miles conversion functions
pub mod miles {
    /// Converts miles to metres.
    /// # Arguments
    /// * `value` - The length in miles to convert
    pub fn to_metres(value: f64) -> f64 {
        value * 1609.34
    }

    /// Converts miles to feet.
    /// # Arguments
    /// * `value` - The length in miles to convert
    pub fn to_feet(value: f64) -> f64 {
        value * 5280.0
    }

    /// Converts miles to inches.
    /// # Arguments
    /// * `value` - The length in miles to convert
    pub fn to_inches(value: f64) -> f64 {
        value * 63360.0
    }

    /// Converts miles to yards.
    /// # Arguments
    /// * `value` - The length in miles to convert
    pub fn to_yards(value: f64) -> f64 {
        value * 1760.0
    }

    /// Converts miles to kilometres.
    /// # Arguments
    /// * `value` - The length in miles to convert
    pub fn to_kilometres(value: f64) -> f64 {
        value * 1.60934
    }

    /// Converts miles to centimetres.
    /// # Arguments
    /// * `value` - The length in miles to convert
    pub fn to_centimetres(value: f64) -> f64 {
        super::metres::to_centimetres(to_metres(value))
    }

    /// Converts miles to millimetres.
    /// # Arguments
    /// * `value` - The length in miles to convert
    pub fn to_millimetres(value: f64) -> f64 {
        super::metres::to_millimetres(to_metres(value))
    }
}

// Legacy function wrappers for backward compatibility
/// Converts metres to feet (legacy function).
///
/// **Note:** Consider using `metres::to_feet()` for better organization.
///
/// # Arguments
/// * `metres` - The length in metres to convert
pub fn metres_to_feet(metres: f64) -> f64 {
    metres::to_feet(metres)
}

/// Converts feet to metres (legacy function).
///
/// **Note:** Consider using `feet::to_metres()` for better organization.
///
/// # Arguments
/// * `feet` - The length in feet to convert
pub fn feet_to_metres(feet: f64) -> f64 {
    feet::to_metres(feet)
}

/// Converts metres to inches (legacy function).
///
/// **Note:** Consider using `metres::to_inches()` for better organization.
///
/// # Arguments
/// * `metres` - The length in metres to convert
pub fn metres_to_inches(metres: f64) -> f64 {
    metres::to_inches(metres)
}

/// Converts inches to metres (legacy function).
///
/// **Note:** Consider using `inches::to_metres()` for better organization.
///
/// # Arguments
/// * `inches` - The length in inches to convert
pub fn inches_to_metres(inches: f64) -> f64 {
    inches::to_metres(inches)
}

/// Converts metres to kilometres (legacy function).
///
/// **Note:** Consider using `metres::to_kilometres()` for better organization.
///
/// # Arguments
/// * `metres` - The length in metres to convert
pub fn metres_to_kilometres(metres: f64) -> f64 {
    metres::to_kilometres(metres)
}

/// Converts kilometres to metres (legacy function).
///
/// **Note:** Consider using `kilometres::to_metres()` for better organization.
///
/// # Arguments
/// * `kilometres` - The length in kilometres to convert
pub fn kilometres_to_metres(kilometres: f64) -> f64 {
    kilometres::to_metres(kilometres)
}

/// Converts metres to centimetres (legacy function).
///
/// **Note:** Consider using `metres::to_centimetres()` for better organization.
///
/// # Arguments
/// * `metres` - The length in metres to convert
pub fn metres_to_centimetres(metres: f64) -> f64 {
    metres::to_centimetres(metres)
}

/// Converts centimetres to metres (legacy function).
///
/// **Note:** Consider using `centimetres::to_metres()` for better organization.
///
/// # Arguments
/// * `centimetres` - The length in centimetres to convert
pub fn centimetres_to_metres(centimetres: f64) -> f64 {
    centimetres::to_metres(centimetres)
}

/// Converts metres to millimetres (legacy function).
///
/// **Note:** Consider using `metres::to_millimetres()` for better organization.
///
/// # Arguments
/// * `metres` - The length in metres to convert
pub fn metres_to_millimetres(metres: f64) -> f64 {
    metres::to_millimetres(metres)
}

/// Converts millimetres to metres (legacy function).
///
/// **Note:** Consider using `millimetres::to_metres()` for better organization.
///
/// # Arguments
/// * `millimetres` - The length in millimetres to convert
pub fn millimetres_to_metres(millimetres: f64) -> f64 {
    millimetres::to_metres(millimetres)
}

/// Converts metres to yards (legacy function).
///
/// **Note:** Consider using `metres::to_yards()` for better organization.
///
/// # Arguments
/// * `metres` - The length in metres to convert
pub fn metres_to_yards(metres: f64) -> f64 {
    metres::to_yards(metres)
}

/// Converts yards to metres (legacy function).
///
/// **Note:** Consider using `yards::to_metres()` for better organization.
///
/// # Arguments
/// * `yards` - The length in yards to convert
pub fn yards_to_metres(yards: f64) -> f64 {
    yards::to_metres(yards)
}

/// Converts metres to miles (legacy function).
///
/// **Note:** Consider using `metres::to_miles()` for better organization.
///
/// # Arguments
/// * `metres` - The length in metres to convert
pub fn metres_to_miles(metres: f64) -> f64 {
    metres::to_miles(metres)
}

/// Converts miles to metres (legacy function).
///
/// **Note:** Consider using `miles::to_metres()` for better organization.
///
/// # Arguments
/// * `miles` - The length in miles to convert
pub fn miles_to_metres(miles: f64) -> f64 {
    miles::to_metres(miles)
}

/// Converts between any two supported length units.
///
/// This is the main conversion function that can handle conversions between any
/// combination of supported length units. All conversions are done through metres
/// as an intermediate base unit to ensure accuracy and consistency.
///
/// # Supported Units
///
/// * **Metric:** `m`, `metre`, `metres`, `km`, `kilometre`, `kilometres`,
///   `cm`, `centimetre`, `centimetres`, `mm`, `millimetre`, `millimetres`
/// * **Imperial:** `ft`, `foot`, `feet`, `in`, `inch`, `inches`,
///   `yd`, `yard`, `yards`, `mi`, `mile`, `miles`
///
/// Unit names are case-insensitive and support both singular and plural forms.
///
/// # Arguments
///
/// * `value` - The numeric value to convert
/// * `from` - The source unit (case-insensitive)
/// * `to` - The target unit (case-insensitive)
///
/// # Returns
/// * `Ok(f64)` - The converted value if both units are recognized
/// * `Err(String)` - An error message if either unit is not supported
///
/// # Examples
///
/// ```
/// use conversions_rs::convert_length;
///
/// // Convert 100 feet to metres
/// let result = convert_length(100.0, "ft", "m").unwrap();
/// assert!((result - 30.48).abs() < 0.01);
///
/// // Convert 5 kilometres to miles
/// let result = convert_length(5.0, "km", "mi").unwrap();
/// assert!((result - 3.10686).abs() < 0.001);
///
/// // Convert 42 inches to centimetres
/// let result = convert_length(42.0, "in", "cm").unwrap();
/// assert!((result - 106.68).abs() < 0.01);
///
/// // Case-insensitive and plural forms work
/// let result = convert_length(1.0, "Metre", "feet").unwrap();
/// assert!((result - 3.28084).abs() < 0.0001);
///
/// // Error handling for unknown units
/// assert!(convert_length(1.0, "invalid", "m").is_err());
/// ```
///
/// # Conversion Accuracy
///
/// All conversions use standard international conversion factors:
/// - 1 metre = 3.28084 feet
/// - 1 metre = 39.3701 inches  
/// - 1 mile = 1609.34 metres
/// - etc.
///
/// Results maintain high precision suitable for most applications.
pub fn convert_length(value: f64, from: &str, to: &str) -> Result<f64, String> {
    // First convert to metres (base unit)
    let metres = match from.to_lowercase().as_str() {
        "m" | "metre" | "metres" => value,
        "km" | "kilometre" | "kilometres" => kilometres_to_metres(value),
        "cm" | "centimetre" | "centimetres" => centimetres_to_metres(value),
        "mm" | "millimetre" | "millimetres" => millimetres_to_metres(value),
        "ft" | "foot" | "feet" => feet_to_metres(value),
        "in" | "inch" | "inches" => inches_to_metres(value),
        "yd" | "yard" | "yards" => yards_to_metres(value),
        "mi" | "mile" | "miles" => miles_to_metres(value),
        _ => return Err(format!("Unknown length unit: {}", from)),
    };

    // Then convert from metres to target unit
    let result = match to.to_lowercase().as_str() {
        "m" | "metre" | "metres" => metres,
        "km" | "kilometre" | "kilometres" => metres_to_kilometres(metres),
        "cm" | "centimetre" | "centimetres" => metres_to_centimetres(metres),
        "mm" | "millimetre" | "millimetres" => metres_to_millimetres(metres),
        "ft" | "foot" | "feet" => metres_to_feet(metres),
        "in" | "inch" | "inches" => metres_to_inches(metres),
        "yd" | "yard" | "yards" => metres_to_yards(metres),
        "mi" | "mile" | "miles" => metres_to_miles(metres),
        _ => return Err(format!("Unknown length unit: {}", to)),
    };

    Ok(result)
}
