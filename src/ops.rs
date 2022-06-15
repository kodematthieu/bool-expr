use crate::Expr::{self, *};
use std::ops::{BitAnd, BitOr, BitXor, Not};

impl Expr {
    pub fn from_outputs<T: AsRef<[bool]>>(value: T) -> Self {
        use Expr::*;
        let slice = value.as_ref();
        assert!(slice.len().is_power_of_two(), "slice length must be in power of 2");
        assert!(slice.len() >= 2, "slice length must be 2 at minimum");
        match *slice {
            // 1 input, 2^2^1 cases
            [false, false] => Var(0) & !Var(0),
            [ true, false] => Var(0),
            [false,  true] => !Var(0),
            [ true,  true] => Var(0) | !Var(0),
            // 2 inputs, 2^2^2 cases
            [false, false, false, false] => Var(0) & !Var(0),
            [ true, false, false, false] => !(Var(0) | Var(1)),
            [false,  true, false, false] => Var(1) & !Var(0),
            [ true,  true, false, false] => Var(0),
            [false, false,  true, false] => Var(0) & !Var(1),
            [ true, false,  true, false] => !Var(1),
            [false,  true,  true, false] => Var(0) ^ Var(1),
            [ true,  true,  true, false] => !(Var(0) & Var(1)),
            [false, false, false,  true] => Var(0) & Var(1),
            [ true, false, false,  true] => !(Var(0) ^ Var(1)),
            [false,  true, false,  true] => Var(1),
            [ true,  true, false,  true] => Var(1) | !Var(0),
            [false, false,  true,  true] => Var(0),
            [ true, false,  true,  true] => Var(0) | !Var(1),
            [false,  true,  true,  true] => Var(0) | Var(1),
            [ true,  true,  true,  true] => Var(0) | !Var(0),
        
            // Rest: 3-256 inputs, (2^2^3)-(2^2^256) cases
            ref value => {
                let mut a = Self::from_outputs(&value[..(value.len() / 2)]);
                let mut b = Self::from_outputs(&value[(value.len() / 2)..]);
                a.inc_vars();
                b.inc_vars();
                (!Var(0) & a) | (Var(0) & b)
            }
        }
    }
}

impl BitXor for Expr {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self {
        Xor(self.into(), rhs.into())
    }
}
impl BitAnd for Expr {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        And(self.into(), rhs.into())
    }
}
impl BitOr for Expr {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Or(self.into(), rhs.into())
    }
}
impl Not for Expr {
    type Output = Self;
    fn not(self) -> Self {
        Not(self.into())
    }
}
