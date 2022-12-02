//! Implements the raw codec.

use core::{convert::TryFrom, iter::Extend};
use libipld_core::cid::Cid;
use libipld_core::codec::{Codec, Decode, Encode, References};
use libipld_core::error::{Result, UnsupportedCodec};
use std::io::{Read, Seek, Write};
use libipld_core::ipld::Ipld;


mod codec;

/// Git codec.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GitCodec;

impl Codec for GitCodec {}


impl From<GitCodec> for u64 {
    fn from(_: GitCodec) -> Self {
        0x78
    }
}

impl TryFrom<u64> for GitCodec {
    type Error = UnsupportedCodec;

    fn try_from(_: u64) -> core::result::Result<Self, Self::Error> {
        Ok(Self)
    }
}




impl Encode<GitCodec> for [u8] {
    fn encode<W: Write>(&self, _: GitCodec, w: &mut W) -> Result<()> {
        w.write_all(self).map_err(anyhow::Error::msg)
    }
}
/*

impl Encode<GitCodec> for Box<[u8]> {
    fn encode<W: Write>(&self, _: GitCodec, w: &mut W) -> Result<()> {
        w.write_all(&self[..]).map_err(anyhow::Error::msg)
    }
}
*/


impl Encode<GitCodec> for Vec<u8> {
    fn encode<W: Write>(&self, _: GitCodec, w: &mut W) -> Result<()> {
        w.write_all(&self[..]).map_err(anyhow::Error::msg)
    }
}

impl Encode<GitCodec> for Ipld {
    fn encode<W: Write>(&self, c: GitCodec, w: &mut W) -> Result<()> {
        if let Ipld::Bytes(bytes) = self {
            bytes.encode(c, w)
        } else {
            Err(anyhow::Error::msg(libipld_core::error::TypeError::new(
                libipld_core::error::TypeErrorType::Bytes,
                self,
            )))
        }
    }
}
/*

impl Decode<GitCodec> for Box<[u8]> {
    fn decode<R: Read + Seek>(c: GitCodec, r: &mut R) -> Result<Self> {
        let buf: Vec<u8> = Decode::decode(c, r)?;
        Ok(buf.into_boxed_slice())
    }
}
*/

impl Decode<GitCodec> for Vec<u8> {
    fn decode<R: Read + Seek>(_: GitCodec, r: &mut R) -> Result<Self> {
        let mut buf = vec![];
        r.read_to_end(&mut buf).map_err(anyhow::Error::msg)?;
        Ok(buf)
    }
}

impl Decode<GitCodec> for Ipld {
    fn decode<R: Read + Seek>(c: GitCodec, r: &mut R) -> Result<Self> {
        let bytes: Vec<u8> = Decode::decode(c, r)?;
        Ok(Ipld::Bytes(bytes))
    }
}


impl References<GitCodec> for Ipld {
    fn references<R: Read, E: Extend<Cid>>(_c: GitCodec, _r: &mut R, _set: &mut E) -> Result<()> {
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use libipld_core::ipld::Ipld;
    use super::*;

    #[test]
    fn test_raw_codec() {
        let data: &[u8] = &[0, 1, 2, 3];
        let bytes = GitCodec.encode(data).unwrap();
        assert_eq!(data, &*bytes);
        let data2: Vec<u8> = GitCodec.decode(&bytes).unwrap();
        assert_eq!(data, &*data2);
/*
        let ipld = Ipld::Bytes(data2);
        let bytes = GitCodec.encode(&ipld).unwrap();
        assert_eq!(data, &*bytes);
        let ipld2: Ipld = GitCodec.decode(&bytes).unwrap();
        assert_eq!(ipld, ipld2);

 */
    }
}
