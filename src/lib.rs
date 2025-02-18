// lib.rs

#![doc = include_str!("../README.md")]

//! # HDBool
//! 
//! Check out the [HDBool] struct for more information.
//! 

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
    use std::u8;

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

        assert!(true);
        assert!(!false);
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
        assert_eq!(HDBool::from_u8(0b_1111_1110_u8).to_bool(), true);
        assert_eq!(HDBool::from_u8(0b_0000_0001_u8).to_bool(), false);

        assert_eq!(HDBool(u8::MAX).as_u8(), 0b_1111_1111_u8);
        assert_eq!(HDBool(u8::MIN).as_u8(), 0b_0000_0000_u8);
        assert_ne!(HDBool(u8::MAX).as_u8(), 0b_0000_0000_u8);
        assert_ne!(HDBool(u8::MIN).as_u8(), 0b_1111_1111_u8);
        assert_ne!(HDBool(u8::MAX).as_u8(), HDBool::new(false).as_u8());
        assert_ne!(HDBool(u8::MIN).as_u8(), HDBool::new(true).as_u8());

        assert!(HDBool(u8::MAX).to_bool());
        assert!(!HDBool(u8::MIN).to_bool());

        assert_eq!(HDBool(u8::MAX).to_bool(), true);
        assert_eq!(HDBool(u8::MIN).to_bool(), false);
        assert_ne!(HDBool(u8::MAX).to_bool(), false);
        assert_ne!(HDBool(u8::MIN).to_bool(), true);
        assert_eq!(HDBool(u8::MAX).as_u8(), u8::MAX);
        assert_eq!(HDBool(u8::MIN).as_u8(), u8::MIN);

        assert_ne!(HDBool(HDBool::refresh(0b_1111_1111_u8)).to_bool(), false);
        assert_ne!(HDBool(HDBool::refresh(0b_0000_0000_u8)).to_bool(), true);
        assert_eq!(HDBool(HDBool::refresh(0b_1111_1111_u8)).to_bool(), true);
        assert_eq!(HDBool(HDBool::refresh(0b_0000_0000_u8)).to_bool(), false);
        assert_eq!(HDBool(HDBool::refresh(0b_1111_1111_u8)).as_u8(), u8::MAX);
        assert_eq!(HDBool(HDBool::refresh(0b_0000_0000_u8)).as_u8(), u8::MIN);
        assert_eq!(HDBool(HDBool::refresh(0b_0000_1111_u8)).as_u8(), u8::MIN);
        assert_eq!(HDBool(HDBool::refresh(0b_1111_0000_u8)).as_u8(), u8::MIN);
        assert_eq!(HDBool(HDBool::refresh(0b_0000_1111_u8)).to_bool(), false);
        assert_eq!(HDBool(HDBool::refresh(0b_1111_0000_u8)).to_bool(), false);
        assert_eq!(HDBool(HDBool::refresh(0b_0100_1111_u8)).to_bool(), true);

        assert_eq!(HDBool::refresh(0b_1111_1111_u8), u8::MAX);
        assert_eq!(HDBool::refresh(0b_0000_0000_u8), u8::MIN);
        assert_eq!(HDBool::refresh(0b_1111_1110_u8), u8::MAX);
        assert_eq!(HDBool::refresh(0b_0000_0001_u8), u8::MIN);
        assert_eq!(HDBool::refresh(0b_1111_1100_u8), u8::MAX);
        assert_eq!(HDBool::refresh(0b_0000_0011_u8), u8::MIN);
        assert_eq!(HDBool::refresh(0b_1111_1000_u8), u8::MAX);
        assert_eq!(HDBool::refresh(0b_0000_0111_u8), u8::MIN);
        assert_eq!(HDBool::refresh(0b_1111_0000_u8), u8::MIN);
        assert_eq!(HDBool::refresh(0b_0000_1111_u8), u8::MIN);
    }

    #[test]
    fn test_memory_footprint() {
        assert_eq!(std::mem::size_of::<HDBool>(), 1);
        assert_eq!(std::mem::size_of::<u8>(), 1);
        assert_eq!(std::mem::size_of::<bool>(), 1);
        assert!(std::mem::size_of::<HDBool>() <= std::mem::size_of::<u8>());

        assert_eq!(std::mem::size_of_val(&HDBool::new(true)), 1);
        assert_eq!(std::mem::size_of_val(&HDBool::new(false)), 1);
        assert_eq!(std::mem::size_of_val(&HDBool::from_u8(0b_1111_1111_u8)), 1);
        assert_eq!(std::mem::size_of_val(&HDBool::from_u8(0b_0000_0000_u8)), 1);
        assert_eq!(std::mem::size_of_val(&HDBool(u8::MAX)), 1);
        assert_eq!(std::mem::size_of_val(&HDBool(u8::MIN)), 1);
        assert!(std::mem::size_of_val(&HDBool::new(true)) <= std::mem::size_of_val(&u8::MAX));
    }

    #[test]
    fn test_equality_of_different_representations_for_hdbools() {
        assert_eq!(HDBool::new(true).as_u8(), 0b_1111_1111_u8);
        assert_eq!(HDBool::new(true).as_u8(), u8::MAX);
        assert_eq!(HDBool::new(false).as_u8(), 0b_0000_0000_u8);
        assert_eq!(HDBool::new(false).as_u8(), u8::MIN);
    }

}
