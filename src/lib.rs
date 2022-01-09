pub fn to_gattaca(buf: Vec<u8>) -> Vec<u8> {
    let out_len = buf.len() * 4;
    let mut out: Vec<u8> = Vec::with_capacity(out_len);

    let map = | sub | {
        match sub {
            0 => return b'A',
            1 => return b'T',
            2 => return b'C',
            3 => return b'G',
            _ => panic!("bad value")
        }
    };

    for atom in buf {
        out.push(map((atom & 0xC0) >> 6));
        out.push(map((atom & 0x30) >> 4));
        out.push(map((atom & 0x0C) >> 2));
        out.push(map(atom & 0x03));
    }

    return out;
}

pub fn from_gattaca(buf: Vec<u8>) -> Vec<u8> {
    let out_len = buf.len() / 4;
    let mut out: Vec<u8> = Vec::with_capacity(out_len);

    let map = | sub | {
        match sub {
            b'A' => return 0,
            b'T' => return 1,
            b'C' => return 2,
            b'G' => return 3,
            _ => panic!("bad value")
        }
    };

    for atom in buf.chunks(4) {
        let mut composite = 0;
        composite |= map(atom[0]) << 6;
        composite |= map(atom[1]) << 4;
        composite |= map(atom[2]) << 2;
        composite |= map(atom[3]);
        out.push(composite);
    }

    return out;
}

#[cfg(test)]
mod tests {
    #[test]
    fn encode() {
        use crate::to_gattaca;

        let input = String::from("hello").into_bytes();
        let output = to_gattaca(input);
        let converted = std::str::from_utf8(&output).unwrap();
        assert_eq!(converted, "TCCATCTTTCGATCGATCGG");
    }

    #[test]
    fn decode() {
        use crate::from_gattaca;

        let input = String::from("TCCATCTTTCGATCGATCGG").into_bytes();
        let output = from_gattaca(input);
        let converted = std::str::from_utf8(&output).unwrap();
        assert_eq!(converted, "hello");
    }

    #[test]
    fn encode_then_decode() {
        use crate::{to_gattaca, from_gattaca};

        let input = String::from("hello").into_bytes();
        let output = to_gattaca(input);
        let output = from_gattaca(output);
        let converted = std::str::from_utf8(&output).unwrap();
        assert_eq!(converted, "hello");
    }

    #[test]
    #[should_panic]
    fn decode_bad_value() {
        use crate::from_gattaca;

        // only values values "A", "T", "C", and "G" are valid in decoding context
        let input = String::from("Z").into_bytes();
        let _output = from_gattaca(input);
    }
}
