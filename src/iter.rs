pub trait FirstLast: Iterator {
    fn first_last(self) -> Option<(Self::Item, Self::Item)>;
}
impl <I> FirstLast for I
where I: Iterator, I::Item: Clone {
    fn first_last(mut self) -> Option<(I::Item, I::Item)>
    where I::Item: Clone {
        if let Some(first) = self.next() {
            let mut last = first.clone();
            for next in self {
                last = next;
            }
            Some((first, last))
        }
        else {
            None
        }
    }
}
