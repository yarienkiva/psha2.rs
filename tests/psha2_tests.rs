#[cfg(test)]
mod tests {
    use hex_literal::hex;
    use psha2::*;

    fn seq(n: i64) -> String {
        let mut out = String::new();

        for i in 1..(n + 1) {
            out.push_str(i.to_string().as_str());
            out.push_str("\n");
        }

        out
    }

    #[test]
    fn test_chunk_hash() {
        // < 1024, just concat s + '/'
        let hash1 = chunk_hash(b"hello");
        assert_eq!(
            hash1,
            hex!("b2f361b1385fd06bb7807a4d7d26064911b1a7efe6746378ffe63a7a1c234ce3")
        );

        // > 1024, call sha256x16
        let hash2 = chunk_hash(seq(300).as_bytes());
        assert_eq!(
            hash2,
            hex!("cde9c9596fd8e050be0545c6fbb42c5a96796452a17b3adef41c0252e0547125")
        );
    }

    #[test]
    fn test_chunk_list() {
        let hash1 = chunk_list(seq(913470).as_bytes());

        assert_eq!(
            hash1,
            hex!(
            "009c35809036580c90709b1e246d9ee814eab713386626565342ab64e3778b9f"
            "2595560c0292d3dbdc182eb1f34cfde0dee72b2fb0784b7ae5f752f8f2274e79"
            "8a8c87368972ef766c8f91a2bdbf0675b2012a165c8435bc45314019eaf4f3bc"
            "00000000005fdfb12f543231"
                )
        )
    }

    #[test]
    fn test_psha2() {
        let hash1 = psha2(seq(300).as_bytes());
        assert_eq!(
            hash1,
            hex!("01000444cde9c9596fd8e050be0545c6fbb42c5a96796452a17b3adef41c0252e0547125")
        );

        let hash2 = psha2(seq(913470).as_bytes());
        assert_eq!(
            hash2,
            hex!("0200005fdfb1ad5ab7fdae86f18fc023daffea11eac2d644c6d3df9c0f0afc6630cb7dc43f58")
        );
    }
}
