# iterators/sample_stream

Reference implementation. Read after attempting the exercise.

```rust
pub struct SampleStream<I> {
    source: I,
    step: usize,
    first: bool,
    done: bool,
}
impl<I: Iterator> SampleStream<I> {
    pub fn new(source: I, step: std::num::NonZeroUsize) -> Self {
        Self {
            source,
            step: step.get(),
            first: true,
            done: false,
        }
    }
}
impl<I: Iterator> Iterator for SampleStream<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        if !self.first {
            for _ in 1..self.step {
                if self.source.next().is_none() {
                    self.done = true;
                    return None;
                }
            }
        }
        self.first = false;
        let next = self.source.next();
        self.done = next.is_none();
        next
    }
}
```
