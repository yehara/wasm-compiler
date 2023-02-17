pub fn i32_to_leb128(num: i32) -> Vec<u8> {
    let mut cur = num;
    let mut res = Vec::<u8>::new();
    while cur<= -127 || cur >= 127 {
        res.push (0x80 +  ((cur & 0x7f) as u8));
        cur >>= 7;
    }
    res.push((cur & 0x7f) as u8);
    res
}

pub fn usize_to_leb128(num: usize) -> Vec<u8> {
    let mut cur = num;
    let mut res = Vec::<u8>::new();
    while cur >= 127 {
        res.push (0x80 +  ((cur & 0x7f) as u8));
        cur >>= 7;
    }
    res.push((cur & 0x7f) as u8);
    res
}

#[test]
fn test_i32() {
    assert_eq!(i32_to_leb128(0), vec![0x00]);
    assert_eq!(i32_to_leb128(126), vec![0x7e]);
    assert_eq!(i32_to_leb128(127), vec![0xff, 0x00]);
    assert_eq!(i32_to_leb128(128), vec![0x80, 0x01]);
    assert_eq!(i32_to_leb128(-1), vec![0x7f]);
    assert_eq!(i32_to_leb128(2147483647), vec![0xff, 0xff, 0xff, 0xff, 0x07]);
    assert_eq!(i32_to_leb128(-127), vec![0x81, 0x7f]);
    assert_eq!(i32_to_leb128(-128), vec![0x80, 0x7f]);
    assert_eq!(i32_to_leb128(-129), vec![0xff, 0x7e]);
    assert_eq!(i32_to_leb128(-123456), vec![0xc0, 0xbb, 0x78]);
    assert_eq!(i32_to_leb128(-2147483648), vec![0x80, 0x80, 0x80, 0x80, 0x78]);
}

#[test]
fn test_usize() {
    assert_eq!(usize_to_leb128(0), vec![0x00]);
    assert_eq!(usize_to_leb128(126), vec![0x7e]);
    assert_eq!(usize_to_leb128(127), vec![0xff, 0x00]);
    assert_eq!(usize_to_leb128(128), vec![0x80, 0x01]);
    assert_eq!(usize_to_leb128(2147483647), vec![0xff, 0xff, 0xff, 0xff, 0x07]);
}

