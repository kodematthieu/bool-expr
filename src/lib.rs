mod cmp;
mod ops;

use std::hint::unreachable_unchecked;

#[derive(Clone, Debug)]
pub enum Expr {
    #[non_exhaustive] Var(u8),
    Not(Box<Self>),
    Xor(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
}
impl Expr {
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
        if self.low_var() > 0 {
            match self {
                &mut Self::Var(ref mut id) => *id -= 1,
                &mut Self::Not(ref mut e) => e.dec_vars(),
                &mut Self::Xor(ref mut a, ref mut b) => {a.dec_vars(); b.dec_vars()},
                &mut Self::And(ref mut a, ref mut b) => {a.dec_vars(); b.dec_vars()},
                &mut Self::Or(ref mut a, ref mut b) => {a.dec_vars(); b.dec_vars()}
            }
        }
    }
    unsafe fn one_expr(self) -> Self {
        match self {
            Self::Not(e) => *e,
            _ => unreachable_unchecked()
        }
    }
    unsafe fn two_expr(self) -> (Self, Self) {
        match self {
            Self::Xor(a, b) => (*a, *b),
            Self::And(a, b) => (*a, *b),
            Self::Or(a, b) => (*a, *b),
            _ => unreachable_unchecked()
        }
    }
    #[inline]
    pub fn vars(&self) -> usize {
        self.high_var() as usize + 1
    }
}
