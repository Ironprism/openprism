#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Pippo {
    field_a: u32,
}

impl core::ops::Add<Self> for Pippo {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            field_a: self.field_a + rhs.field_a,
        }
    }
}