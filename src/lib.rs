use sha2::{Digest, Sha256};

fn be24(n: usize) -> [u8; 3] {
    n.to_be_bytes()[5..].try_into().unwrap()
}

fn be40(n: usize) -> [u8; 5] {
    n.to_be_bytes()[3..].try_into().unwrap()
}

fn be56(n: usize) -> [u8; 7] {
    n.to_be_bytes()[1..].try_into().unwrap()
}

fn be64(n: usize) -> [u8; 8] {
    n.to_be_bytes()
}

pub fn chunk_hash(s: &[u8]) -> [u8; 32] {
    if s.len() < 1024 {
        let mut hasher = Sha256::new();
        hasher.update(s);
        hasher.update(b"/");
        hasher.finalize().into()
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

    let mut h = Sha256::new();

    for lane in lanes {
        h.update(lane.finalize());
    }

    h.update(be64(s.len()));
    h.update(b"/J16");
    h.finalize().into()
}

pub fn chunk_list(s: &[u8]) -> Vec<u8> {
    let mut out = Vec::<u8>::new();

    for block in s.chunks(usize::pow(2, 21)) {
        out.extend(chunk_hash(block));
    }

    out.extend(s.len().to_be_bytes());
    out.extend("/T21".as_bytes());

    out
}

pub fn psha2(s: &[u8]) -> Vec<u8> {
    let mut out = Vec::<u8>::new();

    if s.len() == 0 {
        out.push(0);
    } else if s.len() <= usize::pow(2, 21) {
        out.push(1);
        out.extend(be24(s.len()));
        out.extend(chunk_hash(s))
    } else if s.len() <= usize::pow(2, 37) - usize::pow(2, 21) {
        out.push(2);
        out.extend(be40(s.len()));
        out.extend(chunk_hash(&chunk_list(s)));
    } else if s.len() <= usize::pow(2, 52) {
        out.push(3);
        out.extend(be56(s.len()));
        out.extend(chunk_hash(&chunk_list(&chunk_list(s))));
    } else {
        todo!()
    }

    out
}
