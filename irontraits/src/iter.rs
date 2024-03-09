pub trait Iter {
    type Item;

    type Iter<'a>: Iterator<Item = &'a Self::Item>
    where
        Self: 'a;

    fn iter(&self) -> Self::Iter<'_>;
}

pub trait IterMut: Iter {
    type IterMut<'a>: Iterator<Item = &'a mut Self::Item>
    where
        Self: 'a;

    fn iter_mut(&mut self) -> Self::IterMut<'_>;
}
