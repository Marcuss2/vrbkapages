use itertools::Itertools;
use std::iter::repeat_with;

#[derive(Clone)]
pub struct Belt<T: Sized, const SIZE: usize> {
    start: usize,
    belt: [T; SIZE],
}

impl<T: Sized + Default, const SIZE: usize> Default for Belt<T, SIZE> {
    fn default() -> Self {
        Self {
            start: 0,
            belt: repeat_with(|| T::default())
                .take(SIZE)
                .collect_array()
                .unwrap(),
        }
    }
}

impl<T: Sized + Default, const SIZE: usize> Belt<T, SIZE> {
    pub fn with_state(belt: [T; SIZE]) -> Self {
        Belt { start: 0, belt }
    }

    pub fn push_belt(&mut self, value: T) {
        if self.start == 0 {
            self.start = SIZE - 1;
        } else {
            self.start -= 1;
        }
        self.belt[self.start] = value;
    }

    pub fn peek_belt(&self, pos: usize) -> &T {
        let pos = (self.start + pos) % SIZE;
        &self.belt[pos]
    }
}
