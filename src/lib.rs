use rand_core::{ RngCore, OsRng, };
pub trait UUID {
    fn from_uuid_bytes(bytes: [u8; 16]) -> Self;
}
impl UUID for String {
    fn from_uuid_bytes(bytes: [u8; 16]) -> Self {
        format!(
            concat!(
                "{:02x}{:02x}{:02x}{:02x}",
                "-{:02x}{:02x}",
                "-{:02x}{:02x}",
                "-{:02x}{:02x}",
                "-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            ),
            bytes[0], bytes[1], bytes[2], bytes[3],
            bytes[4], bytes[5], bytes[6], bytes[7],
            bytes[8], bytes[9], bytes[10], bytes[11],
            bytes[12], bytes[13], bytes[14], bytes[15],
        )
    }
}
impl UUID for [u8; 16] {
    fn from_uuid_bytes(bytes: [u8; 16]) -> Self {
        bytes
    }
}
fn rng() -> [u8; 16] {
    let mut rng = OsRng::default();
    let mut rnds = [0x00; 16];
    rng.fill_bytes(&mut rnds);
    rnds
}
pub fn uuidv4<Type>() -> Type
where
    Type: UUID,
{
    let mut random = rng();
    // v4.4 spec
    random[6] = (random[6] & 0x0f) | 0x40;
    random[8] = (random[8] & 0x3f) | 0x80;
    Type::from_uuid_bytes(random)
}
#[cfg(test)]
mod test {
    use crate::{
        rng,
        uuidv4,
        UUID,
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
    fn string_test_1() {
        let x = [
            0x00u8, 0x01u8, 0x02u8, 0x03u8,
            0x04u8, 0x05u8, 0x06u8, 0x07u8,
            0x08u8, 0x09u8, 0x0au8, 0x0bu8,
            0x0cu8, 0x0du8, 0x0eu8, 0x0fu8,
        ];
        assert_eq!(
            "00010203-0405-0607-0809-0a0b0c0d0e0f",
            String::from_uuid_bytes(x),
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
            String::from_uuid_bytes(x),
        );
    }
    #[test]
    fn uuidv4_test() {
        for _ in 0..1000 {
            let first = uuidv4::<String>();
            let second = uuidv4::<String>();
            assert_ne!(first, second);
        }
    }
    #[test]
    fn uuidv4_bytes_test() {
        for _ in 0..1000 {
            let first = uuidv4::<[u8; 16]>();
            let second = uuidv4::<[u8; 16]>();
            assert_ne!(first, second);
        }
    }
}
