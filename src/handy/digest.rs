use ::crypto::{
   Digest, DigestExt,
   Sha1, Sha256, Ripemd160, DHash256, Hash160
};

pub struct HandyDigest<T:DigestExt>(T);

impl <T:DigestExt> HandyDigest<T> {
   #[inline] pub fn output_bits(&self)  -> usize { self.0.output_bits() }
   #[inline] pub fn output_bytes(&self) -> usize { self.0.output_bytes() }
   #[inline] pub fn block_size(&self)   -> usize { self.0.block_size() }
   
   #[inline] pub fn input(&mut self, input: &[u8]) { self.0.input(input) }
   #[inline] pub fn result(&mut self, out: &mut [u8]) { self.0.result(out) }
   #[inline] pub fn reset(&mut self) { self.0.reset() }
   
   #[inline] pub fn input_hex(&mut self, input: &str) { self.0.input_hex(input) }
   #[inline] pub fn result_box(&mut self) -> Box<[u8]> { self.0.result_box() }
   #[inline] pub fn result_hex(&mut self) -> String { self.0.result_hex() }
   #[inline] pub fn u8_to_box(&mut self, input: &[u8]) -> Box<[u8]> { self.0.u8_to_box(input) }
   #[inline] pub fn u8_to_hex(&mut self, input: &[u8]) -> String { self.0.u8_to_hex(input) }
   #[inline] pub fn hex_to_box(&mut self, input: &str) -> Box<[u8]> { self.0.hex_to_box(input) }
   #[inline] pub fn hex_to_hex(&mut self, input: &str) -> String { self.0.hex_to_hex(input) }
}

pub struct Factory();

macro_rules! deffn {
   ($fname:ident, $t:ident) => {
      pub fn $fname(&self) -> HandyDigest<$t> { HandyDigest::<$t>($t::new()) }
   }
}
impl Factory {
   deffn! { create_sha1,      Sha1 }
   deffn! { create_sha256,    Sha256 }
   deffn! { create_ripemd160, Ripemd160 }
   deffn! { create_dhash256,  DHash256 }
   deffn! { create_hash160,   Hash160 }
   /*
   pub fn create_sha1(&self)      -> HandyDigest<Sha1>      { HandyDigest<Sha1>(Sha1::new()) }
   pub fn create_sha256(&self)    -> HandyDigest<Sha256>    { HandyDigest<Sha256>(Sha256::new()) }
   pub fn create_ripemd160(&self) -> HandyDigest<Ripemd160> { HandyDigest<Ripemd160>(Ripemd160::new()) }
   pub fn create_dhash256(&self)  -> HandyDigest<DHash256>  { HandyDigest<DHash256>(DHash256::new()) }
   pub fn create_hash160(&self)   -> HandyDigest<Hash160>   { HandyDigest<Hash160>(Hash160::new()) }
    */
   pub fn create(&self, name:&str) -> Box<Digest> {
      match name {
         "sha1"      => Box::new(Sha1::new()),
         "sha256"    => Box::new(Sha256::new()),
         "ripemd160" => Box::new(Ripemd160::new()),
         "dhash256"  => Box::new(DHash256::new()),
         "hash160"   => Box::new(Hash160::new()),
         _ => panic!(format!("unknown algorithm: {}", name)),
      }
   }
}

lazy_static! {
   pub static ref DIGEST: Factory = {
      Factory()
   };
}

