#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
struct Dollar {
    amount: u8,
}

impl Dollar {
    #[allow(dead_code)]
    fn times(&self, multiplier: u8) -> Dollar {
        return Self {
            amount: self.amount * multiplier,
        };
    }
}

#[allow(dead_code)]
#[derive(PartialEq, Eq, Debug)]
struct Franc {
    amount: u8,
}

impl Franc {
    #[allow(dead_code)]
    fn times(&self, multiplier: u8) -> Franc {
        return Self {
            amount: self.amount * multiplier,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::money::{Dollar, Franc};
    #[test]
    fn multiplication() {
        let five = Dollar { amount: 5 };

        assert_eq!(Dollar { amount: 10 }, five.times(2));
        assert_eq!(Dollar { amount: 15 }, five.times(3));
    }

    #[test]
    fn equality() {
        assert_eq!(Dollar { amount: 5 }, Dollar { amount: 5 });
    }

    #[test]
    fn franc_multiplication() {
        let five = Franc { amount: 5 };

        assert_eq!(Franc { amount: 10 }, five.times(2));
        assert_eq!(Franc { amount: 15 }, five.times(3));
    }
}
