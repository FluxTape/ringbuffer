pub mod iterators {
    use crate::RingBuffer;

    impl<T, const N:usize> IntoIterator for RingBuffer<T, N> 
    where T: Copy
    {
        type Item = T;
        type IntoIter = RingBufferIntoIter<T, N>;
        fn into_iter(self) -> Self::IntoIter {
            RingBufferIntoIter {
                ringbuffer: self,
                index: 0,
            }
        }
    }

    pub struct RingBufferIntoIter<T, const N: usize>
    where T: Copy
    {
        ringbuffer: RingBuffer<T, N>,
        index: usize
    }

    impl<T, const N:usize> Iterator for RingBufferIntoIter<T, N>
    where T: Copy 
    {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            if self.index >= N {return None}
            let result = self.ringbuffer.get_oldest(self.index);
            self.index += 1;
            Some(result)
        }
    }

    // --------------- non consuming iter
    impl<'a, T, const N:usize> IntoIterator for &'a RingBuffer<T, N> 
    where T: Copy
    {
        type Item = T;
        type IntoIter = RingBufferIter<'a, T, N>;
        fn into_iter(self) -> Self::IntoIter {
            RingBufferIter {
                ringbuffer: self,
                index: 0,
            }
        }
    }

    pub struct RingBufferIter<'a, T, const N: usize>
    where T: Copy
    {
        ringbuffer: &'a RingBuffer<T, N>,
        index: usize
    }

    impl<'a, T, const N:usize> Iterator for RingBufferIter<'a, T, N>
    where T: Copy 
    {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            if self.index >= N {return None}
            let result = self.ringbuffer.get_oldest(self.index);
            self.index += 1;
            Some(result)
        }
    }

    impl<T, const N:usize> RingBuffer<T, N>
    where T: Copy 
    {
        pub fn iter<'a>(&'a self) -> RingBufferIter<'a, T, N> {
            RingBufferIter {
                ringbuffer: &self,
                index: 0
            }
        }
    }


}