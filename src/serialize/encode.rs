use ::{Error, UInt256};

pub trait BitcoinEncoder: Sized {
   fn param(&self) -> &BitcoinEncodeParam;

   fn encode_u8(&mut self, v:u8) -> Result<usize, Error>;
   fn encode_u16le(&mut self, v:u16) -> Result<usize, Error>;
   fn encode_u32le(&mut self, v:u32) -> Result<usize, Error>;
   fn encode_u64le(&mut self, v:u64) -> Result<usize, Error>;
   fn encode_u16be(&mut self, v:u16) -> Result<usize, Error>;
   fn encode_u32be(&mut self, v:u32) -> Result<usize, Error>;
   fn encode_u64be(&mut self, v:u64) -> Result<usize, Error>;

   fn encode_i8(&mut self, v:i8) -> Result<usize, Error>;
   fn encode_i16le(&mut self, v:i16) -> Result<usize, Error>;
   fn encode_i32le(&mut self, v:i32) -> Result<usize, Error>;
   fn encode_i64le(&mut self, v:i64) -> Result<usize, Error>;
   fn encode_i16be(&mut self, v:i16) -> Result<usize, Error>;
   fn encode_i32be(&mut self, v:i32) -> Result<usize, Error>;
   fn encode_i64be(&mut self, v:i64) -> Result<usize, Error>;
   
   fn encode_bool(&mut self, v:bool) -> Result<usize, Error>;
   fn encode_varint(&mut self, v:u64) -> Result<usize, Error>;
   fn encode_uint256(&mut self, v:&UInt256) -> Result<usize, Error>;
   fn encode_array_u8(&mut self, v:&[u8]) -> Result<usize, Error>;
   fn encode_sequence_u8(&mut self, v:&[u8]) -> Result<usize, Error>;
   
   fn encode<A:BitcoinEncodee<Self>>(&mut self, v:&A) -> Result<usize, Error> {
      v.encode(self)
   }
   fn encode_sequence<A:BitcoinEncodee<Self>>(&mut self, v:&[A]) -> Result<usize, Error> {
      let mut r:usize = 0;
      for elm in v.iter() {
         r += try!(elm.encode(self));
      }
      Ok(r)
   }
}

pub trait BitcoinEncodee<E:BitcoinEncoder> {
   fn encode(&self, e:&mut E) -> Result<usize, Error>;
}   

#[derive(Debug,Clone)]
pub struct BitcoinEncodeParam {
   version: i32,
   serialize_type: i32,
}

const SER_NET:i32     = 1 << 0;
const SER_DISK:i32    = 1 << 1;
const SER_GETHASH:i32 = 1 << 2;

impl BitcoinEncodeParam {
   pub fn new() -> Self {
      BitcoinEncodeParam {
         version: ::protocol::PROTOCOL_VERSION,
         serialize_type: 0,
      }
   }
   pub fn version(&self)     -> i32  { self.version }
   pub fn is_disk(&self)     -> bool { (self.serialize_type & SER_DISK) != 0 }
   pub fn is_net(&self)      -> bool { (self.serialize_type & SER_NET) != 0 }
   pub fn is_gethash(&self)  -> bool { (self.serialize_type & SER_GETHASH) != 0 }

   pub fn set_version(&mut self, v:i32) -> &mut Self { self.version = v; self }
   pub fn set_version_latest(&mut self) -> &mut Self { self.version = ::protocol::PROTOCOL_VERSION; self }
   pub fn set_disk(&mut self)           -> &mut Self { self.serialize_type |= SER_DISK; self }
   pub fn set_net(&mut self)            -> &mut Self { self.serialize_type |= SER_NET; self }
   pub fn set_gethash(&mut self)        -> &mut Self { self.serialize_type |= SER_GETHASH; self }
}

