use num::{pow, FromPrimitive, Integer, ToPrimitive};

pub struct Digits<T> {
    remainder: T,
    radix: T,
    high: T,
}

impl<T: Integer + FromPrimitive + ToPrimitive + Clone> Digits<T> {
    pub fn new(n: T) -> Digits<T> {
        Digits::with_radix(n, 10)
    }

    fn with_radix(n: T, radix: u8) -> Digits<T> {
        let p = n.to_f64().unwrap().log(radix as f64).floor() as usize;
        let radix: T = FromPrimitive::from_u8(radix).unwrap();
        let high = pow(radix.clone(), p);
        Digits { remainder: n, radix: radix, high: high }

    }
}

impl<T: Integer> Digits<T> {
    fn decrement_high(&mut self) {
        let high = mem::replace(&mut self.high, T::zero());
        self.high = high / &self.radix;
    }
}

impl<T: Integer> Iterator for Digits<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if !self.remainder.is_zero() {
            let (rem, digit) = self.remainder.div_rem(&self.radix);
            self.remainder = rem;
            self.high = self.high / &self.radix;
            Some(digit)
        } else {
            None
        }
    }
}

impl<T: Integer> DoubleEndedIterator for Digits<T> {
    fn next_back(&mut self) -> Option<T> {
        None
    }
}
