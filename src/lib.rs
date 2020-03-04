pub trait Unique<T> {
    fn unique<P>(&mut self, pred: P) -> Option<T>
    where
        P: FnMut(&T) -> bool;
}

impl<T, I> Unique<T> for I
where
    I: Iterator<Item = T>,
{
    fn unique<P>(&mut self, mut pred: P) -> Option<T>
    where
        P: FnMut(&T) -> bool,
    {
        let mut val = self.filter(|x| pred(x));
        let value = val.next();
        let value_next = val.next();
        match value_next {
            Some(_) => None,
            None => value,
        }
    }
}
