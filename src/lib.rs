// lib.rs

#![doc = include_str!("../README.md")]




#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HDBool(u8);

impl HDBool {
    /// Creates an `HDBool` from a normal bool.
    pub fn new(val: bool) -> Self {
        HDBool(if val { u8::MAX } else { u8::MIN })
    }

    /// Creates an `HDBool` from a raw `u8`, refreshing it if it's imperfect.
    pub fn from_u8(raw: u8) -> Self {
        HDBool(Self::refresh(raw))
    }

    /// Extracts the raw `u8` representation (all 1s or all 0s after refresh).
    pub fn as_u8(&self) -> u8 {
        self.0
    }

    /// Returns the normal bool form.
    pub fn to_bool(&self) -> bool {
        // any value other than 0xFF or 0x00 is first corrected
        match self.0 {
            0xFF => true,
            0x00 => false,
            _ => HDBool::refresh(self.0) == u8::MAX,
        }
    }

    /// Refresh a raw `u8` into either 0xFF or 0x00.
    fn refresh(raw: u8) -> u8 {
        if raw.count_ones() > 4 {
            u8::MAX
        } else {
            u8::MIN
        }
    }
}

/// Refreshes the `u8` value based on the density of `1` bits in its 
/// binary representation in order to reverse the damage of bit flips.
/// 
/// If the number of `1` bits exceeds 4, returns `u8::MAX` (all `1`s).
/// Otherwise, returns `u8::MIN` (all `0`s).
///
/// # Arguments
///
/// * `input_value` - A `u8` value whose `1` bits are to be counted.
///
/// # Returns
///
/// * `u8::MAX` if `input_value` contains more than 4 `1` bits.
/// * `u8::MIN` otherwise. Keep in mind, this means that if there is
///   a tie (exactly four `0`'s and four `1`'s, the value will evaluate
///   to false).
pub fn refresh_hdbool(input_value: u8) -> u8 {
    if input_value.count_ones() > 4 {
        u8::MAX
    } else {
        u8::MIN
    }
}

/// Converts a normal boolean to a Heavy Duty Boolean. 
pub fn convert_bool_to_hdbool(input_bool: bool) -> u8 {
    match input_bool {
        false => u8::MIN,
        true => u8::MAX,
    }
}

/// Converts a Heavy Duty Bool to a normal bool.
/// 
/// If the Heavy Duty Bool is imperfect, it gets corrected with 
/// `refresh_hdbool()` first. 
pub fn convert_hdbool_to_bool(hdbool: u8) -> bool {
    match hdbool {
        u8::MAX => true,
        u8::MIN => false,
        _ => convert_hdbool_to_bool(refresh_hdbool(hdbool)), 

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_bool_representation() {
        assert_eq!(true as u8, 0b00000001u8);
        assert_eq!(false as u8, 0b00000000u8);
        assert_ne!(true as u8, 0b11111111u8);
        assert_ne!(false as u8, 0b10000000u8);
        assert_ne!(true as u8, HDBool::new(true).as_u8());
        assert_ne!(false as u8, HDBool::new(true).as_u8());
        assert_ne!(true as u8, HDBool::new(false).as_u8());
        assert_eq!(false as u8, HDBool::new(false).as_u8());

    }

    #[test]
    fn test_hdbool() {
        assert_eq!(HDBool::new(true), HDBool(u8::MAX));
        // CONTINUE TO TEST ALL THE METHODS HERE
    }

    #[test]
    fn test_refresh_bool() {
        assert_eq!(refresh_hdbool(0b10011001u8), u8::MIN);
        assert_eq!(refresh_hdbool(0b11111100u8), u8::MAX);
        assert_eq!(refresh_hdbool(u8::MAX), u8::MAX);
        assert_eq!(refresh_hdbool(0b11111111u8), u8::MAX);
        assert_eq!(refresh_hdbool(0b00000000u8), u8::MIN);
        assert_eq!(refresh_hdbool(u8::MIN), u8::MIN);
        assert_eq!(refresh_hdbool(0b00000111u8), u8::MIN);
        assert_eq!(refresh_hdbool(0b01000011u8), u8::MIN);
        assert_eq!(refresh_hdbool(0b00001111u8), u8::MIN);
    }

    #[test]
    fn test_convert_hdbool_to_bool() {
        assert_eq!(convert_hdbool_to_bool(0b11111111_u8), true);
        assert_eq!(convert_hdbool_to_bool(0b00000000_u8), false);
        assert_eq!(convert_hdbool_to_bool(0b00001111_u8), false);
        assert_eq!(convert_hdbool_to_bool(0b11110000_u8), false);
    }

    #[test]
    fn test_convert_bool_to_hdbool() {
        assert_eq!(convert_bool_to_hdbool(false), 0b00000000_u8);
        assert_eq!(convert_bool_to_hdbool(true), 0b11111111_u8);
    }

    #[test]
    fn test_equality_of_different_representations_for_hdbools() {
        assert_eq!(HDBool::new(true).as_u8(), 0b_1111_1111_u8);
        assert_eq!(HDBool::new(true).as_u8(), u8::MAX);
        assert_eq!(HDBool::new(false).as_u8(), 0b_0000_0000_u8);
        assert_eq!(HDBool::new(false).as_u8(), u8::MIN);
    }

}
