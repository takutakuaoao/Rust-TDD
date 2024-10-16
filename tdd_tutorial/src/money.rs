#[allow(dead_code)]
struct Dollar {
    pub amount: u8,
}

impl Dollar {
    #[allow(dead_code)]
    fn times(&mut self, multiplier: u8) {
        self.amount *= multiplier
    }
}

#[cfg(test)]
mod tests {
    use crate::money::Dollar;
    #[test]
    fn multiplication() {
        let mut five = Dollar { amount: 5 };
        five.times(2);
        assert_eq!(10, five.amount);
    }
}
