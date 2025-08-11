#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::cmp::{ Ord, Ordering };

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s {
            "A" => Antigen::A,
            "B" => Antigen::B,
            "AB" => Antigen::AB,
            "O" => Antigen::O,
            &_ => todo!(),
        };
        return Ok(r);
    }
}

impl FromStr for RhFactor {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rr = match s {
            "+" => RhFactor::Positive,
            "-" => RhFactor::Negative,
            &_ => todo!(),
        };
        return Ok(rr);
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.antigen.clone(), self.rh_factor.clone()).cmp(
            &(other.antigen.clone(), other.rh_factor.clone())
        )
    }
}
impl FromStr for BloodType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut anti: Antigen = Antigen::A;
        let mut rh: RhFactor = RhFactor::Negative;

        anti = s[0..s.len() - 1].parse::<Antigen>()?;
        rh = s[s.len() - 1..].parse::<RhFactor>()?;

        return Ok(BloodType { antigen: anti, rh_factor: rh });
    }
}

use std::fmt::{ self, Debug };

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rh = match self.rh_factor {
            RhFactor::Positive => "+",
            RhFactor::Negative => "-",
        };

        write!(f, "{:?}{}", self.antigen, rh)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let antigen_ok = if self.antigen == Antigen::AB {
            true
        } else if self.antigen == Antigen::A {
            other.antigen == Antigen::A || other.antigen == Antigen::O
        } else if self.antigen == Antigen::B {
            other.antigen == Antigen::B || other.antigen == Antigen::O
        } else {
            other.antigen == Antigen::O
        };
        let rh_ok = if self.rh_factor == RhFactor::Negative {
            other.rh_factor == RhFactor::Negative
        } else {
            true
        };

        antigen_ok && rh_ok
    }

    pub fn donors(&self) -> Vec<Self> {
        let mut result = Vec::new();

        let o_neg = BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative };
        let o_pos = BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive };
        let a_neg = BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative };
        let a_pos = BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive };
        let b_neg = BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative };
        let b_pos = BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive };
        let ab_neg = BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative };
        let ab_pos = BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive };

        if self.can_receive_from(&o_neg) {
            result.push(o_neg);
        }
        if self.can_receive_from(&o_pos) {
            result.push(o_pos);
        }
        if self.can_receive_from(&a_neg) {
            result.push(a_neg);
        }
        if self.can_receive_from(&a_pos) {
            result.push(a_pos);
        }
        if self.can_receive_from(&b_neg) {
            result.push(b_neg);
        }
        if self.can_receive_from(&b_pos) {
            result.push(b_pos);
        }
        if self.can_receive_from(&ab_neg) {
            result.push(ab_neg);
        }
        if self.can_receive_from(&ab_pos) {
            result.push(ab_pos);
        }

        result
    }

    pub fn recipients(&self) -> Vec<Self> {
        let mut result = Vec::new();

        let o_neg = BloodType { antigen: Antigen::O, rh_factor: RhFactor::Negative };
        let o_pos = BloodType { antigen: Antigen::O, rh_factor: RhFactor::Positive };
        let a_neg = BloodType { antigen: Antigen::A, rh_factor: RhFactor::Negative };
        let a_pos = BloodType { antigen: Antigen::A, rh_factor: RhFactor::Positive };
        let b_neg = BloodType { antigen: Antigen::B, rh_factor: RhFactor::Negative };
        let b_pos = BloodType { antigen: Antigen::B, rh_factor: RhFactor::Positive };
        let ab_neg = BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Negative };
        let ab_pos = BloodType { antigen: Antigen::AB, rh_factor: RhFactor::Positive };

        if o_neg.can_receive_from(self) {
            result.push(o_neg);
        }
        if o_pos.can_receive_from(self) {
            result.push(o_pos);
        }
        if a_neg.can_receive_from(self) {
            result.push(a_neg);
        }
        if a_pos.can_receive_from(self) {
            result.push(a_pos);
        }
        if b_neg.can_receive_from(self) {
            result.push(b_neg);
        }
        if b_pos.can_receive_from(self) {
            result.push(b_pos);
        }
        if ab_neg.can_receive_from(self) {
            result.push(ab_neg);
        }
        if ab_pos.can_receive_from(self) {
            result.push(ab_pos);
        }

        result
    }
}
