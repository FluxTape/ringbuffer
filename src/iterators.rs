pub mod into_iter {
    use crate::RingBuffer;
    use std::iter::FusedIterator;

    impl<T, const N: usize> IntoIterator for RingBuffer<T, N>
    where
        T: Copy,
    {
        type Item = T;
        type IntoIter = RingBufferIntoIter<T, N>;
        fn into_iter(self) -> Self::IntoIter {
            RingBufferIntoIter {
                ringbuffer: self,
                index_forward: 0,
                index_backward: N,
            }
        }
    }

    pub struct RingBufferIntoIter<T, const N: usize>
    where
        T: Copy,
    {
        ringbuffer: RingBuffer<T, N>,
        index_forward: usize,
        index_backward: usize,
    }

    impl<T, const N: usize> Iterator for RingBufferIntoIter<T, N>
    where
        T: Copy,
    {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            if self.index_forward >= self.index_backward {
                return None;
            }
            let result = self.ringbuffer.get_oldest(self.index_forward);
            self.index_forward += 1;
            Some(result)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (N, Some(N))
        }
    }

    impl<T, const N: usize> DoubleEndedIterator for RingBufferIntoIter<T, N>
    where
        T: Copy,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index_backward <= self.index_forward {
                return None;
            }
            self.index_backward -= 1;
            let result = self.ringbuffer.get_oldest(self.index_backward);
            Some(result)
        }
    }

    impl<T, const N: usize> FusedIterator for RingBufferIntoIter<T, N> where T: Copy {}

    impl<T, const N: usize> ExactSizeIterator for RingBufferIntoIter<T, N> where T: Copy {}
}

pub mod iter {
    use crate::RingBuffer;
    use std::iter::FusedIterator;

    // --------------- non consuming iter
    impl<'a, T, const N: usize> IntoIterator for &'a RingBuffer<T, N>
    where
        T: Copy,
    {
        type Item = T;
        type IntoIter = RingBufferIter<'a, T, N>;
        fn into_iter(self) -> Self::IntoIter {
            RingBufferIter {
                ringbuffer: self,
                index_forward: 0,
                index_backward: N,
            }
        }
    }

    pub struct RingBufferIter<'a, T, const N: usize>
    where
        T: Copy,
    {
        ringbuffer: &'a RingBuffer<T, N>,
        index_forward: usize,
        index_backward: usize,
    }

    impl<'a, T, const N: usize> Iterator for RingBufferIter<'a, T, N>
    where
        T: Copy,
    {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            if self.index_forward >= self.index_backward {
                return None;
            }
            let result = self.ringbuffer.get_oldest(self.index_forward);
            self.index_forward += 1;
            Some(result)
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (N, Some(N))
        }
    }

    impl<T, const N: usize> DoubleEndedIterator for RingBufferIter<'_, T, N>
    where
        T: Copy,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            if self.index_backward <= self.index_forward {
                return None;
            }
            self.index_backward -= 1;
            let result = self.ringbuffer.get_oldest(self.index_backward);
            Some(result)
        }
    }

    impl<T, const N: usize> RingBuffer<T, N>
    where
        T: Copy,
    {
        pub fn iter(&self) -> RingBufferIter<'_, T, N> {
            RingBufferIter {
                ringbuffer: self,
                index_forward: 0,
                index_backward: N,
            }
        }
    }

    impl<T, const N: usize> FusedIterator for RingBufferIter<'_, T, N> where T: Copy {}

    impl<T, const N: usize> ExactSizeIterator for RingBufferIter<'_, T, N> where T: Copy {}
}

pub mod iter_mut {
    use crate::RingBuffer;
    use std::iter::Chain;
    use std::iter::FusedIterator;
    use std::slice::IterMut;

    // --------------- non consuming iter
    pub struct RingBufferIterMut<'a, T, const N: usize>
    where
        T: Copy,
    {
        iter: Chain<IterMut<'a, T>, IterMut<'a, T>>,
    }

    impl<'a, T, const N: usize> RingBufferIterMut<'a, T, N>
    where
        T: Copy,
    {
        pub fn new(buf: &'a mut RingBuffer<T, N>) -> Self {
            let (l, r) = buf.buffer.split_at_mut(buf.head);
            let iter = r.as_mut().iter_mut().chain(l.as_mut().iter_mut());
            RingBufferIterMut { iter }
        }
    }

    impl<'a, T, const N: usize> IntoIterator for &'a mut RingBuffer<T, N>
    where
        T: Copy,
    {
        type Item = &'a mut T;
        type IntoIter = RingBufferIterMut<'a, T, N>;
        fn into_iter(self) -> Self::IntoIter {
            RingBufferIterMut::new(self)
        }
    }

    impl<'a, T, const N: usize> Iterator for RingBufferIterMut<'a, T, N>
    where
        T: Copy,
    {
        type Item = &'a mut T;

        fn next(&mut self) -> Option<&'a mut T> {
            self.iter.next()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (N, Some(N))
        }
    }

    impl<T, const N: usize> DoubleEndedIterator for RingBufferIterMut<'_, T, N>
    where
        T: Copy,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.iter.next_back()
        }
    }

    impl<T, const N: usize> RingBuffer<T, N>
    where
        T: Copy,
    {
        pub fn iter_mut(&mut self) -> RingBufferIterMut<'_, T, N> {
            RingBufferIterMut::new(self)
        }
    }

    impl<T, const N: usize> FusedIterator for RingBufferIterMut<'_, T, N> where T: Copy {}

    impl<T, const N: usize> ExactSizeIterator for RingBufferIterMut<'_, T, N> where T: Copy {}
}
