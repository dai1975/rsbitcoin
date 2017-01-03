use ::{Error, UInt256};
use super::{BitcoinDecoder, BitcoinCodecParam};
use super::{ReadStream};

pub struct BitcoinDeserializer<R:ReadStream> {
   r: R,
   p: BitcoinCodecParam,
}
impl <R:ReadStream> BitcoinDeserializer<R> {
   pub fn new_with(r:R) -> Self { BitcoinDeserializer {r:r, p:BitcoinCodecParam::new()} }
   pub fn readstream(&self) -> &R { &self.r }
   pub fn mut_param(&mut self) -> &mut BitcoinCodecParam { &mut self.p }
}

macro_rules! def_decode {
   ($n:ident, $t:ty, $size:expr) => ( interpolate_idents! {
      #[inline(always)]
      fn [decode_ $n](&mut self, v:&mut $t) -> Result<usize, Error> {
         try!(self.r.[readto_ $n](v));
         Ok($size as usize)
      }
   } )
}

impl <R:ReadStream> BitcoinDecoder for BitcoinDeserializer<R> {
   fn param(&self) -> &BitcoinCodecParam { &self.p }
   
   def_decode!{u8,     u8, 1}
   def_decode!{u16le, u16, 2}
   def_decode!{u32le, u32, 4}
   def_decode!{u64le, u64, 8}
   def_decode!{u16be, u16, 2}
   def_decode!{u32be, u32, 4}
   def_decode!{u64be, u64, 8}
   
   def_decode!{i8,     i8, 1}
   def_decode!{i16le, i16, 2}
   def_decode!{i32le, i32, 4}
   def_decode!{i64le, i64, 8}
   def_decode!{i16be, i16, 2}
   def_decode!{i32be, i32, 4}
   def_decode!{i64be, i64, 8}

   #[inline(always)]
   fn decode_bool(&mut self, v:&mut bool) -> Result<usize, Error> {
      let mut x:u8 = 0;
      try!(self.r.readto_u8(&mut x));
      *v = x == 1;
      Ok(1usize)
   }

   fn decode_varint(&mut self, v:&mut u64) -> Result<usize, Error> {
      let mut x:u8 = 0;
      try!(self.r.readto_u8(&mut x));
      if x < 253 {
         *v = x as u64;
         Ok(1)
      } else if x == 253 {
         let mut y:u16 = 0;
         try!(self.r.readto_u16le(&mut y));
         *v = y as u64;
         Ok(3)
      } else if x == 253 {
         let mut y:u32 = 0;
         try!(self.r.readto_u32le(&mut y));
         *v = y as u64;
         Ok(5)
      } else {
         try!(self.r.readto_u64le(v));
         Ok(9)
      }
   }

   #[inline(always)]
   fn decode_uint256(&mut self, v:&mut UInt256) -> Result<usize, Error> {
      self.decode_array_u8(&mut v.data[..])
   }
   
   #[inline(always)]
   fn decode_array_u8(&mut self, v:&mut [u8]) -> Result<usize, Error> {
      let r = try!(self.r.read(v));
      Ok(r)
   }
   
   #[inline(always)]
   fn decode_sequence_u8(&mut self, v:&mut Vec<u8>) -> Result<usize, Error> {
      let mut r:usize = 0;
      {
         let mut x:u64 = 0;
         r += try!(self.decode_varint(&mut x));
         v.resize(x as usize, 0);
      }
      r += try!(self.r.read(v.as_mut_slice()));
      Ok(r)
   }
}


use std::borrow::Borrow;
use super::SliceReadStream;
pub type SliceBitcoinDeserializer<T: Borrow<[u8]>> = BitcoinDeserializer<SliceReadStream<T>>;
impl <T: Borrow<[u8]>> SliceBitcoinDeserializer<T> {
   pub fn new(inner:T) -> Self { BitcoinDeserializer::new_with( SliceReadStream::new(inner) ) }
   pub fn as_slice(&self) -> &[u8] { self.r.as_slice() }
   pub fn rewind(&mut self) { self.r.rewind() }
   pub fn inner(self) -> T { self.r.inner() }
}

use super::FixedReadStream;
pub type FixedBitcoinDeserializer = BitcoinDeserializer<FixedReadStream>;
impl FixedBitcoinDeserializer {
   pub fn new(size:usize) -> Self { BitcoinDeserializer::new_with( FixedReadStream::new(size) ) }
   pub fn as_slice(&self) -> &[u8] { self.r.as_slice() }
   pub fn rewind(&mut self) { self.r.rewind() }
   pub fn inner(self) -> Box<[u8]> { self.r.inner() }
}

