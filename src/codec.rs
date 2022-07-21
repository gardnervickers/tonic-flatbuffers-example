use bytes::{Buf, BufMut};
use std::marker::PhantomData;
use tonic::{
    codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder},
    Status,
};

use crate::common::FlatbufferOwned;

/// Flatbuffers encoder for transmitting the binary representation of a flatbuffer over GRPC.
#[derive(Debug, Clone, Copy, Default)]
pub struct FlatbuffersEncoder<T>(PhantomData<T>);

/// Flatbuffers decoder for parsing the binary representation of a flatbuffer over GRPC.
#[derive(Debug, Clone, Copy, Default)]
pub struct FlatbuffersDecoder<T>(PhantomData<T>);

impl<M> Decoder for FlatbuffersDecoder<M>
where
    M: FlatbufferOwned,
{
    type Item = M;

    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        if !buf.has_remaining() {
            return Ok(None);
        }
        let msg = M::from_buf(buf);
        Ok(Some(msg))
    }
}

impl<M> Encoder for FlatbuffersEncoder<M>
where
    M: FlatbufferOwned,
{
    type Item = M;

    type Error = Status;

    fn encode(&mut self, item: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        let slice = item.as_slice();
        dst.put(slice);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FlatbuffersCodec<T, U>(PhantomData<(T, U)>);

impl<T, U> Default for FlatbuffersCodec<T, U> {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<T, U> Codec for FlatbuffersCodec<T, U>
where
    T: FlatbufferOwned + Send + 'static,
    U: FlatbufferOwned + Send + 'static,
{
    type Encode = T;

    type Decode = U;

    type Encoder = FlatbuffersEncoder<T>;

    type Decoder = FlatbuffersDecoder<U>;

    fn encoder(&mut self) -> Self::Encoder {
        FlatbuffersEncoder(PhantomData)
    }

    fn decoder(&mut self) -> Self::Decoder {
        FlatbuffersDecoder(PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {}
}
