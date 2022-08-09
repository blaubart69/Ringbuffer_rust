pub struct Ringbuffer<T> {
    buf : Vec<T>,
    write_idx : usize,
    inserted : usize
}

impl<T> Ringbuffer<T> where T : Clone + Default {

    pub fn new(size : usize) -> Ringbuffer<T> {
        
        let mut _buf : Vec<T> = Vec::with_capacity(size);
        _buf.resize(size, Default::default());
        
        Ringbuffer {
           buf : _buf,
           write_idx : 0,
           inserted : 0
        }
    }

    pub fn push(&mut self, val : T) {

        if self.write_idx == self.buf.len() {
            self.write_idx = 0;
        }

        self.buf[self.write_idx] = val;

        self.write_idx += 1;
        self.inserted  += 1;

    }
}
pub struct RingbufferIter<'a, T> {
    idx  : usize,
    items_to_yield : usize,
    ring  : &'a Ringbuffer<T>
}

impl<'a, T> IntoIterator for &'a Ringbuffer<T> {
    type Item     = &'a T;
    type IntoIter = RingbufferIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {

        RingbufferIter {
            idx : self.write_idx,
            ring : self,
            items_to_yield : if self.inserted < self.buf.len() {
                self.inserted
            }
            else {
                self.buf.len()
            },
        }        
    }
}

impl<'a, T> Iterator for RingbufferIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {

        if self.items_to_yield == 0 {
            None
        }
        else {
            self.items_to_yield -= 1;

            if self.idx == 0 {
                self.idx = self.ring.buf.len();
            }

            self.idx -= 1;

            self.ring.buf.get(self.idx)
        }
    }
}

#[cfg(test)]
mod tests {

use super::Ringbuffer;

    #[test]
    fn empty() {
        let mut rb : Ringbuffer::<i32> = Ringbuffer::new(17);       
        let mut iter = rb.into_iter();
        assert!(iter.next().is_none());
    }

    #[test]
    fn one_item() {
        let mut rb : Ringbuffer::<i32> = Ringbuffer::new(1);       
        rb.push(18);
        let mut iter = rb.into_iter();
        assert_eq!(18, *iter.next().unwrap());
        assert!(iter.next().is_none());
    }


    #[test]
    fn two_items() {
        let mut rb : Ringbuffer::<i32> = Ringbuffer::new(2);       
        rb.push(18);
        rb.push(19);
        let mut iter = rb.into_iter();
        assert_eq!(19, *iter.next().unwrap());
        assert_eq!(18, *iter.next().unwrap());
        assert!(iter.next().is_none());
    }    
    #[test]
    fn two_items_overflow() {
        let mut rb : Ringbuffer::<i32> = Ringbuffer::new(2);       
        rb.push(18);
        rb.push(19);
        rb.push(20);
        let mut iter = rb.into_iter();
        assert_eq!(20, *iter.next().unwrap());
        assert_eq!(19, *iter.next().unwrap());
        assert!(iter.next().is_none());
    }    
    #[test]
    fn push_20_items_with_buffersize_10() {
        let mut rb : Ringbuffer::<i32> = Ringbuffer::new(10);
        for i in 1..21 {       
            rb.push(i);
        }
        
        let mut i = 20;
        for v in rb.into_iter() {
            assert_eq!(i,*v);
            i -= 1;
        }
    }
    #[test]
    fn push_20_items_with_buffersize_50() {
        let mut rb : Ringbuffer::<i32> = Ringbuffer::new(50);
        for i in 1..21 {       
            rb.push(i);
        }
        
        let mut i = 20;
        for v in rb.into_iter() {
            assert_eq!(i,*v);
            i -= 1;
        }
    }
    #[test]
    fn push_20_items_get_last_10_with_buffersize_50() {
        let mut rb : Ringbuffer::<i32> = Ringbuffer::new(50);
        for i in 1..21 {       
            rb.push(i);
        }
        
        let mut i = 20;
        for v in rb.into_iter().take(10) {
            assert_eq!(i,*v);
            i -= 1;
        }
    }

}