
use jsonrpc_core::{Error, ErrorCode};
use serde::{Deserialize, Serialize};
use sp_core::{hashing::keccak_256, H256};

pub trait AbstractMerkleTree {
    fn generate_tree_root(leaves_data: Vec<Vec<u8>>) -> Result<H256, Error>;
    fn gnereate_merkle_path(leaf: &Vec<u8>, leaves_data: Vec<Vec<u8>>) -> Result<Vec<H256>, Error>;
    fn merkle_proof(leaf: Vec<u8>, leaves_data: Vec<Vec<u8>>) -> Result<MerklePathData, Error>;
}

// New implementation of merkle_tree
pub mod balanced_merkle_tree;

#[derive(Clone, Serialize, Deserialize, Debug)]
// Contains all the data required to prove that `encoded_leaf` is part of a merkle tree.
pub struct MerklePathData {
    // abi encoded leaf which can be decoded by Ethereum
    pub encoded_leaf: EncodedLeafData,
    // Merkle path to prove the inclusion of the `encoded_leaf` in a merkle tree
    pub merkle_path: Vec<H256>,
}

// Vector of bytes that represents abi encoded leaf data
pub type EncodedLeafData = Vec<u8>;
