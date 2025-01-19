pub fn correct_errors_for_redundant_bool(input_value: u8) -> u8 {
    if input_value.count_ones() > 4 {
        u8::MAX
    } else {
        u8::MIN
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_errors_for_redundant_bool() {
        assert_eq!(correct_errors_for_redundant_bool(0b10011001u8), u8::MIN);
        assert_eq!(correct_errors_for_redundant_bool(0b11111100u8), u8::MAX);
        assert_eq!(correct_errors_for_redundant_bool(u8::MAX), u8::MAX);
        assert_eq!(correct_errors_for_redundant_bool(0b11111111u8), u8::MAX);
        assert_eq!(correct_errors_for_redundant_bool(0b00000000u8), u8::MIN);
        assert_eq!(correct_errors_for_redundant_bool(u8::MIN), u8::MIN);
        assert_eq!(correct_errors_for_redundant_bool(0b00000111u8), u8::MIN);
        assert_eq!(correct_errors_for_redundant_bool(0b01000011u8), u8::MIN);
    }

    #[test]
    fn test_normal_bool_conversion() {
        assert_eq!(true as u8, 0b00000001u8);
        assert_eq!(false as u8, 0b00000000u8);
        assert!(true as u8 != 0b11111111u8);
        assert!(false as u8 != 0b10000000u8);
    }
}
