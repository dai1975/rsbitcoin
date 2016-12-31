use std;
use super::message::{ Message, Command };

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
   pub command : Command,
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

impl Message for RejectMessage {
   const COMMAND: Command = Command { data: &[0x72, 0x65, 0x6a, 0x65, 0x63, 0x74, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00] };
}

impl RejectMessage {
   pub fn new(msg_: &super::Message, code_:u8, reason_:&String) -> RejectMessage {
      RejectMessage {
         command: msg_.get_command(),
         code:    code_,
         reason:  reason_.clone(),
      }
   }
}

impl std::fmt::Display for RejectMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Reject(cmd={}, code={}, reason={})",
             self.command.as_str(), self.code, self.reason)
   }
}

