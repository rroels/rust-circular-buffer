
use circular_buffer::CircularBuffer;

// NOTE: most unit tests are in lib.rs, so that tests can check the state of private fields

#[test]
// verify that all expected methods are publicly available (not private)
fn test_check_methods() {

    let mut buf = CircularBuffer::<u8>::new(4);
    buf.print();
    buf.write(1).unwrap();
    buf.is_empty();
    buf.is_full();
    buf.peek().unwrap();
    buf.read().unwrap();
    buf.size();
    buf.capacity();
    buf.write_many(&[1,2]).unwrap();
    buf.peek_many(2).unwrap();
    buf.read_many(2).unwrap();
    buf.clear();

}