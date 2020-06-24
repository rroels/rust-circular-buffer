![Rust](https://github.com/rroels/rust-circular-buffer/workflows/Rust/badge.svg?branch=master)

# circular_buffer
## Warning
This library was created as a "hello world" project for me to learn the Rust programming language. If is not the most efficient, and it should not be used in a production environment. **The buffer is not even thread-safe!** Instead, please consider these more widely used alternatives:

- https://docs.rs/queues/1.1.0/queues/struct.CircularBuffer.html
- https://docs.rs/circular-queue/0.2.5/circular_queue/
- https://docs.rs/ringbuf/0.2.1/ringbuf/

## Description
This library provides a basic fixed-size circular buffer implementation for the Rust programming language. Internally it works as described here:
https://en.wikipedia.org/wiki/Circular_buffer

However, this implementation does not overwrite the eldest results when it has reached its capacity. Instead it returns a [Result<T, E>](https://doc.rust-lang.org/std/result/) with an error when the buffer is full. Even though this essentially makes it act like a fixed-size queue, it still works like a circular buffer internally. One of the benefits, for instance, is that it uses a consecutive block of memory, and none of it is ever reallocated/copied/shifted when elements are removed (see Wikipedia article above).

What does it offer:
- FIFO queue-like fixed-size data structure
- fixed linear memory usage
- generics allow you to put objects of any type in the buffer
- efficient writing/reading of single elements
- slightly less efficient writing/reading of groups of elements
- peeking at single or multiple elements without removing them
- no unsafe code

What does it NOT offer:
- it is not thread-safe
- by design it does not overwrite old values when the buffer is full

## Example Usage

```
// create new buffer that can fit 10 elements of type u8
let mut buf = CircularBuffer::<u8>::new(10);

// write a single element and unwrap the Result<T, E>
buf.write(1).unwrap();

// write multiple elements and unwrap the Result<T, E>
buf.write_many(&[2, 3]).unwrap();

// read one element (removing it from the buffer) and unwrap the Result<T, E>
buf.read().unwrap() // returns '1'

// peek at one element (does NOT remove it) and unwrap the Result<T, E>
buf.peek().unwrap() // returns '2'

// peek at multiple elements (does NOT remove them) and unwrap the Result<T, E>
buf.peek_many().unwrap() // returns a vector with [2,3]

// read multiple elements (removing them from the buffer) and unwrap the Result<T, E>
buf.read_many(2).unwrap() // returns a vector with [2,3]

// returns true is the buffer is empty
buf.is_empty();

// returns true if the buffer is full
buf.is_full();

// empties the buffer, removing everything inside
buf.clear();

// return the amount of elements currently in the buffer
buf.size();

// return maximum amount of elements the buffer can hold
buf.capacity();
```

More examples of usage can be found in the unit tests.

## Feedback
As this is one of my first Rust projects, any feedback is always welcome! 
