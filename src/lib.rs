mod cmp;
mod ops;

use std::cmp::{min, max};
use std::fmt;
use std::hint::unreachable_unchecked;

#[derive(Clone)]
pub enum Expr {
    #[non_exhaustive] Var(u8),
    Not(Box<Self>),
    Xor(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
}
impl Expr {
    fn low_var(&self) -> u8 {
        match self {
            &Self::Var(id) => id,
            Self::Not(ref e) => e.low_var(),
            Self::Xor(ref a, ref b) => min(a.low_var(), b.low_var()),
            Self::And(ref a, ref b) => min(a.low_var(), b.low_var()),
            Self::Or(ref a, ref b) => min(a.low_var(), b.low_var())
        }
    }
    fn high_var(&self) -> u8 {
        match self {
            &Self::Var(id) => id,
            Self::Not(ref e) => e.high_var(),
            Self::Xor(ref a, ref b) => max(a.high_var(), b.high_var()),
            Self::And(ref a, ref b) => max(a.high_var(), b.high_var()),
            Self::Or(ref a, ref b) => max(a.high_var(), b.high_var())
        }
    }
    fn precedence(&self) -> u16 {
        match self {
            &Self::Var(id) => id as u16,
            Self::Not(_) => u8::MAX as u16 + 1,
            Self::Xor(..) => u8::MAX as u16 + 2,
            Self::And(..) => u8::MAX as u16 + 3,
            Self::Or(..) => u8::MAX as u16 + 4,
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
    fn expr_count(&self) -> u8 {
        match self {
            Self::Var(_) => 0,
            Self::Not(_) => 1,
            _ => 2,
        }
    }
    unsafe fn one_expr(self) -> Self {
        match self {
            Self::Not(e) => *e,
            _ => unreachable_unchecked()
        }
    }
    unsafe fn one_expr_ref(&self) -> &Self {
        match self {
            Self::Not(ref e) => &**e,
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
    unsafe fn two_expr_ref(&self) -> (&Self, &Self) {
        match self {
            Self::Xor(ref a, ref b) => (&**a, &**b),
            Self::And(ref a, ref b) => (&**a, &**b),
            Self::Or(ref a, ref b) => (&**a, &**b),
            _ => unreachable_unchecked()
        }
    }
    #[inline]
    pub fn vars(&self) -> usize {
        self.high_var() as usize + 1
    }
}
impl fmt::Debug for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn inner(expr: &Expr, f: &mut fmt::Formatter<'_>, paren: bool) -> fmt::Result {
            match expr.expr_count() {
                0 => if let Expr::Var(ref id) = expr {
                    write!(f, "{id}")
                } else {Ok(())},
                1 => unsafe {
                    f.write_str("!")?;
                    if paren {
                        f.write_str("(")?;
                    }
                    inner(expr.one_expr_ref(), f, true)?;
                    if paren {
                        f.write_str("(")
                    } else {
                        Ok(())
                    }
                },
                _ => unsafe {
                    let (a, b) = expr.two_expr_ref();
                    if paren {
                        f.write_str("(")?;
                    }
                    inner(a, f, true)?;
                    write!(f, " {} ", symbol(expr))?;
                    inner(b, f, true)?;
                    if paren {
                        f.write_str("(")
                    } else {
                        Ok(())
                    } 
                }
            }
        }
        unsafe fn symbol(expr: &Expr) -> char {
            match expr {
                Expr::Var(_) => std::hint::unreachable_unchecked(),
                Expr::Not(_) => '!',
                Expr::Xor(..) => '^',
                Expr::And(..) => '&',
                Expr::Or(..) => '|'
            }
        }
        inner(self, f, false)
    }
}
