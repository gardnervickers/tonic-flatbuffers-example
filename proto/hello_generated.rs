// automatically generated by the FlatBuffers compiler, do not modify



use core::mem;
use core::cmp::Ordering;

extern crate flatbuffers;
use self::flatbuffers::{EndianScalar, Follow};

#[allow(unused_imports, dead_code)]
pub mod hello_world {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};
#[allow(unused_imports, dead_code)]
pub mod greeter {

  use core::mem;
  use core::cmp::Ordering;

  extern crate flatbuffers;
  use self::flatbuffers::{EndianScalar, Follow};

pub enum GreetRequestOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct GreetRequest<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for GreetRequest<'a> {
  type Inner = GreetRequest<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> GreetRequest<'a> {
  pub const VT_NAME: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    GreetRequest { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args GreetRequestArgs<'args>
  ) -> flatbuffers::WIPOffset<GreetRequest<'bldr>> {
    let mut builder = GreetRequestBuilder::new(_fbb);
    if let Some(x) = args.name { builder.add_name(x); }
    builder.finish()
  }


  #[inline]
  pub fn name(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(GreetRequest::VT_NAME, None)
  }
}

impl flatbuffers::Verifiable for GreetRequest<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("name", Self::VT_NAME, false)?
     .finish();
    Ok(())
  }
}
pub struct GreetRequestArgs<'a> {
    pub name: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for GreetRequestArgs<'a> {
  #[inline]
  fn default() -> Self {
    GreetRequestArgs {
      name: None,
    }
  }
}

pub struct GreetRequestBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> GreetRequestBuilder<'a, 'b> {
  #[inline]
  pub fn add_name(&mut self, name: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(GreetRequest::VT_NAME, name);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> GreetRequestBuilder<'a, 'b> {
    let start = _fbb.start_table();
    GreetRequestBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<GreetRequest<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for GreetRequest<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("GreetRequest");
      ds.field("name", &self.name());
      ds.finish()
  }
}
pub enum GreetResponseOffset {}
#[derive(Copy, Clone, PartialEq)]

pub struct GreetResponse<'a> {
  pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for GreetResponse<'a> {
  type Inner = GreetResponse<'a>;
  #[inline]
  fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
    Self { _tab: flatbuffers::Table { buf, loc } }
  }
}

impl<'a> GreetResponse<'a> {
  pub const VT_MESSAGE: flatbuffers::VOffsetT = 4;

  #[inline]
  pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
    GreetResponse { _tab: table }
  }
  #[allow(unused_mut)]
  pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
    _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
    args: &'args GreetResponseArgs<'args>
  ) -> flatbuffers::WIPOffset<GreetResponse<'bldr>> {
    let mut builder = GreetResponseBuilder::new(_fbb);
    if let Some(x) = args.message { builder.add_message(x); }
    builder.finish()
  }


  #[inline]
  pub fn message(&self) -> Option<&'a str> {
    self._tab.get::<flatbuffers::ForwardsUOffset<&str>>(GreetResponse::VT_MESSAGE, None)
  }
}

impl flatbuffers::Verifiable for GreetResponse<'_> {
  #[inline]
  fn run_verifier(
    v: &mut flatbuffers::Verifier, pos: usize
  ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
    use self::flatbuffers::Verifiable;
    v.visit_table(pos)?
     .visit_field::<flatbuffers::ForwardsUOffset<&str>>("message", Self::VT_MESSAGE, false)?
     .finish();
    Ok(())
  }
}
pub struct GreetResponseArgs<'a> {
    pub message: Option<flatbuffers::WIPOffset<&'a str>>,
}
impl<'a> Default for GreetResponseArgs<'a> {
  #[inline]
  fn default() -> Self {
    GreetResponseArgs {
      message: None,
    }
  }
}

pub struct GreetResponseBuilder<'a: 'b, 'b> {
  fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
  start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}
impl<'a: 'b, 'b> GreetResponseBuilder<'a, 'b> {
  #[inline]
  pub fn add_message(&mut self, message: flatbuffers::WIPOffset<&'b  str>) {
    self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(GreetResponse::VT_MESSAGE, message);
  }
  #[inline]
  pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> GreetResponseBuilder<'a, 'b> {
    let start = _fbb.start_table();
    GreetResponseBuilder {
      fbb_: _fbb,
      start_: start,
    }
  }
  #[inline]
  pub fn finish(self) -> flatbuffers::WIPOffset<GreetResponse<'a>> {
    let o = self.fbb_.end_table(self.start_);
    flatbuffers::WIPOffset::new(o.value())
  }
}

impl core::fmt::Debug for GreetResponse<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let mut ds = f.debug_struct("GreetResponse");
      ds.field("message", &self.message());
      ds.finish()
  }
}
#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_greet_response<'a>(buf: &'a [u8]) -> GreetResponse<'a> {
  unsafe { flatbuffers::root_unchecked::<GreetResponse<'a>>(buf) }
}

#[inline]
#[deprecated(since="2.0.0", note="Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_greet_response<'a>(buf: &'a [u8]) -> GreetResponse<'a> {
  unsafe { flatbuffers::size_prefixed_root_unchecked::<GreetResponse<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `GreetResponse`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_greet_response_unchecked`.
pub fn root_as_greet_response(buf: &[u8]) -> Result<GreetResponse, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root::<GreetResponse>(buf)
}
#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `GreetResponse` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_greet_response_unchecked`.
pub fn size_prefixed_root_as_greet_response(buf: &[u8]) -> Result<GreetResponse, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root::<GreetResponse>(buf)
}
#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `GreetResponse` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_greet_response_unchecked`.
pub fn root_as_greet_response_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<GreetResponse<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::root_with_opts::<GreetResponse<'b>>(opts, buf)
}
#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `GreetResponse` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_greet_response_unchecked`.
pub fn size_prefixed_root_as_greet_response_with_opts<'b, 'o>(
  opts: &'o flatbuffers::VerifierOptions,
  buf: &'b [u8],
) -> Result<GreetResponse<'b>, flatbuffers::InvalidFlatbuffer> {
  flatbuffers::size_prefixed_root_with_opts::<GreetResponse<'b>>(opts, buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a GreetResponse and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid `GreetResponse`.
pub unsafe fn root_as_greet_response_unchecked(buf: &[u8]) -> GreetResponse {
  flatbuffers::root_unchecked::<GreetResponse>(buf)
}
#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed GreetResponse and returns it.
/// # Safety
/// Callers must trust the given bytes do indeed contain a valid size prefixed `GreetResponse`.
pub unsafe fn size_prefixed_root_as_greet_response_unchecked(buf: &[u8]) -> GreetResponse {
  flatbuffers::size_prefixed_root_unchecked::<GreetResponse>(buf)
}
#[inline]
pub fn finish_greet_response_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<GreetResponse<'a>>) {
  fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_greet_response_buffer<'a, 'b>(fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>, root: flatbuffers::WIPOffset<GreetResponse<'a>>) {
  fbb.finish_size_prefixed(root, None);
}
}  // pub mod Greeter
}  // pub mod HelloWorld

