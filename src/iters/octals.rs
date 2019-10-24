pub struct Octals<I>
    where
        I: Iterator<Item = u8>,
{
    byte_iter: I,
    remainder: u8,
    bits_left: u8,
}

impl<I> Octals<I> where I: Iterator<Item=u8>
{
    pub fn new(iter: I) -> Self {
        Self {
            byte_iter: iter,
            remainder: 0,
            bits_left: 0,
        }
    }
}

impl<I> Iterator for Octals<I>
    where
        I: Iterator<Item = u8>,
{
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        let mut quire = self.remainder as u16;
        if self.bits_left < 3 {
            if let Some(b) = self.byte_iter.next() {
                quire |= (b as u16) << self.bits_left;
                self.bits_left += 8;
            } else if self.bits_left > 0 {
                self.bits_left = 0;
                return Some(self.remainder);
            } else {
                return None;
            }
        }
        self.remainder = (quire >> 3) as u8;
        self.bits_left -= 3;
        Some((quire & 0b00000000_00000111) as u8)
    }
}

#[test]
fn test_octals() {
    assert_eq!(
        Octals::new([0b01001001, 0b10010010, 0b00100100].iter().cloned()).collect::<Vec<u8>>(),
        vec![1, 1, 1, 1, 1, 1, 1, 1]
    );

    assert_eq!(
        Octals::new([0b11010001, 0b01011000, 0b00011111].iter().cloned()).collect::<Vec<u8>>(),
        vec![1, 2, 3, 4, 5, 6, 7, 0]
    );

    assert_eq!(
        Octals::new("a".bytes()).collect::<Vec<u8>>(),
        vec![1, 4, 1]
    );

    assert_eq!(
        Octals::new("aa".bytes()).collect::<Vec<u8>>(),
        vec![1, 4, 5, 0, 6, 0]
    );
    assert_eq!(
        Octals::new("au".bytes()).collect::<Vec<u8>>(),
        vec![1, 4, 5, 2, 7, 0]
    );
    assert_eq!(
        Octals::new("aab".bytes()).collect::<Vec<u8>>(),
        vec![1, 4, 5, 0, 6, 4, 0, 3]
    );
    assert_eq!(
        Octals::new("aaa".bytes()).collect::<Vec<u8>>(),
        vec![1, 4, 5, 0, 6, 2, 0, 3]
    );
}

#[derive(Copy, Clone, Debug)]
pub struct OctalPrinter{
    stored: u8,
    bits_got: u8,
}

impl OctalPrinter {
    pub fn new() -> Self {
        Self {
            stored: 0,
            bits_got: 0,
        }
    }
    pub fn add(&mut self, input: u8) -> Option<u8> {
        let mut quire = (input as u16) << self.bits_got;
        self.bits_got += 3;
        quire |= self.stored as u16;
        if self.bits_got >= 8 {
            self.stored = (quire >> 8) as u8;
            self.bits_got -= 8;
            Some((0x00FF & quire) as u8)
        } else {
            self.stored = (0x00FF & quire) as u8;
            None
        }
    }
}

#[test]
fn test_octal_printer() {
    let mut printer = OctalPrinter::new();
    assert_eq!(printer.add(1), None);
    assert_eq!(printer.add(1), None);
    assert_eq!(printer.add(1), Some(0b01001001));
    assert_eq!(printer.add(1), None);
    assert_eq!(printer.add(1), None);
    assert_eq!(printer.add(1), Some(0b10010010));
    assert_eq!(printer.add(1), None);
    assert_eq!(printer.add(1), Some(0b00100100));

    assert_eq!(printer.add(1), None);
    assert_eq!(printer.add(2), None);
    assert_eq!(printer.add(3), Some(0b11010001));
    assert_eq!(printer.add(4), None);
    assert_eq!(printer.add(5), None);
    assert_eq!(printer.add(6), Some(0b01011000));
    assert_eq!(printer.add(7), None);
    assert_eq!(printer.add(0), Some(0b00011111));
}

#[test]
fn test_octal_roundtrip() {
    let test_string = "test TEST 123 \0 hoge \r\n hello! ";
    let mut printer = OctalPrinter::new();
    let mut result = Vec::new();
    for o in Octals::new(test_string.bytes()) {
        if let Some(b) = printer.add(o) {
            result.push(b);
        }
    }
    assert_eq!(result, test_string.as_bytes());
}