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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_bool_representation() {
        assert_eq!(true as u8, 0b_0000_0001_u8);
        assert_eq!(false as u8, 0b_0000_0000_u8);
        assert_ne!(true as u8, 0b_1111_1111_u8);
        assert_ne!(false as u8, 0b_1000_0000_u8);
        assert_ne!(true as u8, 0b_0100_0010_u8);
        assert_ne!(false as u8, 0b_0010_0001_u8);
        assert_ne!(true as u8, HDBool::new(true).as_u8());
        assert_ne!(false as u8, HDBool::new(true).as_u8());
        assert_ne!(true as u8, HDBool::new(false).as_u8());
        assert_eq!(false as u8, HDBool::new(false).as_u8());
    }

    #[test]
    fn test_hdbool() {
        assert_eq!(HDBool::new(true), HDBool(u8::MAX));
        assert_eq!(HDBool::new(false), HDBool(u8::MIN));
        assert_eq!(HDBool::new(true).to_bool(), true);
        assert_eq!(HDBool::new(false).to_bool(), false);
        assert_eq!(HDBool::new(true).as_u8(), u8::MAX);
        assert_eq!(HDBool::new(false).as_u8(), u8::MIN);

        assert_eq!(HDBool::from_u8(0b_1111_1111_u8), HDBool(u8::MAX));
        assert_eq!(HDBool::from_u8(0b_0000_0000_u8), HDBool(u8::MIN));
        assert_eq!(HDBool::from_u8(0b_1111_1111_u8).to_bool(), true);
        assert_eq!(HDBool::from_u8(0b_0000_0000_u8).to_bool(), false);
        assert_eq!(HDBool::from_u8(0b_1111_1111_u8).as_u8(), u8::MAX);
        assert_eq!(HDBool::from_u8(0b_0000_0000_u8).as_u8(), u8::MIN);

        assert_eq!(HDBool(u8::MAX).as_u8(), 0b_1111_1111_u8);
        assert_eq!(HDBool(u8::MIN).as_u8(), 0b_0000_0000_u8);
        assert_ne!(HDBool(u8::MAX).as_u8(), 0b_0000_0000_u8);
        assert_ne!(HDBool(u8::MIN).as_u8(), 0b_1111_1111_u8);
        assert_ne!(HDBool(u8::MAX).as_u8(), HDBool::new(false).as_u8());
        assert_ne!(HDBool(u8::MIN).as_u8(), HDBool::new(true).as_u8());
        assert!(HDBool(u8::MAX).to_bool());
        assert!(!HDBool(u8::MIN).to_bool());
        // CONTINUE TO TEST ALL THE METHODS HERE
    }

    #[test]
    fn test_equality_of_different_representations_for_hdbools() {
        assert_eq!(HDBool::new(true).as_u8(), 0b_1111_1111_u8);
        assert_eq!(HDBool::new(true).as_u8(), u8::MAX);
        assert_eq!(HDBool::new(false).as_u8(), 0b_0000_0000_u8);
        assert_eq!(HDBool::new(false).as_u8(), u8::MIN);
    }

}
