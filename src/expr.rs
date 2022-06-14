use crate::algo;
use std::cmp::{min, max};
use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Clone, Eq, PartialEq)]
pub enum Expr {
    #[non_exhaustive] Var(u8),
    #[non_exhaustive] Not(Box<Self>),
    #[non_exhaustive] Xor(Box<Self>, Box<Self>),
    #[non_exhaustive] And(Box<Self>, Box<Self>),
    #[non_exhaustive] Or(Box<Self>, Box<Self>),
}
impl Expr {
    fn lowest(&self) -> u8 {
        match self {
            &Self::Var(id) => id,
            &Self::Not(ref e) => e.lowest(),
            &Self::Xor(ref a, ref b) => min(a.lowest(), b.lowest()),
            &Self::And(ref a, ref b) => min(a.lowest(), b.lowest()),
            &Self::Or(ref a, ref b) => min(a.lowest(), b.lowest())
        }
    }
    fn highest(&self) -> u8 {
        match self {
            &Self::Var(id) => id,
            &Self::Not(ref e) => e.highest(),
            &Self::Xor(ref a, ref b) => max(a.highest(), b.highest()),
            &Self::And(ref a, ref b) => max(a.highest(), b.highest()),
            &Self::Or(ref a, ref b) => max(a.highest(), b.highest())
        }
    }
    fn inc_vars(&mut self) {
    match self {
        &mut Self::Var(ref mut id) => *id += 1, 
        &mut Self::Not(ref mut e) => e.inc_vars(),
        &mut Self::Xor(ref mut a, ref mut b) => {a.inc_vars(); b.inc_vars()},
        &mut Self::And(ref mut a, ref mut b) => {a.inc_vars(); b.inc_vars()},
        &mut Self::Or(ref mut a, ref mut b) => {a.inc_vars(); b.inc_vars()}
    }
    }
    fn dec_vars(&mut self) {
        if self.lowest() > 0 {
            match self {
                &mut Self::Var(ref mut id) => *id -= 1,
                &mut Self::Not(ref mut e) => e.dec_vars(),
                &mut Self::Xor(ref mut a, ref mut b) => {a.dec_vars(); b.dec_vars()},
                &mut Self::And(ref mut a, ref mut b) => {a.dec_vars(); b.dec_vars()},
                &mut Self::Or(ref mut a, ref mut b) => {a.dec_vars(); b.dec_vars()}
            }
        }
    }
    #[inline]
    pub fn vars(&self) -> usize {
        self.highest() as usize + 1
    }
    pub fn from_outputs<T: AsRef<[bool]>>(value: T) -> Self {
        use algo::*;
        use Expr::*;
        let slice = value.as_ref();
        assert!(slice.len().is_power_of_two(), "slice length must be in power of 2");
        assert!(slice.len() >= 2, "slice length must be 2 at minimum");
        match *slice {
            // 1 input, 2^2^1 cases
            [false, false] => raw_and(Var(0), raw_not(Var(0))),
            [ true, false] => Var(0),
            [false,  true] => raw_not(Var(0)),
            [ true,  true] => raw_or(Var(0), raw_not(Var(0))),
            // 2 inputs, 2^2^2 cases
            [false, false, false, false] => raw_and(Var(0), raw_not(Var(0))),
            [ true, false, false, false] => raw_not(raw_or(Var(0), Var(1))),
            [false,  true, false, false] => raw_and(Var(1), raw_not(Var(0))),
            [ true,  true, false, false] => Var(0),
            [false, false,  true, false] => raw_and(Var(0), raw_not(Var(1))),
            [ true, false,  true, false] => raw_not(Var(1)),
            [false,  true,  true, false] => raw_xor(Var(0), Var(1)),
            [ true,  true,  true, false] => raw_not(raw_and(Var(0), Var(1))),
            [false, false, false,  true] => raw_and(Var(0), Var(1)),
            [ true, false, false,  true] => raw_not(raw_xor(Var(0), Var(1))),
            [false,  true, false,  true] => Var(1),
            [ true,  true, false,  true] => raw_or(Var(1), raw_not(Var(0))),
            [false, false,  true,  true] => Var(0),
            [ true, false,  true,  true] => raw_or(Var(0), raw_not(Var(1))),
            [false,  true,  true,  true] => raw_or(Var(0), Var(1)),
            [ true,  true,  true,  true] => raw_or(Var(0), raw_not(Var(0))),

            // Rest: 3-256 inputs, (2^2^3)-(2^2^256) cases
            ref value => {
                let mut a = Self::from_outputs(&value[..(value.len() / 2)]);
                let mut b = Self::from_outputs(&value[(value.len() / 2)..]);
                a.inc_vars();
                b.inc_vars();
                (raw_not(Var(0)) & a) | (Var(0) & b)
            }
        }
    }
}
impl BitAnd for Expr {
    type Output = Self;
    fn bitand(mut self, mut rhs: Self) -> Self {
        algo::sort_mut(&mut self, &mut rhs);
        algo::and(self, rhs)
    }
}
impl BitOr for Expr {
    type Output = Self;
    fn bitor(mut self, mut rhs: Self) -> Self {
        algo::sort_mut(&mut self, &mut rhs);
        algo::or(self, rhs)
    }
}
impl BitXor for Expr {
    type Output = Self;
    fn bitxor(mut self, mut rhs: Self) -> Self {
        algo::sort_mut(&mut self, &mut rhs);
        algo::xor(self, rhs)
    }
}
impl Not for Expr {
    type Output = Self;
    #[inline]
    fn not(self) -> Self {
        algo::not(self)
    }
}
