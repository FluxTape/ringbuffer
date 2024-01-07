#[cfg(test)]
mod tests {
    use crate::RingBuffer;
    //use crate::iterators::iterators::*;

    #[test]
    fn get() {
        const SIZE: usize = 8;
        for offset in 0..SIZE {
            let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
            for i in 0..(SIZE+offset) as i32 {
                buf.put(i);
            }
            for i in 0..SIZE {
                //println!("{}:{}",i, buf.get(i as isize));
                assert_eq!(buf.get_oldest(i), (i+offset) as i32);
            }
            for i in 0..SIZE {
                assert_eq!(buf.get_oldest(i+SIZE), (i+offset) as i32);
            }
        }
    }

    #[test]
    fn get_reverse() {
        const SIZE: usize = 8;
        for offset in 0..SIZE {
            let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
            for i in 0..(SIZE+offset) as i32 {
                buf.put(i);
            }
            for i in 0..SIZE {
                //println!("{}:{}",i, buf.get_newest(i));
                assert_eq!(buf.get_newest(i), (SIZE+offset-1 - i) as i32);
            }
        }  
    }

    #[test]
    fn get_either() {
        const SIZE: usize = 8;
        for offset in 0..SIZE {
            let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
            for i in 0..(SIZE+offset) as i32 {
                buf.put(i);
            }
            for i in 0..SIZE as isize {
                //println!("{}:{}",i, buf.get(i));
                assert_eq!(buf.get(i), i as i32 + offset as i32);
            }
            for i in -(SIZE as isize)..0 {
                //println!("{}:{}",i, buf.get(i));
                assert_eq!(buf.get(i), ((SIZE+offset) as isize + i) as i32);
            }
        }
    }

    #[test]
    fn into_iterator() {
        const SIZE: usize = 8;
        for offset in 0..SIZE {
            let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
            for i in 0..(SIZE+offset) as i32 {
                buf.put(i);
            }
            for (i, x) in buf.into_iter().enumerate() {
                assert_eq!((i+offset) as i32, x);
            }
        }
    }

    #[test]
    fn into_back() {
        const SIZE: usize = 8;
        for offset in 0..SIZE {
            let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
            for i in 0..(SIZE+offset) as i32 {
                buf.put(i);
            }
            for (i, x) in buf.into_iter().rev().enumerate() {
                assert_eq!((SIZE+offset-1 - i) as i32, x);
            }
        } 
    }

    #[test]
    fn into_front_back() {
        const SIZE: usize = 8;
        let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
        for i in 0..SIZE as i32 {
            buf.put(i);
        }
        let mut iter = buf.into_iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(7));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), Some(6));
        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterator() {
        const SIZE: usize = 8;
        for offset in 0..SIZE {
            let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
            for i in 0..(SIZE+offset) as i32 {
                buf.put(i);
            }
            for (i, x) in buf.iter().enumerate() {
                assert_eq!((i+offset) as i32, x);
            }
            for (i, x) in buf.iter().enumerate() {
                assert_eq!((i+offset) as i32, x);
            }
        }  
    }

    #[test]
    fn iterator_back() {
        const SIZE: usize = 8;
        for offset in 0..SIZE {
            let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
            for i in 0..(SIZE+offset) as i32 {
                buf.put(i);
            }
            for (i, x) in buf.iter().rev().enumerate() {
                assert_eq!((SIZE+offset-1 - i) as i32, x);
            }
        }
    }

    #[test]
    fn iterator_front_back() {
        const SIZE: usize = 8;
        let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
        for i in 0..SIZE as i32 {
            buf.put(i);
        }
        let mut iter = buf.iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next_back(), Some(7));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), Some(6));
        assert_eq!(iter.next_back(), Some(5));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        const SIZE: usize = 8;
        for offset in 0..SIZE {
            let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
            for i in 0..(SIZE+offset+2) as i32 {
                buf.put(i);
            }
            for i in 0..SIZE {
                //println!("{}:{}",i, buf.get(i));
                assert_eq!(buf.get_oldest(i), (i+offset+2) as i32);
            }
            for (i, e) in buf.iter_mut().enumerate() {
                assert_eq!((i+offset+2) as i32, *e);
                *e -= 2;
            }
            for (i, e) in buf.iter().enumerate() {
                assert_eq!((i+offset) as i32, e);
            }
        }
    }
}