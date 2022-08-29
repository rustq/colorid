use rand::{
    rngs::{StdRng},
    Rng, SeedableRng,
};

pub const HEX_CHARS: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];

pub fn hex(number: u8) -> [char;2] {
    let low = number % 16;
    let high = (number - low) / 16;
    [HEX_CHARS[high as usize], HEX_CHARS[low as usize]]
}

pub fn format(count: usize, random: fn(usize) -> Vec<u8>) -> String {
    assert!(count > 0, "The count should at least greater than 0");

    let buffer: Vec<u8> = random(count * 3);
    let mut id: String = String::with_capacity(count * 7 - 1);

    for i in 0..count {
        let r = hex(buffer[i * 3]);
        let g = hex(buffer[i * 3 + 1]);
        let b = hex(buffer[i * 3 + 2]);
        id.push(r[0]);
        id.push(r[1]);
        id.push(g[0]);
        id.push(g[1]);
        id.push(b[0]);
        id.push(b[1]);;
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