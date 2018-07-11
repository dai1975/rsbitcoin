use super::apriori::COMMAND_LENGTH;

#[derive(Debug,Default,Clone)]
pub struct MessageHeader {
   pub magic:    u32,
   pub command:  [u8; COMMAND_LENGTH],
   pub length:   u32,
   pub checksum: u32,
}


use ::bitcoin::serialize::{
   Encoder as BitcoinEncoder,
   Encodee as BitcoinEncodee,
   Decoder as BitcoinDecoder,
   Decodee as BitcoinDecodee,
};
impl BitcoinEncodee for MessageHeader {
   fn encode(&self, e:&mut BitcoinEncoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_u32le(self.magic));
      r += try!(e.encode_octets(&self.command[..]));
      r += try!(e.encode_u32le(self.length));
      r += try!(e.encode_u32le(self.checksum));
      Ok(r)
   }
}
impl BitcoinDecodee for MessageHeader {
   fn decode(&mut self, d:&mut BitcoinDecoder) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_u32le(&mut self.magic));
      r += try!(d.decode_octets(&mut self.command[..]));
      r += try!(d.decode_u32le(&mut self.length));
      r += try!(d.decode_u32le(&mut self.checksum));
      Ok(r)
   }
}


#[test]
fn test_message_header() {
   use super::apriori::COMMAND_LENGTH;
   let VERSION:[u8; COMMAND_LENGTH] = [0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00];
   let obj = MessageHeader {
      magic:    ::bitcoin::presets::bitcoin_mainnet::CHAIN.magic,
      command:  VERSION,
      length:   0x39,
      checksum: 0x12345678,
   };

   let mut w = ::serialize::VecWriteStream::default();
   {
      let     m = ::bitcoin::serialize::Medium::new("net").unwrap();
      let mut e = ::bitcoin::serialize::Encoder::new(&mut w, &m);
      assert_matches!(obj.encode(&mut e), Ok(24usize));
   }
   assert_eq!(&w.get_ref()[..24],
              [0xF9, 0xBE, 0xB4, 0xD9,
               0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x00, 0x00, 0x00, 0x00,
               0x39, 0x00, 0x00, 0x00,
               0x78, 0x56, 0x34, 0x12]);
}