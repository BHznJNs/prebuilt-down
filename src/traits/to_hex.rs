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

#[cfg(test)]
mod tests {
    use super::ToHex;

    #[test]
    fn to_hex_outputs_lowercase_hex() {
        let data = [0x00, 0x0f, 0x10, 0xab, 0xff];
        assert_eq!(data.to_hex(), "000f10abff");
    }
}
