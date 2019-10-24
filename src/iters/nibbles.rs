pub struct Nibbles<I>
    where
        I: Iterator<Item = u8>,
{
    byte_iter: I,
    remainder: u8,
    bits_left: u8,
}

impl<I> Nibbles<I> where I: Iterator<Item=u8>
{
    pub fn new(iter: I) -> Self {
        Self {
            byte_iter: iter,
            remainder: 0,
            bits_left: 0,
        }
    }
}

impl<I> Iterator for Nibbles<I>
    where
        I: Iterator<Item = u8>,
{
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        let mut quire = self.remainder as u16;
        if self.bits_left < 4 {
            if let Some(b) = self.byte_iter.next() {
                quire |= (b as u16) << self.bits_left;
                self.bits_left += 8;
            } else {
                return None;
            }
        }
        self.remainder = (quire >> 4) as u8;
        self.bits_left -= 4;
        Some((quire & 0b00000000_00001111) as u8)
    }
}

#[test]
fn test_nibbles() {
    assert_eq!(
        Nibbles::new([0x21, 0x43, 0x65, 0x87, 0xa9, 0xcb, 0xed, 0x0f].iter().cloned()).collect::<Vec<u8>>(),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0]
    );
}
