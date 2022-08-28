use hex;
use rand::{
    rngs::{StdRng},
    Rng, SeedableRng,
};


pub fn colorid(size: usize) -> String {
    let mut rng = StdRng::from_entropy();
    let mut buffer: Vec<u8> = vec![0; size * 3];

    rng.fill(&mut buffer[..]);

    let result: String = {
        let mut color_string: String = "".to_owned();
        for i in 0..size {
            let color = hex::encode_upper(&buffer[i*3..i*3+3]);
            color_string.push_str(&color);
            if i != size - 1 {
                color_string.push_str("-");
            }
        }
        color_string
    };

    result
}

#[cfg(test)]
mod test_color {
    use super::*;

    #[test]
    fn size_len() {
        assert_eq!(colorid!().len(), 27);
        assert_eq!(colorid(5).len(), 34);
        assert_eq!(colorid(0).len(), 0);
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