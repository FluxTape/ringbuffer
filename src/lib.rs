pub mod iterators;
mod test;

#[derive(Debug, Clone)]
pub struct RingBuffer<T, const N: usize> {
    buffer: [T; N],
    head: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    #[inline(always)]
    fn wrap_idx(idx: usize) -> usize {
        idx % N
    }

    pub fn put(&mut self, item: T) -> T {
        use std::mem::replace;
        let old = replace(&mut self.buffer[self.head], item);
        self.head = Self::wrap_idx(self.head + 1);
        old
    }
}

impl<T, const N: usize> RingBuffer<T, N>
where
    T: Default,
{
    pub fn get_owned(&mut self, idx: isize) -> T {
        use std::mem::take;
        let new_idx = usize::wrapping_add_signed(self.head, idx);
        take(&mut self.buffer[Self::wrap_idx(new_idx)])
    }
}

impl<T, const N: usize> Default for RingBuffer<T, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        RingBuffer {
            buffer: [T::default(); N],
            head: 0,
        }
    }
}

impl<T, const N: usize> RingBuffer<T, N>
where
    T: Copy,
{
    const MID: usize = if N != 0 {
        (usize::MAX / 2) - (usize::MAX / 2) % N
    } else {
        0
    };
    const UPPER: usize = if N != 0 {
        usize::MAX - N - (usize::MAX % N)
    } else {
        0
    };

    pub const fn new(init_value: T) -> Self {
        RingBuffer {
            buffer: [init_value; N],
            head: 0,
        }
    }

    pub fn get_oldest(&self, idx: usize) -> T {
        let wrapped_idx = Self::wrap_idx(idx + self.head);
        self.buffer[wrapped_idx]
    }

    pub fn get_newest(&self, idx: usize) -> T {
        let wrapped_idx = Self::wrap_idx(Self::UPPER + self.head - 1 - idx);
        self.buffer[wrapped_idx]
    }

    pub fn get(&self, idx: isize) -> T {
        // may result in wrong index if idx is near isize::MIN and N is not a power of 2
        let new_idx = usize::wrapping_add_signed(self.head + Self::MID, idx);
        self.buffer[Self::wrap_idx(new_idx)]
    }
}
