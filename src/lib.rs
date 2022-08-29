use hex;
use rand::{
    rngs::{StdRng},
    Rng, SeedableRng,
};

pub fn format(count: usize, random: fn(usize) -> Vec<u8>) -> String {
    assert!(count > 0, "The count should at least greater than 0");

    let buffer: Vec<u8> = random(count * 3);
    let mut id: String = String::with_capacity(count * 7 - 1);

    for i in 0..count {
        let color = hex::encode_upper(&buffer[i*3..i*3+3]);
        id.push_str(&color);
        if i + 1 < count {
            id.push('-');
        }
    }
    id
}

pub fn default_random(size: usize) -> Vec<u8> {
    let mut rng = StdRng::from_entropy();
    let mut buffer: Vec<u8> = vec![0; size];

    rng.fill(&mut buffer[..]);
    buffer
}


pub fn colorid(count: usize) -> String {
    format(count, default_random)
}

#[cfg(test)]
mod test_color {
    use super::*;

    #[test]
    fn test_colorid() {
        assert_eq!(colorid!().len(), 27);
        assert_eq!(colorid(5).len(), 34);
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
        $crate::colorid(4)
    };

    // size
    ($size:expr) => {
        $crate::colorid($size)
    };
}