#[test]
fn test_cursor_vec() {
   use std::io::Cursor;
   let mut v = Vec::<u8>::with_capacity(100);
   v.push(1);
   v.push(0);
   let mut ser = BitcoinDeserializer::new_with(Cursor::new(v));

   let mut r = false;
   assert_matches!(ser.decode_bool(&mut r),  Ok(1));
   assert_eq!(true, r);
   
   assert_matches!(ser.decode_bool(&mut r), Ok(1));
   assert_eq!(false, r);
}

/*
#[test]
fn test_slice() {
   {
      let mut ser = SliceBitcoinSerializer::new([0u8; 32]);
      assert_eq!(32, ser.len());
      assert_matches!(ser.encode_bool(true),  Ok(1));
      assert_matches!(ser.encode_bool(false), Ok(1));
      assert_eq!(32, ser.len());
      assert_eq!([0x01, 0x00], &ser.as_slice()[0..2]);
   }
   {
      let mut v = Vec::<u8>::with_capacity(100);
      unsafe { v.set_len(100); }
      let mut ser = SliceBitcoinSerializer::new(v);
      assert_eq!(100, ser.len());
      assert_matches!(ser.encode_bool(true),  Ok(1));
      assert_matches!(ser.encode_bool(false), Ok(1));
      assert_eq!(100, ser.len());
      assert_eq!([0x01, 0x00], &ser.as_slice()[0..2]);
   }
}

#[test]
fn test_serializer_fixed() {
   let mut ser = FixedBitcoinSerializer::new(100);
   assert_eq!(100, ser.len());
   assert_matches!(ser.encode_bool(true),  Ok(1));
   assert_matches!(ser.encode_bool(false), Ok(1));
   assert_eq!(100, ser.len());
   assert_eq!([0x01, 0x00], &ser.as_slice()[0..2]);
}

#[test]
fn test_serializer_size() {
   let mut ser = SizeBitcoinSerializer::new();
   assert_eq!(0, ser.size());
   assert_matches!(ser.encode_bool(true),  Ok(1));
   assert_matches!(ser.encode_bool(false), Ok(1));
   assert_eq!(2, ser.size());
}

#[test]
fn test_serializer_hash() {
   let mut ser = DHash256BitcoinSerializer::new();
   assert_matches!(ser.encode_bool(true),  Ok(1));
   assert_matches!(ser.encode_bool(false), Ok(1));
   assert_eq!("677b2d718464ee0121475600b929c0b4155667486577d1320b18c2dc7d4b4f99", ser.hash_hexresult());
}

#[test]
fn test_varint() {
   let mut ser = FixedBitcoinSerializer::new(100);

   assert_matches!(ser.encode_varint(0u64), Ok(1));
   assert_matches!(ser.encode_varint(252u64), Ok(1));
   assert_eq!([0, 252], &ser.as_slice()[0..2]);

   ser.rewind();
   assert_matches!(ser.encode_varint(253u64), Ok(3));    //lower limit
   assert_matches!(ser.encode_varint(0x0102u64), Ok(3)); //endian test
   assert_matches!(ser.encode_varint(0xFFFFu64), Ok(3)); //higher limit
   assert_eq!([253, 253, 0, 253, 0x02, 0x01, 253, 0xFF, 0xFF], &ser.as_slice()[0..9]);

   ser.rewind();
   assert_matches!(ser.encode_varint(0x10000u64), Ok(5));
   assert_matches!(ser.encode_varint(0x01020304u64), Ok(5));
   assert_matches!(ser.encode_varint(0xFFFFFFFFu64), Ok(5));
   assert_eq!([254, 0x00, 0x00, 0x01, 0x00,
               254, 0x04, 0x03, 0x02, 0x01,
               254, 0xFF, 0xFF, 0xFF, 0xFF], &ser.as_slice()[0..15]);
   ser.rewind();
   assert_matches!(ser.encode_varint(0x100000000u64), Ok(9));
   assert_matches!(ser.encode_varint(0x0102030405060708u64), Ok(9));
   assert_matches!(ser.encode_varint(0xFFFFFFFFFFFFFFFFu64), Ok(9));
   assert_eq!([255, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
               255, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01,
               255, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF], &ser.as_slice()[0..27]);
}

 */