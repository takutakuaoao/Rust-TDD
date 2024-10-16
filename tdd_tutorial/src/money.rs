#[allow(dead_code)]
#[derive(PartialEq, Eq)]
struct Dollar {
    pub amount: u8,
}

impl Dollar {
    #[allow(dead_code)]
    fn times(&self, multiplier: u8) -> Dollar {
        return Dollar {
            amount: self.amount * multiplier,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::money::Dollar;
    #[test]
    fn multiplication() {
        let five = Dollar { amount: 5 };

        let mut product = five.times(2);
        assert_eq!(10, product.amount);

        product = five.times(3);
        assert_eq!(15, product.amount);
    }

    #[test]
    fn equality() {
        assert_eq!(Dollar { amount: 5 } == Dollar { amount: 5 }, true);
    }
}
