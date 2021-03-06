use digest::FixedOutput;
use digest::Input;

#[derive(Debug, Clone)]
pub enum Digestable {
    Sha1(::sha1::Sha1),
    Sha256(::sha2::Sha256),
    Sha512(::sha2::Sha512),
}

impl Digestable {
    pub fn sha1() -> Digestable {
        Digestable::Sha1(::sha1::Sha1::default())
    }

    pub fn sha256() -> Digestable {
        Digestable::Sha256(::sha2::Sha256::default())
    }

    pub fn sha512() -> Digestable {
        Digestable::Sha512(::sha2::Sha512::default())
    }

    // Like digest::Input
    pub fn process(&mut self, data: &[u8]) {
        match *self {
            Digestable::Sha1(ref mut x) => x.input(data),
            Digestable::Sha256(ref mut x) => x.input(data),
            Digestable::Sha512(ref mut x) => x.input(data),
        }
    }

    pub fn hash(self) -> Vec<u8> {
        match self {
            Digestable::Sha1(x) => x.fixed_result().to_vec(),
            Digestable::Sha256(x) => x.fixed_result().to_vec(),
            Digestable::Sha512(x) => x.fixed_result().to_vec(),
        }
    }

    // https://tools.ietf.org/html/rfc3447#section-9.2
    pub fn emsa_pkcs1_v1_5(&self, hash: &[u8], output_len: usize) -> Option<Vec<u8>> {
        // step 1: compute digest

        // step 2
        let mut digest_info = self.asn1_prefix().to_vec();
        digest_info.extend(hash);

        // step 3: intended encoded message length too short
        if output_len <= digest_info.len() + 11 {
            return None;
        }

        // step 4, 5
        let mut ret = Vec::with_capacity(output_len);
        ret.push(0x00);
        ret.push(0x01);

        for _ in 0..(output_len - digest_info.len() - 3) {
            ret.push(0xff);
        }

        ret.push(0x00);
        ret.extend(digest_info);

        assert_eq!(ret.len(), output_len);
        Some(ret)
    }

    pub fn asn1_prefix(&self) -> &'static [u8] {
        // https://tools.ietf.org/html/rfc4880#section-5.2.2
        use self::Digestable::*;

        // rustfmt currently puts these one on a line, which is ugly;
        // this is how they are in the RFC.
        #[cfg_attr(rustfmt, rustfmt_skip)]
        match *self {
            Sha1(_) => &[
                0x30, 0x21, 0x30, 0x09, 0x06, 0x05, 0x2b, 0x0E,
                0x03, 0x02, 0x1A, 0x05, 0x00, 0x04, 0x14],
            Sha256(_) => &[
                0x30, 0x31, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86,
                0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x01, 0x05,
                0x00, 0x04, 0x20],
            Sha512(_) => &[
                0x30, 0x51, 0x30, 0x0d, 0x06, 0x09, 0x60, 0x86,
                0x48, 0x01, 0x65, 0x03, 0x04, 0x02, 0x03, 0x05,
                0x00, 0x04, 0x40],
        }
    }
}
