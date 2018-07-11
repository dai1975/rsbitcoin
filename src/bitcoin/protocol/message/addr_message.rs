use std;
use super::super::{ NetworkAddress, NetworkAddressEncodee, NetworkAddressDecodee };

#[derive(Debug,Default,Clone)]
pub struct AddrMessage {
   pub addrs : Vec<NetworkAddress>,
}

use super::message::{ Message, COMMAND_LENGTH };
impl Message for AddrMessage {
   const COMMAND:[u8; COMMAND_LENGTH] = [0x61, 0x64, 0x64, 0x72, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
}

impl std::fmt::Display for AddrMessage {
   fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
      write!(f, "Addr(len={})", self.addrs.len())
   }
}


use ::bitcoin::serialize::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for AddrMessage {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      {
         use super::super::apriori::MAX_ADDR_SIZE;
         let tmp:Vec<NetworkAddressEncodee> = self.addrs.iter().map(|a| NetworkAddressEncodee(a, true)).collect();
         r += try!(e.encode_var_array(&tmp, MAX_ADDR_SIZE));
      }
      Ok(r)
   }
}
impl BitcoinDecodee for AddrMessage {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      {
         use super::super::apriori::MAX_ADDR_SIZE;
         let mut tmp:Vec<NetworkAddressDecodee> = Vec::new();
         r += try!(d.decode_var_array(&mut tmp, MAX_ADDR_SIZE));
         self.addrs = tmp.into_iter().map(|a| a.0).collect();
      }
      Ok(r)
   }
}