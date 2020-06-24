
use core::{mem};

pub struct CircularBuffer<T> {
    // maximum amount of elements the buffer can hold
    capacity: usize,
    // buffer that holds the actual data
    buffer: Box<[T]>,
    // index of where the data starts in the buffer (the "head")
    index_start: usize,
    // non-inclusive index of where the data stops in the buffer (the "tail")
    index_next_free: usize,
    // keep track of amount of elements currently stored, it also makes things easier to understand in the internal code
    // also solves the problem that if index_start == index_next_free, we don't know if it's full or empty
    size: usize
}

impl<T: Default + Clone + ToString> CircularBuffer<T> {

    pub fn new(capacity: usize) -> Self {

        let result = Self {
            capacity,
            buffer: vec!(T::default(); capacity).into_boxed_slice(),
            index_start: 0,
            index_next_free: 0,
            size: 0
        };

        result
    }

    pub fn write(&mut self, value: T) -> Result<(), &'static str>{
        if !self.is_full() {
            self.buffer[self.index_next_free] = value;
            self.index_next_free = self.increase_index(self.index_next_free);
            self.size += 1;
            Ok(())
        } else {
            Err("CircularBuffer is full")
        }
    }

    pub fn write_many(&mut self, values: &[T]) -> Result<(), &'static str>{

        if values.len() > self.capacity - self.size() {
            return Err("CircularBuffer does not have enough space for the provided elements");
        }

        for element in values {
            self.write(element.clone())?;
        }

        Ok(())

    }

    pub fn read(&mut self) -> Result<T, &'static str> {
        if !self.is_empty() {
            let result = mem::replace(&mut self.buffer[self.index_start], T::default());
            self.index_start = self.increase_index(self.index_start);
            self.size = self.size - 1;
            Ok(result)
        } else {
            Err("CircularBuffer is empty")
        }
    }

    pub fn read_many(&mut self, amount: usize) -> Result<Vec<T>, &'static str> {

        if amount > self.size() {
            return Err("CircularBuffer does not contain the amount of requested elements");
        }

        let mut vec: Vec<T> = Vec::with_capacity(amount);
        for _ in 0..amount {
            vec.push(self.read()?);
        }

        Ok(vec)
    }

    pub fn peek(&self) -> Result<&T, &'static str> {
        if !self.is_empty() {
            Ok(&self.buffer[self.index_start])
        } else {
            Err("CircularBuffer is empty")
        }
    }

    pub fn peek_many(&self, amount: usize) -> Result<Vec<T>, &'static str> {

        if amount > self.size() {
            return Err("CircularBuffer does not contain the amount of requested elements");
        }

        let mut vec: Vec<T> = Vec::with_capacity(amount);
        let mut index = self.index_start;
        for _ in 0..amount {
            vec.push(self.buffer[index].clone());
            index = self.increase_index(index);
        }

        Ok(vec)
    }

    // returns the amount of elements currently inside the buffer
    // not to be confused with capacity, which is the *maximum* amount of elements that the buffer can hold
    pub fn size(&self) -> usize {
        return self.size;
    }

    // returns the *maximum* amount of elements that the buffer can hold
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    // if the start index equals the next_free index, then the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn is_full(&self) -> bool {
        self.size == self.capacity
    }


    pub fn clear(&mut self) {
        // we read the rest of the buffer, to ensure the remaining elements are dropped from memory properly
        while !self.is_empty() {
            self.read().unwrap();
        }
    }

    pub fn print(&self) {
        println!("{}", self.to_string());
    }

    // private function that increases the index, overflowing if we're going beyond the capacity
    // this function does not check if the increase makes sense, it's the responsibility of the caller!
    fn increase_index(&self, index: usize) -> usize {
        if index == self.capacity - 1 {
            0
        } else {
            index + 1
        }
    }

}

impl<T: Default + Clone + ToString> ToString for CircularBuffer<T> {
    fn to_string(&self) -> String {
        let mut output: String = String::from("[");

        //is_wrapping: bool = self.index_next_free < self.index_start;
        for i in 0 .. self.capacity {
            if i >= self.index_start && i < self.index_next_free {
                output += self.buffer[i].to_string().as_str();
            } else {
                output += "_";
            }
            if i != self.capacity-1 {
                output += ",";
            }
        }
        output += "]";
        output
    }
}


#[cfg(test)]
mod tests {

    use crate::CircularBuffer;
    use rand::distributions::{Distribution, Uniform};
    use std::ops::Deref;

    #[test]
    fn test_basic_write() {
        let mut buf = CircularBuffer::<u8>::new(4);

        let write_result = buf.write(1);
        assert!(write_result.is_ok());
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.size(), 1);

        assert_eq!(buf.index_start, 0);
        assert_eq!(buf.index_next_free, 1);

