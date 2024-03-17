pub fn square(x: u32) -> u32 {
    x * x
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Pippo {
    field_a: u32,
}

impl Pippo {
    pub fn new(field_a: u32) -> Self {
        Self { field_a }
    }

    pub fn field_a(&self) -> u32 {
        self.field_a
    }

    pub fn print(&self) {
        println!("Pippo: field_a={}", self.field_a);
    }
}

impl core::ops::Add<Self> for Pippo {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            field_a: self.field_a + rhs.field_a,
        }
    }
}

impl core::ops::Index<usize> for Pippo {
    type Output = u32;
    fn index(&self, _idx: usize) -> &Self::Output {
        &self.field_a
    }
}

impl core::ops::Index<u8> for Pippo {
    type Output = u32;
    fn index(&self, _idx: u8) -> &Self::Output {
        &self.field_a
    }
}