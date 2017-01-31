use super::{TxIn, TxOut, LockTime};

#[derive(Debug,Default,Clone)]
pub struct Transaction {
   pub version:  i32,
   pub ins:      Vec<TxIn>,
   pub outs:     Vec<TxOut>,
   pub locktime: LockTime,
}

const TRANSACTION_CURRENT_VERSION:i32 = 1i32;

impl Transaction {
   pub fn new() -> Self {
      Transaction {
         version:  TRANSACTION_CURRENT_VERSION,
         ins:      Vec::<TxIn>::new(), 
         outs:     Vec::<TxOut>::new(), 
         locktime: LockTime::NoLock,
      }
   }
   pub fn is_coin_base(&self) -> bool {
      self.ins.len() == 1 && self.ins[0].prevout.is_null()
   }
}

impl ::std::fmt::Display for Transaction {
   fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
      write!(f, "Tx(ver={}, ins={}, outs={}, locktime={:?})", self.version, self.ins.len(), self.outs.len(), self.locktime)
   }
}

use ::std::borrow::Borrow;
use ::codec::{EncodeStream, Encodee, DecodeStream, Decodee};
impl Encodee for Transaction {
   type P = ();
   fn encode<ES:EncodeStream, BP:Borrow<Self::P>>(&self, e:&mut ES, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(e.encode_i32le(self.version));
      let p = (::std::usize::MAX, ());
      r += try!(self.ins.encode(e, &p));
      r += try!(self.outs.encode(e, &p));
      r += try!(self.locktime.encode(e, ()));
      Ok(r)
   }
}
impl Decodee for Transaction {
   type P = ();
   fn decode<DS:DecodeStream, BP:Borrow<Self::P>>(&mut self, d:&mut DS, _p:BP) -> ::Result<usize> {
      let mut r:usize = 0;
      r += try!(d.decode_i32le(&mut self.version));
      let p = (::std::usize::MAX, ());
      r += try!(self.ins.decode(d, &p));
      r += try!(self.outs.decode(d, &p));
      r += try!(self.locktime.decode(d, ()));
      Ok(r)
   }
}

impl_dump! { Transaction, () }

#[test]
fn test_decode_transaction() {
   use ::codec::{WithBytes, ToBytes};
   let bytes = Vec::<u8>::with_hex("0100000002d8c8df6a6fdd2addaf589a83d860f18b44872d13ee6ec3526b2b470d42a96d4d000000008b483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7abffffffff2ab3fa4f68a512266134085d3260b94d3b6cfd351450cff021c045a69ba120b2000000008b4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226ffffffff02c0175302000000001976a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788acc01c3d09000000001976a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac00000000").unwrap();

   let tx = Transaction::with_bytes(bytes);
   assert_matches!(tx, Ok(_));
   let tx = tx.unwrap();
   assert_eq!(tx.version, 1);
   assert_eq!(tx.ins.len(), 2);
   assert_eq!(tx.outs.len(), 2);
   assert_eq!(tx.locktime.is_no_lock(), true);

   assert_eq!(tx.ins[0].prevout.txid.to_rhex().unwrap(), "4d6da9420d472b6b52c36eee132d87448bf160d8839a58afdd2add6f6adfc8d8");
   assert_eq!(tx.ins[0].prevout.n, 0);
   assert_eq!(tx.ins[0].is_sequence_final(), true);
   assert_eq!(tx.ins[0].script_sig.bytecode().to_hex().unwrap(), "483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab");

   assert_eq!(tx.ins[1].prevout.txid.to_rhex().unwrap(), "b220a19ba645c021f0cf501435fd6c3b4db960325d0834612612a5684ffab32a");
   assert_eq!(tx.ins[1].prevout.n, 0);
   assert_eq!(tx.ins[1].is_sequence_final(), true);
   assert_eq!(tx.ins[1].script_sig.bytecode().to_hex().unwrap(), "4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226");

   assert_eq!(tx.outs[0].value, 39000000);
   assert_eq!(tx.outs[0].script_pubkey.bytecode().to_hex().unwrap(), "76a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788ac");
   assert_eq!(tx.outs[1].value, 155000000);
   assert_eq!(tx.outs[1].script_pubkey.bytecode().to_hex().unwrap(), "76a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac");
}

#[test]
fn test_encode_transaction() {
   use ::script::Script;
   use ::codec::{WithBytes, ToBytes, ToDigest};
   use super::OutPoint;

   let mut tx = Transaction::new();
   tx.ins.push(TxIn {
      prevout: OutPoint { txid: ::UInt256::with_rhex("4d6da9420d472b6b52c36eee132d87448bf160d8839a58afdd2add6f6adfc8d8").unwrap(), n:0 },
      script_sig: Script::with_hex("483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7ab").unwrap(),
      sequence: 0xFFFFFFFFu32,
   } );
   tx.ins.push(TxIn {
      prevout: OutPoint { txid: ::UInt256::with_rhex("b220a19ba645c021f0cf501435fd6c3b4db960325d0834612612a5684ffab32a").unwrap(), n:0 },
      script_sig: Script::with_hex("4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226").unwrap(),
      sequence: 0xFFFFFFFFu32,
   } );
   tx.outs.push(TxOut {
      value: 39000000,
      script_pubkey: Script::with_hex("76a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788ac").unwrap(),
   });
   tx.outs.push(TxOut {
      value: 155000000,
      script_pubkey: Script::with_hex("76a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac").unwrap(),
   });

   assert_eq!(tx.to_hex().unwrap(), "0100000002d8c8df6a6fdd2addaf589a83d860f18b44872d13ee6ec3526b2b470d42a96d4d000000008b483045022100b31557e47191936cb14e013fb421b1860b5e4fd5d2bc5ec1938f4ffb1651dc8902202661c2920771fd29dd91cd4100cefb971269836da4914d970d333861819265ba014104c54f8ea9507f31a05ae325616e3024bd9878cb0a5dff780444002d731577be4e2e69c663ff2da922902a4454841aa1754c1b6292ad7d317150308d8cce0ad7abffffffff2ab3fa4f68a512266134085d3260b94d3b6cfd351450cff021c045a69ba120b2000000008b4830450220230110bc99ef311f1f8bda9d0d968bfe5dfa4af171adbef9ef71678d658823bf022100f956d4fcfa0995a578d84e7e913f9bb1cf5b5be1440bcede07bce9cd5b38115d014104c6ec27cffce0823c3fecb162dbd576c88dd7cda0b7b32b0961188a392b488c94ca174d833ee6a9b71c0996620ae71e799fc7c77901db147fa7d97732e49c8226ffffffff02c0175302000000001976a914a3d89c53bb956f08917b44d113c6b2bcbe0c29b788acc01c3d09000000001976a91408338e1d5e26db3fce21b011795b1c3c8a5a5d0788ac00000000");
   
   assert_eq!(tx.to_dhash256_rhex().unwrap(), "9021b49d445c719106c95d561b9c3fac7bcb3650db67684a9226cd7fa1e1c1a0");
}
