use crate::Expr::{self, *};
use std::cmp::{
    Eq,
    min,
    max,
    Ord,
    Ordering,
    PartialEq,
    PartialOrd
};

impl Expr { 
    pub(crate) fn low_var(&self) -> u8 {
        match self {  
            &Var(id) => id,   
            Not(ref e) => e.low_var(),   
            Xor(ref a, ref b) => min(a.low_var(), b.low_var()),
            And(ref a, ref b) => min(a.low_var(), b.low_var()),
            Or(ref a, ref b) => min(a.low_var(), b.low_var())
        }     
    }     
    pub(crate) fn high_var(&self) -> u8 {
        match self {
            &Var(id) => id,
            Not(ref e) => e.high_var(),
            Xor(ref a, ref b) => max(a.high_var(), b.high_var()),
            And(ref a, ref b) => max(a.high_var(), b.high_var()),
            Or(ref a, ref b) => max(a.high_var(), b.high_var())
        }
    }
    fn precedence(&self) -> u16 {
        match self {
            &Var(id) => id as u16,
            Not(_) => u8::MAX as u16 + 1,
            Xor(..) => u8::MAX as u16 + 2,
            And(..) => u8::MAX as u16 + 3,
            Or(..) => u8::MAX as u16 + 4,
        }
    }
}
impl Eq for Expr {}
impl PartialEq for Expr {
    fn eq(&self, rhs: &Self) -> bool {
        match (self, rhs) {
            (Var(a), Var(b)) => a == b,
            (Not(a), Not(b)) => a == b,
            (Xor(a, b), Xor(c, d)) => a == c && b == d,
            (And(a, b), And(c, d)) => a == c && b == d,
            (Or(a, b), Or(c, d)) => a == c && b == d,
            _ => false
        }
    }
}
impl PartialOrd for Expr {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        let a = self.precedence();
        let b = rhs.precedence();
        Some(if a == b {
            Ordering::Equal
        } else if a > b {
            Ordering::Greater
        } else {
            Ordering::Less
        })
    }
}
