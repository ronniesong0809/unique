pub trait Unique<T> {
    fn unique<P>(&mut self, pred: P) -> Option<T>
    where
        P: FnMut(&T) -> bool;
}

impl<T, I> Unique<T> for I
where
    I: Iterator<Item = T>,
{
    fn unique<P>(&mut self, _pred: P) -> Option<T> {
        None
    }
}
