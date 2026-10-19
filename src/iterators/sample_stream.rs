// cargo test --lib iterators::sample_stream::

//! Build an iterator adapter yielding positions 0, step, 2*step, ... from its source.
//! Consume lazily: the first next call consumes only the first source item, and each subsequent next
//! consumes at most step more source items. Do not require Clone or collect the source.
//! Once the source returns None, the adapter remains exhausted even if the source is not fused.

pub struct SampleStream<I> {
    source: I,
    step: usize,
    first: bool,
    done: bool,
}
impl<I: Iterator> SampleStream<I> {
    pub fn new(source: I, step: std::num::NonZeroUsize) -> Self {
        todo!()
    }
}
impl<I: Iterator> Iterator for SampleStream<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;
    #[test]
    fn positions() {
        assert_eq!(
            SampleStream::new(0..8, NonZeroUsize::new(3).unwrap()).collect::<Vec<_>>(),
            [0, 3, 6]
        );
    }
    #[test]
    fn lazy_consumption() {
        let calls = std::cell::Cell::new(0);
        let source = (0..100).inspect(|_| calls.set(calls.get() + 1));
        let mut s = SampleStream::new(source, NonZeroUsize::new(4).unwrap());
        assert_eq!(s.next(), Some(0));
        assert_eq!(calls.get(), 1);
        assert_eq!(s.next(), Some(4));
        assert_eq!(calls.get(), 5);
    }
    #[test]
    fn exhausted_stays_exhausted() {
        let mut values = vec![Some(1), None, Some(2)].into_iter();
        let source = std::iter::from_fn(move || values.next().flatten());
        let mut s = SampleStream::new(source, NonZeroUsize::new(1).unwrap());
        assert_eq!(s.next(), Some(1));
        assert_eq!(s.next(), None);
        assert_eq!(s.next(), None);
    }
}