        assert_eq!(buf.buffer[0], 1);

    }

    #[test]
    fn test_basic_read() {
        let mut buf = CircularBuffer::<u8>::new(4);

        let write_result = buf.write(1);
        assert!(write_result.is_ok());
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.size(), 1);

        let read_result = buf.read();
        assert!(read_result.is_ok());
        assert!(buf.is_empty());
        assert_eq!(buf.size(), 0);
        assert_eq!(read_result.unwrap(), 1);

    }

    #[test]
    fn test_basic_peek() {
        let mut buf = CircularBuffer::<u8>::new(4);

        let write_result = buf.write(1);
        assert!(write_result.is_ok());
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.size(), 1);

        let peek_result = buf.peek();
        assert!(peek_result.is_ok());
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.size(), 1);
        assert_eq!(*peek_result.unwrap(), 1);

        let read_result = buf.read();
        assert!(read_result.is_ok());
        assert!(buf.is_empty());
        assert_eq!(buf.size(), 0);
        assert_eq!(read_result.unwrap(), 1);

    }

    #[test]
    fn test_basic_peek_many() {

        let mut buf = CircularBuffer::<u8>::new(4);
        buf.write_many(&[1,2,3]).unwrap();

        // can't read 4
        let mut peek_result = buf.peek_many(4);
        assert!(peek_result.is_err());
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.size(), 3);

        peek_result = buf.peek_many(2);
        assert!(peek_result.is_ok());
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.size(), 3);

        let values = peek_result.unwrap();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], 1);
        assert_eq!(values[1], 2);

    }

    #[test]
    fn test_basic_write_many() {
        let mut buf = CircularBuffer::<u8>::new(4);

        let mut write_result = buf.write_many(&[1,2,3]);
        assert!(write_result.is_ok());
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.size(), 3);

        let mut read_result = buf.read();
        assert!(read_result.is_ok());
        assert_eq!(buf.size(), 2);
        assert_eq!(read_result.unwrap(), 1);

        read_result = buf.read();
        assert!(read_result.is_ok());
        assert_eq!(buf.size(), 1);
        assert_eq!(read_result.unwrap(), 2);

        read_result = buf.read();
        assert!(read_result.is_ok());
        assert_eq!(buf.size(), 0);
        assert_eq!(read_result.unwrap(), 3);

        assert!(buf.is_empty());

        // buffer is now empty, try to add more that allowed
        write_result = buf.write_many(&[1,2,3,4,5]);
        assert!(write_result.is_err());
        assert!(buf.is_empty());
        assert_eq!(buf.size(), 0);

    }

    #[test]
    fn test_basic_read_many() {
        let mut buf = CircularBuffer::<u8>::new(4);
        buf.write_many(&[1,2,3]).unwrap();

        // can't read 4
        let mut read_result = buf.read_many(4);
        assert!(read_result.is_err());
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.size(), 3);

        read_result = buf.read_many(2);
        assert!(read_result.is_ok());
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.size(), 1);
        let boxed_values = read_result.unwrap();
        let values = boxed_values.deref();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], 1);
        assert_eq!(values[1], 2);
    }

    #[test]
    fn test_basic_clear() {
        let mut buf = CircularBuffer::<u8>::new(4);

        let write_result = buf.write_many(&[1,2,3]);
        assert!(write_result.is_ok());
        assert_eq!(buf.is_empty(), false);
        assert_eq!(buf.size(), 3);

        buf.clear();
        assert!(buf.is_empty());
        assert_eq!(buf.size(), 0);

    }

    #[test]
    fn test_usage_single_elements() {

        // repeatedly write X elements, remove X elements, with peeks before every read
        // the amount of write/reads is randomised every iteration
        // this cause the buffer to "wrap around" multiple times in different ways

        let loop_count = 1000;
        let capacity: u8 = 4;

        let mut buf = CircularBuffer::<u8>::new(capacity.into());
        let mut rng = rand::thread_rng();
        let random_range = Uniform::from(1..capacity+1);

        for _ in 0..loop_count {
            let read_write_amount = random_range.sample(&mut rng);
            let write_value = random_range.sample(&mut rng);
            for _ in 0..read_write_amount {
                assert_eq!(buf.is_full(), false);
                buf.write(write_value).unwrap();
            }
            for _ in 0..read_write_amount {
                let peek_value = buf.peek().unwrap();
                assert_eq!(*peek_value, write_value);
                let read_value = buf.read().unwrap();
                assert_eq!(read_value, write_value);
            }
        }

    }

    #[test]
    fn test_usage_many_elements() {

        // repeatedly write X elements, remove X elements, with peeks before every read
        // the amount of write/reads is randomised every iteration
        // this cause the buffer to "wrap around" multiple times in different ways

        let loop_count = 1000;
        let capacity: u8 = 4;

        let mut buf = CircularBuffer::<u8>::new(capacity.into());
        let mut rng = rand::thread_rng();
        let random_range = Uniform::from(1..capacity+1);

        for _ in 0..loop_count {
            let read_write_amount = random_range.sample(&mut rng);
            let write_value = random_range.sample(&mut rng);

            assert_eq!(buf.is_full(), false);
            let data_vec = vec![write_value; read_write_amount as usize];
            let data = data_vec.as_slice();
            buf.write_many(data.as_ref()).unwrap();

            let peek_values = buf.peek_many(read_write_amount as usize).unwrap();
            for i in 0..peek_values.len() {
                assert_eq!(peek_values[i], write_value);
            }

            let read_result = buf.read_many(read_write_amount as usize).unwrap();
            let values = read_result.deref();
            assert_eq!(values.len(), read_write_amount as usize);
            assert_eq!(values, data);


        }

    }


}
