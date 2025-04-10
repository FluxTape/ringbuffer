pub mod into_iter {
    use crate::RingBuffer;
    use core::iter::FusedIterator;

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

        fn next(&mut self) -> Option<Self::Item> {
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

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.index_forward += n;
            self.next()
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

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            self.index_backward -= n;
            self.next_back()
        }
    }

    impl<T, const N: usize> FusedIterator for RingBufferIntoIter<T, N> where T: Copy {}

    impl<T, const N: usize> ExactSizeIterator for RingBufferIntoIter<T, N> where T: Copy {}
}

pub mod iter {
    use crate::RingBuffer;
    use core::iter::FusedIterator;

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

    impl<T, const N: usize> Iterator for RingBufferIter<'_, T, N>
    where
        T: Copy,
    {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
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

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.index_forward += n;
            self.next()
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

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            self.index_backward -= n;
            self.next_back()
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
    use core::iter::Chain;
    use core::iter::FusedIterator;
    use core::slice::IterMut;

    // --------------- non consuming iter
    pub struct RingBufferIterMut<'a, T: Copy, const N: usize>(
        Chain<IterMut<'a, T>, IterMut<'a, T>>,
    );

    impl<'a, T, const N: usize> RingBufferIterMut<'a, T, N>
    where
        T: Copy,
    {
        pub fn new(buf: &'a mut RingBuffer<T, N>) -> Self {
            let (l, r) = buf.buffer.split_at_mut(buf.head);
            let iter = r.as_mut().iter_mut().chain(l.as_mut().iter_mut());
            RingBufferIterMut(iter)
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

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }

        fn size_hint(&self) -> (usize, Option<usize>) {
            (N, Some(N))
        }

        fn nth(&mut self, n: usize) -> Option<Self::Item> {
            self.0.nth(n)
        }
    }

    impl<T, const N: usize> DoubleEndedIterator for RingBufferIterMut<'_, T, N>
    where
        T: Copy,
    {
        fn next_back(&mut self) -> Option<Self::Item> {
            self.0.next_back()
        }

        fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
            self.0.nth_back(n)
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

mod from_iter {
    use crate::RingBuffer;

    impl<T, const N: usize> FromIterator<T> for RingBuffer<T, N>
    where
        T: Copy + Default,
    {
        fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
            let mut new_buf: RingBuffer<T, N> = RingBuffer::default();
            let iterator = iter.into_iter();
            let (min, _) = iterator.size_hint();
            let to_skip = usize::saturating_sub(min, N);
            for item in iterator.skip(to_skip) {
                new_buf.put(item);
            }
            new_buf
        }
    }
}
