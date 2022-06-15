use crate::Expr::{self, *};
use std::ops::{BitAnd, BitOr, BitXor, Not};

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
