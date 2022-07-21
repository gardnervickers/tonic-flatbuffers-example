use bytes::{Buf, BufMut, Bytes};
use std::marker::PhantomData;
use tonic::{
    codec::{Codec, DecodeBuf, Decoder, EncodeBuf, Encoder},
    Status,
};

use crate::hello_generated::hello_world::greeter::{
    GreetRequest, GreetResponse, GreetResponseArgs, Time,
};

/// Flatbuffers encoder for transmitting the binary representation of a flatbuffer over GRPC.
#[derive(Debug, Clone, Copy, Default)]
pub struct FlatbuffersEncoder<T>(PhantomData<T>);

/// Flatbuffers decoder for parsing the binary representation of a flatbuffer over GRPC.
#[derive(Debug, Clone, Copy, Default)]
pub struct FlatbuffersDecoder<T>(PhantomData<T>);

/// [FlatbuffersMessage] contains the buffer for a flatbuffer.
///
/// The buffer is guaranteed to be validated.
pub struct FlatbuffersMessage {
    buf: Bytes,
}

impl<'this> FlatbuffersMessage {
    fn follow<T>(&'this self) -> T::Inner
    where
        T: flatbuffers::Follow<'this> + flatbuffers::Verifiable,
    {
        T::follow(&self.buf[..], 0)
    }
}

impl Decoder for FlatbuffersDecoder<FlatbuffersMessage> {
    type Item = FlatbuffersMessage;

    type Error = Status;

    fn decode(&mut self, buf: &mut DecodeBuf<'_>) -> Result<Option<Self::Item>, Self::Error> {
        if !buf.has_remaining() {
            return Ok(None);
        }
        // Copy will just bump the refcnt
        let copied = buf.copy_to_bytes(buf.remaining());
        // TODO: run fbs validator
        Ok(Some(FlatbuffersMessage { buf: copied }))
    }
}

impl Encoder for FlatbuffersEncoder<FlatbuffersMessage> {
    type Item = FlatbuffersMessage;

    type Error = Status;

    fn encode(&mut self, item: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        dst.put(item.buf);
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct FlatbuffersCodec();

impl Codec for FlatbuffersCodec {
    type Encode = FlatbuffersMessage;

    type Decode = FlatbuffersMessage;

    type Encoder = FlatbuffersEncoder<FlatbuffersMessage>;

    type Decoder = FlatbuffersDecoder<FlatbuffersMessage>;

    fn encoder(&mut self) -> Self::Encoder {
        todo!()
    }

    fn decoder(&mut self) -> Self::Decoder {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_trip() {}
}
