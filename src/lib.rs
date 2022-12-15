use rand_core::{ RngCore, OsRng, };
fn rng() -> [u8; 16] {
    let mut rng = OsRng::default();
    let mut rnds = [0x00; 16];
    rng.fill_bytes(&mut rnds);
    rnds
}
fn stringify(arr: [u8; 16]) -> String {
    format!(
        concat!(
            "{:02x}{:02x}{:02x}{:02x}",
            "-{:02x}{:02x}",
            "-{:02x}{:02x}",
            "-{:02x}{:02x}",
            "-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        ),
        arr[0], arr[1], arr[2], arr[3],
        arr[4], arr[5], arr[6], arr[7],
        arr[8], arr[9], arr[10], arr[11],
        arr[12], arr[13], arr[14], arr[15],
    )
}
pub fn uuidv4() -> String {
    let mut random = rng();
    // v4.4 spec
    random[6] = (random[6] & 0x0f) | 0x40;
    random[8] = (random[8] & 0x3f) | 0x80;
    stringify(random)
}
#[cfg(test)]
mod test {
    use crate::{
        rng,
        stringify,
        uuidv4,
    };
    #[test]
    fn rng_test() {
        for _ in 0..1000 {
            let first = rng();
            let second = rng();
            assert_ne!(first, second);
        }
    }
    #[test]
    fn stringify_test_1() {
        let x = [
            0x00u8, 0x01u8, 0x02u8, 0x03u8,
            0x04u8, 0x05u8, 0x06u8, 0x07u8,
            0x08u8, 0x09u8, 0x0au8, 0x0bu8,
            0x0cu8, 0x0du8, 0x0eu8, 0x0fu8,
        ];
        assert_eq!(
            "00010203-0405-0607-0809-0a0b0c0d0e0f",
            stringify(x),
        );
    }
    #[test]
    fn stringify_test_2() {
        let x = [
            0x10u8, 0x11u8, 0x12u8, 0x13u8,
            0x14u8, 0x15u8, 0x16u8, 0x17u8,
            0x18u8, 0x19u8, 0x1au8, 0x1bu8,
            0x1cu8, 0x1du8, 0x1eu8, 0x1fu8,
        ];
        assert_eq!(
            "10111213-1415-1617-1819-1a1b1c1d1e1f",
            stringify(x),
        );
    }
    #[test]
    fn uuidv4_test() {
        for _ in 0..1000 {
            let first = uuidv4();
            let second = uuidv4();
            assert_ne!(first, second);
        }
    }
}
