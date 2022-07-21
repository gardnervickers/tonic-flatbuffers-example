// The hello_generated module is generated using the flatbuffers compiler. You can find the command in the `gen.sh`
// script at the project root.
#[allow(dead_code, unused_imports)]
#[path = "../proto/hello_generated.rs"]
pub(crate) mod hello_generated;

use bytes::{Buf, Bytes};

pub(crate) use hello_generated::hello_world::greeter::{
    GreetRequest, GreetRequestBuilder, GreetResponse, GreetResponseBuilder,
};

/// [FlatbufferOwned] is a trait implemented by types which can be represented as flatbuffers, but own their contents.
pub trait FlatbufferOwned {
    fn from_buf(buf: impl Buf) -> Self;

    fn as_slice(&self) -> &[u8];
}

pub struct GreetRequestOwned {
    buf: Bytes,
}

pub struct GreetResponseOwned {
    buf: Bytes,
}

impl GreetRequestOwned {
    pub fn new(name: impl AsRef<str>) -> Self {
        let mut root_bldr = flatbuffers::FlatBufferBuilder::new();
        root_bldr.reset();
        let message = root_bldr.create_string(name.as_ref());
        let mut bldr = GreetRequestBuilder::new(&mut root_bldr);
        bldr.add_name(message);
        let req_off = bldr.finish();
        root_bldr.finish(req_off, None);
        let buf = root_bldr.finished_data();
        let buf = Bytes::copy_from_slice(buf);
        Self { buf }
    }

    pub fn inner(&self) -> &Bytes {
        &self.buf
    }

    pub fn name(&self) -> Option<&str> {
        let rt = flatbuffers::root::<GreetRequest>(&self.buf[..]).unwrap();
        rt.name()
    }
}

impl FlatbufferOwned for GreetRequestOwned {
    fn from_buf(mut buf: impl Buf) -> Self {
        let buf = buf.copy_to_bytes(buf.remaining());
        Self { buf }
    }

    fn as_slice(&self) -> &[u8] {
        &self.inner()[..]
    }
}

impl GreetResponseOwned {
    pub fn new(message: impl AsRef<str>) -> Self {
        let mut root_bldr = flatbuffers::FlatBufferBuilder::new();
        root_bldr.reset();
        let message = root_bldr.create_string(message.as_ref());
        let mut bldr = GreetResponseBuilder::new(&mut root_bldr);
        bldr.add_message(message);
        let resp_off = bldr.finish();
        root_bldr.finish(resp_off, None);
        let buf = root_bldr.finished_data();
        let buf = Bytes::copy_from_slice(buf);
        Self { buf }
    }

    pub fn inner(&self) -> &Bytes {
        &self.buf
    }

    pub fn message(&self) -> Option<&str> {
        let rt = flatbuffers::root::<GreetResponse>(&self.buf[..]).unwrap();
        rt.message()
    }
}

impl FlatbufferOwned for GreetResponseOwned {
    fn from_buf(mut buf: impl Buf) -> Self {
        let buf = buf.copy_to_bytes(buf.remaining());
        Self { buf }
    }

    fn as_slice(&self) -> &[u8] {
        &self.inner()[..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip() {
        let req = GreetRequestOwned::new("gardner");
        assert_eq!(req.name(), Some("gardner"));

        let req = GreetResponseOwned::new("hello");
        assert_eq!(req.message(), Some("hello"));
    }
}
