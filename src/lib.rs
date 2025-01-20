// lib.rs

//! Heavy Duty Bools are the beefier cousins of regular `bool` values
//! in Rust. While both normal bools and Heavy Duty Bools each take up 
//! only 8 bits of memory, Heavy Duty Bools come with much stronger
//! recoverability in the event of bit flips. 
//! 
//! This is due to Heavy Duty Bools being `u8` values where all the 
//! bits in the value are set to the same thing.
//! 
//! **Normal Bools**
//! 
//! True: `0b00000001`
//! 
//! False: `0b00000000`
//! 
//! **Heavy Duty Bools** 
//! 
//! Heavy Duty True: `0b11111111`
//! 
//! Heavy Duty False: `0b00000000`
//! 
//! With the Heavy Duty version, if I change any bit at random, the 
//! system can still figure out which Heavy Duty value the overall 
//! value is by calling the [refresh_hdbool()] function. 
//! 
//! On the other hand, normal bools don't have any level of inherent 
//! redundancy, as only the final bit determines the value of their 
//! entire unit. 

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
/// * `u8::MIN` otherwise.
pub fn refresh_hdbool(input_value: u8) -> u8 {
    if input_value.count_ones() > 4 {
        u8::MAX
    } else {
        u8::MIN
    }
}

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
        assert!(true as u8 != 0b11111111u8);
        assert!(false as u8 != 0b10000000u8);
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
        assert_eq!(convert_hdbool_to_bool(0b11111111u8), true);
        assert_eq!(convert_hdbool_to_bool(0b00000000_u8), false);
        assert_eq!(convert_hdbool_to_bool(0b00001111_u8), false);
    }

    #[test]
    fn test_convert_bool_to_hdbool() {
        assert_eq!(convert_bool_to_hdbool(false), 0b00000000_u8);
        assert_eq!(convert_bool_to_hdbool(true), 0b11111111_u8);
    }

}
