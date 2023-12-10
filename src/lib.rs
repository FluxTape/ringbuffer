mod test;

pub struct RingBuffer<T, const N: usize> 
where T: Copy
{
    buffer: [T; N],
    head: usize, 
} 

impl<T, const N:usize> RingBuffer<T, N> 
where T: Copy
{
    #[inline(always)]
    fn wrap_idx(idx: usize) -> usize {
        idx % N
    }

    pub fn new(init_value: T) -> Self {
        RingBuffer {
            buffer: [init_value; N],
            head: 0,
        }
    }

    pub fn put(&mut self, item: T) -> T {
        use std::mem::replace;
        self.head = Self::wrap_idx(self.head+1);
        replace(&mut self.buffer[self.head], item)
    }

    pub fn get_oldest(&self, idx: usize) -> T {
        let current_idx = Self::wrap_idx(idx + self.head +1);
        self.buffer[current_idx]
    }

    pub fn get_newest(&self, idx: usize) -> T {
        let idx_wrapped = Self::wrap_idx(idx + self.head + N-1);
        self.buffer[N-1-idx_wrapped]
    }

    pub fn get(&self, idx: isize) -> T {
        let new_idx = usize::wrapping_add_signed(self.head +1, idx);
        self.buffer[Self::wrap_idx(new_idx)]
    }
}
