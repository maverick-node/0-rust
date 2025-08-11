use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(value: u32) -> Self {
        match value {
            0 => RomanDigit::Nulla,
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => todo!(),
        }
    }
}
use std::collections::HashMap;

impl From<u32> for RomanNumber {
    fn from(mut num: u32) -> Self {
        let mut result = Vec::new();
if num == 0{
    return RomanNumber(vec![Nulla]);
}
        let mut symbols = HashMap::new();
        symbols.insert(1000, vec![RomanDigit::M]);
        symbols.insert(900, vec![RomanDigit::C, RomanDigit::M]);
        symbols.insert(500, vec![RomanDigit::D]);
        symbols.insert(400, vec![RomanDigit::C, RomanDigit::D]);
        symbols.insert(100, vec![RomanDigit::C]);
        symbols.insert(90, vec![RomanDigit::X, RomanDigit::C]);
        symbols.insert(50, vec![RomanDigit::L]);
        symbols.insert(40, vec![RomanDigit::X, RomanDigit::L]);
        symbols.insert(10, vec![RomanDigit::X]);
        symbols.insert(9, vec![RomanDigit::I, RomanDigit::X]);
        symbols.insert(5, vec![RomanDigit::V]);
        symbols.insert(4, vec![RomanDigit::I, RomanDigit::V]);
        symbols.insert(1, vec![RomanDigit::I]);
        

        let mut keys: Vec<u32> = symbols.keys().cloned().collect();
        keys.sort_unstable_by(|a, b| b.cmp(a));

        for &value in &keys {
            while num >= value {
                if let Some(digits) = symbols.get(&value) {
                    result.extend_from_slice(digits);
                    num -= value;
                }
            }
        }

        RomanNumber(result)
    }
}
