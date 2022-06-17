mod cmp;
mod ops;

pub use self::Expr::{Var, Const, Not, And, Or};
use std::cmp::{min, max};
use std::fmt;
use std::hint::unreachable_unchecked;

#[derive(Clone)]
pub enum Expr {
    #[non_exhaustive] Var(u8),
    Const(bool),
    Not(Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
}
impl Expr {
    fn low_var(&self) -> u8 {
        match self {
            Var(id) => *id,
            Const(_) => u8::MAX,
            Not(e) => e.low_var(),
            And(a, b) => min(a.low_var(), b.low_var()),
            Or(a, b) => min(a.low_var(), b.low_var())
        }
    }
    fn high_var(&self) -> u8 {
        match self {
            Var(id) => *id,
            Const(_) => u8::MIN,
            Not(e) => e.high_var(),
            And(a, b) => max(a.high_var(), b.high_var()),
            Or(a, b) => max(a.high_var(), b.high_var())
        }
    }
    fn precedence(&self) -> u16 {
        match self {
            Var(_) => 0,
            Const(_) => 0,
            Not(_) => 1,
            And(..) => 3,
            Or(..) => 4,
        }
    }
    fn inc_vars(&mut self) {
        match self {
            Var(id) => *id += 1, 
            Const(_) => (),
            Not(e) => e.inc_vars(),
            And(a, b) => {a.inc_vars(); b.inc_vars()},
            Or(a, b) => {a.inc_vars(); b.inc_vars()}
        }
    }
    fn dec_vars(&mut self) {
        if self.low_var() > 0 {
            match self {
                Var(id) => *id -= 1,
                Const(_) => (),
                Not(e) => e.dec_vars(),
                And(a, b) => {a.dec_vars(); b.dec_vars()},
                Or(a, b) => {a.dec_vars(); b.dec_vars()}
            }
        }
    }
    fn expr_count(&self) -> u8 {
        match self {
            Var(_) | Const(_) => 0,
            Not(_) => 1,
            _ => 2,
        }
    }
    unsafe fn one_expr(self) -> Self {
        match self {
            Not(e) => *e,
            _ => unreachable_unchecked()
        }
    }
    unsafe fn one_expr_ref(&self) -> &Self {
        match self {
            Not(e) => &**e,
            _ => unreachable_unchecked()
        }
    }
    unsafe fn two_expr(self) -> (Self, Self) {
        match self {
            And(a, b) => (*a, *b),
            Or(a, b) => (*a, *b),
            _ => unreachable_unchecked()
        }
    }
    unsafe fn two_expr_ref(&self) -> (&Self, &Self) {
        match self {
            And(a, b) => (&**a, &**b),
            Or(a, b) => (&**a, &**b),
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
                0 => match expr {
                    Var(id) => f.debug_tuple("Var").field(id).finish(),
                    Const(konst) => {
                        f.write_str("k")?;
                        f.write_str(if *konst {"1"} else {"0"})
                    }
                    // SAFETY: 100% sure this won't be executed
                    _ => unsafe { unreachable_unchecked() }
                },
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
                // SAFETY: It is clear that `symbol()` will only get executed if
                // `Expr::expr_count()` returns 2, thus `symbol()` requirements
                // are met.
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
        // SAFETY: Caller must ensure that `expr` has child `Expr`/s
        unsafe fn symbol(expr: &Expr) -> char {
            match expr {
                Var(_)
                    | Const(_) => unreachable_unchecked(),
                Not(_) => '!',
                And(..) => '&',
                Or(..) => '|'
            }
        }
        inner(self, f, false)
    }
}
