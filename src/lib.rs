use sha2::{Digest, Sha256};

fn be24(n: u64) -> [u8; 3] {
    n.to_be_bytes()[5..].try_into().unwrap()
}

fn be40(n: u64) -> [u8; 5] {
    n.to_be_bytes()[3..].try_into().unwrap()
}

fn be56(n: u64) -> [u8; 7] {
    n.to_be_bytes()[1..].try_into().unwrap()
}

fn be64(n: u64) -> [u8; 8] {
    n.to_be_bytes()
}

pub fn chunk_hash(s: &[u8]) -> [u8; 32] {
    if s.len() < 1024 {
        Sha256::new()
            .chain_update(s)
            .chain_update(b"/")
            .finalize()
            .into()
    } else {
        sha256x16(s)
    }
}

pub fn sha256x16(s: &[u8]) -> [u8; 32] {
    let mut lanes: [Sha256; 16] = core::array::from_fn(|_| Sha256::new());

    for block in s.chunks(64) {
        for (i, sub_block) in block.chunks(4).enumerate() {
            lanes[i].update(sub_block);
        }
    }

    let mut hasher = Sha256::new();

    for lane in lanes {
        hasher.update(lane.finalize());
    }

    hasher
        .chain_update(be64(s.len() as u64))
        .chain_update(b"/J16")
        .finalize()
        .into()
}

pub fn chunk_list(s: &[u8]) -> Vec<u8> {
    let mut out = Vec::<u8>::new();

    for block in s.chunks(1 << 21) {
        out.extend(chunk_hash(block));
    }

    out.extend(s.len().to_be_bytes());
    out.extend("/T21".as_bytes());

    out
}

pub fn psha2(s: &[u8]) -> Vec<u8> {
    let mut out = Vec::<u8>::new();

    if s.is_empty() {
        out.push(0);
    } else if s.len() <= 1 << 21 {
        out.push(1);
        out.extend(be24(s.len() as u64));
        out.extend(chunk_hash(s))
    } else if s.len() <= (1 << 37) - (1 << 21) {
        out.push(2);
        out.extend(be40(s.len() as u64));
        out.extend(chunk_hash(&chunk_list(s)));
    } else if s.len() <= 1 << 52 {
        out.push(3);
        out.extend(be56(s.len() as u64));
        out.extend(chunk_hash(&chunk_list(&chunk_list(s))));
    } else {
        unimplemented!("Hashing data larger than 2**52B is undefined in the spec.")
    }

    out
}
