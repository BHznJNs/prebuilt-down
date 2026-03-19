use std::fmt::Write;

pub trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for [u8] {
    /// convert a byte array into a hex string, should output a lowercased string
    fn to_hex(&self) -> String {
        let mut s = String::with_capacity(self.len() * 2);
        for &b in self {
            write!(&mut s, "{:02x}", b).unwrap();
        }
        return s;
    }
}
