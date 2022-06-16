use crate::Expr::{self, *};
use std::cmp::{
    Eq,
    Ordering,
    PartialEq,
    PartialOrd
};


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
