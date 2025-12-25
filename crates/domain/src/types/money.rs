/// Money represents a monetary amount, in euros.
#[derive(Debug, PartialEq)]
pub struct Money {
    cents: i64,
}

impl Money {
    pub fn from_cents(cents: i64) -> Self {
        Self { cents }
    }

    pub fn from_euros(euros: i64) -> Self {
        Self::from_cents(euros * 100)
    }

    pub fn cents(&self) -> i64 {
        self.cents
    }

    pub fn is_negative(&self) -> bool {
        self.cents < 0
    }
}
