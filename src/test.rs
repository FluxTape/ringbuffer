#[cfg(test)]
mod tests {
    use crate::RingBuffer;
    //use crate::iterators::iterators::*;

    #[macro_export]
    macro_rules! test_variants {
        (  $t:ident ) => {
            $t::<0>();
            $t::<1>();
            $t::<2>();
            $t::<3>();
            $t::<4>();
            $t::<5>();
            $t::<6>();
            $t::<7>();
            $t::<8>();
            $t::<9>();
            $t::<10>();
            $t::<11>();
            $t::<12>();
        };
    }

    #[test]
    fn size() {
        fn t<const SIZE: usize>() {
            //dbg!(SIZE);
            let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
            {
                let iter = buf.iter();
                assert_eq!(iter.size_hint(), (SIZE, Some(SIZE)));
                assert_eq!(iter.len(), SIZE);
            }
            {
                let iter_mut = buf.iter_mut();
                assert_eq!(iter_mut.size_hint(), (SIZE, Some(SIZE)));
                // no exact size iter for iter_mut
            }
            {
                let into_iter = buf.into_iter();
                assert_eq!(into_iter.size_hint(), (SIZE, Some(SIZE)));
                assert_eq!(into_iter.len(), SIZE);
            }
        }
        test_variants!(t);
    }

    #[test]
    fn get() {
        fn t<const SIZE: usize>() {
            for offset in 0..SIZE {
                let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
                for i in 0..(SIZE + offset) as i32 {
                    buf.put(i);
                }
                for i in 0..SIZE {
                    //println!("{}:{}",i, buf.get(i as isize));
                    assert_eq!(buf.get_oldest(i), (i + offset) as i32);
                }
                for i in 0..SIZE {
                    assert_eq!(buf.get_oldest(i + SIZE), (i + offset) as i32);
                }
            }
        }
        test_variants!(t);
    }

    #[test]
    fn get_reverse() {
        fn t<const SIZE: usize>() {
            //dbg!(SIZE);
            for offset in 0..SIZE {
                //dbg!(offset);
                let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
                for i in 0..(SIZE + offset) as i32 {
                    buf.put(i);
                }
                //dbg!(&buf);
                for i in 0..SIZE {
                    //println!("{}:{}",i, buf.get_newest(i));
                    assert_eq!(buf.get_newest(i), (SIZE + offset - 1 - i) as i32);
                }
            }
        }
        test_variants!(t);
    }

    #[test]
    fn get_either() {
        fn t<const SIZE: usize>() {
            //dbg!(SIZE);
            for offset in 0..SIZE {
                let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
                for i in 0..(SIZE + offset) as i32 {
                    buf.put(i);
                }
                for i in 0..SIZE as isize {
                    //println!("{}:{}",i, buf.get(i));
                    assert_eq!(buf.get(i), i as i32 + offset as i32);
                }
                for i in -(SIZE as isize)..0 {
                    //println!("{}:{}",i, buf.get(i));
                    assert_eq!(buf.get(i), ((SIZE + offset) as isize + i) as i32);
                }
            }
        }
        test_variants!(t);
    }

    #[test]
    fn into_iterator() {
        fn t<const SIZE: usize>() {
            for offset in 0..SIZE {
                let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
                for i in 0..(SIZE + offset) as i32 {
                    buf.put(i);
                }
                for (i, x) in buf.into_iter().enumerate() {
                    assert_eq!((i + offset) as i32, x);
                }
            }
        }
        test_variants!(t);
    }

    #[test]
    fn into_back() {
        fn t<const SIZE: usize>() {
            for offset in 0..SIZE {
                let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
                for i in 0..(SIZE + offset) as i32 {
                    buf.put(i);
                }
                for (i, x) in buf.into_iter().rev().enumerate() {
                    assert_eq!((SIZE + offset - 1 - i) as i32, x);
                }
            }
        }
        test_variants!(t);
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
        fn t<const SIZE: usize>() {
            for offset in 0..SIZE {
                let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
                for i in 0..(SIZE + offset) as i32 {
                    buf.put(i);
                }
                for (i, x) in buf.iter().enumerate() {
                    assert_eq!((i + offset) as i32, x);
                }
                for (i, x) in buf.iter().enumerate() {
                    assert_eq!((i + offset) as i32, x);
                }
            }
        }
        test_variants!(t);
    }

    #[test]
    fn iterator_back() {
        fn t<const SIZE: usize>() {
            for offset in 0..SIZE {
                let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
                for i in 0..(SIZE + offset) as i32 {
                    buf.put(i);
                }
                for (i, x) in buf.iter().rev().enumerate() {
                    assert_eq!((SIZE + offset - 1 - i) as i32, x);
                }
            }
        }
        test_variants!(t);
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
        fn t<const SIZE: usize>() {
            for offset in 0..SIZE {
                let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
                for i in 0..(SIZE + offset + 2) as i32 {
                    buf.put(i);
                }
                for i in 0..SIZE {
                    //println!("{}:{}",i, buf.get(i));
                    assert_eq!(buf.get_oldest(i), (i + offset + 2) as i32);
                }
                for (i, e) in buf.iter_mut().enumerate() {
                    assert_eq!((i + offset + 2) as i32, *e);
                    *e -= 2;
                }
                for (i, e) in buf.iter().enumerate() {
                    assert_eq!((i + offset) as i32, e);
                }
            }
        }
        test_variants!(t);
    }
}
