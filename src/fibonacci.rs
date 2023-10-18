use num::{CheckedAdd, FromPrimitive, Integer};

#[derive(Clone)]
pub struct LimitedFibonacci<T: Copy + Integer + FromPrimitive + CheckedAdd> {
    current: T,
    next: T,
}

impl<T: Copy + Integer + FromPrimitive + CheckedAdd> Default for LimitedFibonacci<T> {
    fn default() -> Self {
        LimitedFibonacci {
            current: T::zero(),
            next: T::one(),
        }
    }
}

impl<T: Copy + Integer + FromPrimitive + CheckedAdd> Iterator for LimitedFibonacci<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.current.checked_add(&self.next)?;
        self.current = self.next;
        self.next = new_next;
        Some(self.current)
    }
}
