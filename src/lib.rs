mod cmp;
mod ops;

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
    #[inline]
    pub fn vars(&self) -> usize {
        self.high_var() as usize + 1
    }
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
