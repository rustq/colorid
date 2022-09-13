pub mod rng;

mod hex;

pub fn default() -> String {
    let buffer: [u8; 12] = rng::default();
    let mut id: [u8; 31] = [hex::DASH; 31];
    for i in 0..4 {
        let r = hex::hex(buffer[i * 3]);
        let g = hex::hex(buffer[i * 3 + 1]);
        let b = hex::hex(buffer[i * 3 + 2]);
        id[i * 8] = hex::NUMBER_SIGN;
        id[i * 8 + 1] = r[0];
        id[i * 8 + 2] = r[1];
        id[i * 8 + 3] = g[0];
        id[i * 8 + 4] = g[1];
        id[i * 8 + 5] = b[0];
        id[i * 8 + 6] = b[1];
    }
    unsafe {
        std::str::from_utf8_unchecked_mut(&mut id).to_owned()
    }
}

pub fn colorid(count: usize) -> String {
    assert!(count > 0, "The count should at least greater than 0");

    let buffer: Vec<u8> = rng::generate(count * 3);
    let mut id: Vec<u8> = vec![hex::DASH; count * 8 - 1];
    for i in 0..count {
        let r = hex::hex(buffer[i * 3]);
        let g = hex::hex(buffer[i * 3 + 1]);
        let b = hex::hex(buffer[i * 3 + 2]);
        id[i * 8] = hex::NUMBER_SIGN;
        id[i * 8 + 1] = r[0];
        id[i * 8 + 2] = r[1];
        id[i * 8 + 3] = g[0];
        id[i * 8 + 4] = g[1];
        id[i * 8 + 5] = b[0];
        id[i * 8 + 6] = b[1];
    }
    unsafe {
        std::str::from_utf8_unchecked_mut(&mut id).to_owned()
    }
}

#[cfg(test)]
mod test_color {
    use super::*;

    #[test]
    fn test_colorid() {
        assert_eq!(colorid!().len(), 31);
        assert_eq!(colorid(5).len(), 39);
    }

    #[test]
    #[should_panic]
    fn test_invalid_colorid() {
        let _ = colorid(0);
    }
}

#[macro_export]
macro_rules! colorid {
    // simple
    () => {
        $crate::default()
    };

    // size
    ($size:expr) => {
        $crate::colorid($size)
    };
}