use num::{FromPrimitive, Integer};

pub struct Digits<T> {
    remainder: T,
    radix: T,
}

impl<T: Integer + FromPrimitive> Digits<T> {
    pub fn new(n: T) -> Digits<T> {
        let ten = FromPrimitive::from_u8(10).unwrap();
        Digits{remainder: n, radix: ten}
    }
}

impl<T: Integer> Iterator for Digits<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if !self.remainder.is_zero() {
            let (rem, digit) = self.remainder.div_rem(&self.radix);
            self.remainder = rem;
            Some(digit)
        } else {
            None
        }
    }
}
