pub trait Printable {
    fn dbg(&self);
}

impl<T> Printable for T
where
    T: std::fmt::Debug,
{
    fn dbg(&self) {
        dbg!(self);
    }
}
