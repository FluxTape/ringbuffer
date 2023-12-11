#[cfg(test)]
mod tests {
    use crate::RingBuffer;
    //use crate::iterators::iterators::*;

    #[test]
    fn get() {
        const SIZE: usize = 8;
        let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
        for i in 0..SIZE as i32 {
            buf.put(i);
        }
        for i in 0..SIZE {
            //println!("{}:{}",i, buf.get(i));
            assert_eq!(buf.get_oldest(i), i as i32);
        }
        for i in 0..SIZE {
            assert_eq!(buf.get_oldest(i+SIZE), i as i32);
        }
    }

    #[test]
    fn get_reverse() {
        const SIZE: usize = 8;
        let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
        for i in 0..SIZE as i32 {
            buf.put(i);
        }
        for i in 0..SIZE {
            //println!("{}:{}",i, buf.get_reverse(i));
            assert_eq!(buf.get_newest(i), (SIZE-1 - i) as i32);
        }
    }

    #[test]
    fn get_either() {
        const SIZE: usize = 8;
        let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
        for i in 0..SIZE as i32 {
            buf.put(i);
        }
        for i in 0..SIZE as isize {
            //println!("{}:{}",i, buf.get(i));
            assert_eq!(buf.get(i), i as i32);
        }
        for i in -(SIZE as isize)..0 {
            //println!("{}:{}",i, buf.get(i));
            assert_eq!(buf.get(i), (SIZE as isize + i) as i32);
        }
    }

    #[test]
    fn iterator() {
        const SIZE: usize = 8;
        let mut buf: RingBuffer<i32, SIZE> = RingBuffer::default();
        for i in 0..SIZE as i32 {
            buf.put(i);
        }
        for (i, x) in buf.into_iter().enumerate() {
            assert_eq!(i as i32, x)
        }
    }
}