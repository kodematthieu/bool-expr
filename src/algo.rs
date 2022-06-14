use crate::expr::Expr::{self, *};

fn prec(expr: &Expr) -> u16 {
    u16::MAX - match expr {
        &Var(id) => 4 + id as u16,
        &Not(_) => 3,
        &Xor(..) => 2,
        &And(..) => 1,
        &Or(..) => 0
    }
}

pub fn sort(lhs: Expr, rhs: Expr) -> (Expr, Expr) {
    let a = prec(&lhs);
    let b = prec(&rhs);
    if a > b {
        (rhs, lhs)
    } else {
        (lhs, rhs)
    }
}
pub fn sort_mut(lhs: &mut Expr, rhs: &mut Expr) {
    if prec(lhs) > prec(rhs) {
        std::mem::swap(lhs, rhs);
    }
}

pub fn raw_not(expr: Expr) -> Expr {
    Not(expr.into())
}
pub fn not(expr: Expr) -> Expr {
    match expr {
        Not(expr) => *expr,
        expr => raw_not(expr)
    }
}
pub fn raw_xor(lhs: Expr, rhs: Expr) -> Expr {
    Xor(lhs.into(), rhs.into())
}
pub fn xor(lhs: Expr, rhs: Expr) -> Expr {
    match (lhs, rhs) {
        (a, b) => raw_xor(a, b)
    }
}
pub fn raw_and(lhs: Expr, rhs: Expr) -> Expr {
    And(lhs.into(), rhs.into())
}
pub fn and(lhs: Expr, rhs: Expr) -> Expr {
    match (lhs, rhs) {
        (a, b) if a == b => a,
        (Not(a), Not(b)) => not(and(*a, *b)),
        (Not(c), Or(a, b)) if matches!(*c, And(ref c, ref d) if &a == c && &b == d) => xor(*a, *b),
        (a, b) => raw_and(a, b)
    }
}
pub fn raw_or(lhs: Expr, rhs: Expr) -> Expr {
    Or(lhs.into(), rhs.into())
}
pub fn or(lhs: Expr, rhs: Expr) -> Expr {
    match (lhs, rhs) {
        (a, b) if a == b => a,
        (Not(c), And(a, b)) if matches!(*c, Or(ref c, ref d) if &a == c && &b == d) => not(xor(*a, *b)),
        (a, b) => raw_or(a, b)
    }
}
