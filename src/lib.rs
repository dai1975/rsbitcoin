#![feature(core_intrinsics)]
#![feature(associated_consts)]
#![feature(associated_type_defaults)]
#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(specialization)]
#![feature(slice_patterns)]
#![feature(advanced_slice_patterns)]
#![feature(trace_macros)]
#![feature(slice_concat_ext)]
#![feature(try_from)]
#![feature(range_contains)]

#![feature(plugin)]
#![plugin(interpolate_idents)]

#[macro_use] extern crate assert_matches;
#[macro_use] extern crate lazy_static;
extern crate secp256k1;

#[macro_use]
pub mod error;
pub use self::error::{Error, GenericError};
pub type Result<T> = ::std::result::Result<T, ::Error>; 

#[macro_use]
pub mod codec;

pub mod uint256;
pub use self::uint256::UInt256;

pub mod display;

pub mod tx;
pub use self::tx::{ OutPoint, TxIn, Amount, TxOut, LockTime, Transaction };

pub mod block;
pub use self::block::{ PartialMerkleTree, MerkleBlock, BlockHeader, Block, BlockLocator };

pub mod chain;
pub use self::chain::consensus_params::ConsensusParams;
pub use self::chain::chain_params::ChainParams;

pub mod apriori;
pub use self::apriori::network::Network;
pub use self::apriori::network::{MAIN_PARAMS, TESTNET_PARAMS, REGTEST_PARAMS};
pub use self::apriori::network::{get_chain_params_by_id, get_chain_params_by_name};

pub mod crypto;

#[macro_use]
pub mod script;
pub use self::script::{Script};

pub mod protocol;

#[test]
fn apriori_test() {
   assert!(::MAIN_PARAMS.name == ::get_chain_params_by_name("main").unwrap().name);
}


