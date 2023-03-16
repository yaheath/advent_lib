use lazy_static::lazy_static;

lazy_static! {
    static ref ONEBITS: Vec<usize> = {
        let mut v = Vec::with_capacity(256);
        v.push(0);
        for i in 1..256 {
            let n = (i & 1) + v[i / 2];
            v.push(n);
        }
        v
    };
}

pub fn one_bits_u8(arg: u8) -> usize {
    ONEBITS[arg as usize]
}

pub fn one_bits_u16(arg: u16) -> usize {
    let mut sum = 0;
    for b in arg.to_ne_bytes() {
        sum += ONEBITS[b as usize];
    }
    sum
}

pub fn one_bits_u32(arg: u32) -> usize {
    let mut sum = 0;
    for b in arg.to_ne_bytes() {
        sum += ONEBITS[b as usize];
    }
    sum
}

pub fn one_bits_u64(arg: u64) -> usize {
    let mut sum = 0;
    for b in arg.to_ne_bytes() {
        sum += ONEBITS[b as usize];
    }
    sum
}

pub fn one_bits_u128(arg: u128) -> usize {
    let mut sum = 0;
    for b in arg.to_ne_bytes() {
        sum += ONEBITS[b as usize];
    }
    sum
}
