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
            $t::<13>();
            $t::<14>();
            $t::<15>();
            $t::<16>();
            $t::<17>();
            $t::<31>();
            $t::<32>();
            $t::<33>();
            $t::<64>();
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
                assert_eq!(iter_mut.len(), SIZE);
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
    // checks expected wrong behaviour of get()
    fn get_either_wrong_idx() {
        fn get_big_mid(size: usize) -> u128 {
            (u128::MAX / 2) - (u128::MAX / 2) % (size as u128)
        }
        // assumes buf.head = 0
        fn get_correct_idx<const SIZE: usize>(idx: isize) -> usize {
            let big_mid = get_big_mid(SIZE);
            let new_idx = u128::wrapping_add_signed(big_mid, idx as i128);
            (new_idx % (SIZE as u128)) as usize
        }
        fn is_power_of_2(x: usize) -> bool {
            usize::count_ones(x) == 1
        }

        fn t<const SIZE: usize>() {
            if SIZE == 0 {
                return;
            }
            let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
            for i in 0..SIZE as i32 {
                buf.put(i);
            }
            assert_eq!(buf.head, 0);
            for i in 0..SIZE {
                assert_eq!(buf.buffer[i], i as i32)
            }
            if is_power_of_2(SIZE) {
                // correct if SIZE is a power of two
                assert_eq!(
                    buf.get(isize::MIN),
                    buf.buffer[get_correct_idx::<SIZE>(isize::MIN)]
                );
            } else {
                // wrong otherwise
                assert_ne!(
                    buf.get(isize::MIN),
                    buf.buffer[get_correct_idx::<SIZE>(isize::MIN)]
                );
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

    #[test]
    fn iter_mut_front_back() {
        const SIZE: usize = 8;
        let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
        for i in 0..SIZE as i32 {
            buf.put(i);
        }
        let mut iter = buf.iter_mut();
        assert_eq!(iter.next(), Some(&mut 0));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next_back(), Some(&mut 7));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next_back(), Some(&mut 6));
        assert_eq!(iter.next_back(), Some(&mut 5));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 4));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn from_iter() {
        const SIZE1: usize = 7;
        const SIZE2: usize = 9;
        const SIZE3: usize = 3;

        let mut buf1: RingBuffer<i32, SIZE1> = RingBuffer::default();
        for i in 0..SIZE1 as i32 {
            buf1.put(i);
        }
        for (i, x) in buf1.iter().enumerate() {
            assert_eq!(i as i32, x);
        }
        let buf2: RingBuffer<i32, SIZE2> = RingBuffer::from_iter(buf1);
        assert_eq!(buf2.buffer, [0, 1, 2, 3, 4, 5, 6, 0, 0]);
        assert_eq!(buf2.get_oldest(0), 0);
        for i in 0..SIZE1 {
            assert_eq!(buf2.get_newest(i), (SIZE1 - 1 - i) as i32);
        }
        for i in SIZE1..SIZE2 {
            assert_eq!(buf2.get_newest(i), i32::default());
        }

        let buf3: RingBuffer<i32, SIZE3> = RingBuffer::from_iter(buf2);
        assert_eq!(buf3.get_newest(0), 6);
        assert_eq!(buf3.get_newest(1), 5);
        assert_eq!(buf3.get_newest(2), 4);
    }
}
