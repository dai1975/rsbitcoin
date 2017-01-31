use std;

//pub const MAX_REJECT_MESSAGE_LENGTH:usize = 111;

const REJECT_MALFORMED:u8        = 0x01;
const REJECT_INVALID:u8          = 0x10;
const REJECT_OBSOLETE:u8         = 0x11;
const REJECT_DUPLICATE:u8        = 0x12;
const REJECT_NON_STANDARD:u8     = 0x40;
const REJECT_DUST:u8             = 0x41;
const REJECT_INSUFFICIENT_FEE:u8 = 0x42;
const REJECT_CHECKPOINT:u8       = 0x43;

#[derive(Debug,Clone)]
pub struct RejectMessage {
   pub command : String, //not [u8;12] but var_str. check https://en.bitcoin.it/wiki/Protocol_documentation#reject
   pub code    : u8,
   pub reason  : String,
}

impl RejectMessage {
   pub const MAX_REJECT_MESSAGE_LENGTH:usize = 111;
   pub fn is_malformed(&self)        -> bool { self.code == REJECT_MALFORMED }
   pub fn is_invalid(&self)          -> bool { self.code == REJECT_INVALID }
   pub fn is_obsolete(&self)         -> bool { self.code == REJECT_OBSOLETE }
   pub fn is_duplicate(&self)        -> bool { self.code == REJECT_DUPLICATE }
   pub fn is_non_standard(&self)     -> bool { self.code == REJECT_NON_STANDARD }
   pub fn is_dust(&self)             -> bool { self.code == REJECT_DUST }
   pub fn is_insufficient_fee(&self) -> bool { self.code == REJECT_INSUFFICIENT_FEE }
   pub fn is_checkpoint(&self)       -> bool { self.code == REJECT_CHECKPOINT }
}

impl RejectMessage {
   pub fn new(msg_: &super::Message, code_:u8, reason_:&String) -> RejectMessage {
      RejectMessage {
         command: msg_.get_command().as_str().to_string(),
         code:    code_,
         reason:  reason_.clone(),
      }
   }
}

impl std::fmt::Display for RejectMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Reject(cmd={}, code={}, reason={})",
             self.command, self.code, self.reason)
   }
}

use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for RejectMessage {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.command.encode(e, ::std::usize::MAX));
      r += try!(e.encode_u8(self.code));
      r += try!(self.reason.encode(e, RejectMessage::MAX_REJECT_MESSAGE_LENGTH));
      Ok(r)
   }
}
impl Decodee for RejectMessage {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(self.command.decode(d, ::std::usize::MAX));
      r += try!(d.decode_u8(&mut self.code));
      r += try!(self.reason.decode(d, RejectMessage::MAX_REJECT_MESSAGE_LENGTH));
      // この後に拡張データがあるが、メッセージヘッダのサイズを見ないと分からない。
      Ok(r)
   }
}
