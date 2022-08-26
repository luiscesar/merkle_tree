// A Merkle Tree is a concept often used in Blockchains.
// It is a binary tree where each leaf node represents the hash of some interesting data
// and each internal node is the hash of the concatenated contents of its two children.
// Merkle Trees often record groups of transactions, and the roots are published widely to
// serve as summaries of all recognised transactions on a given date.

// By construction, the tree's root is a hash of all its leaves organised in a specific order.
// Since hash functions are hard to reverse, it is unfeasible to create a tree with a specific
// root if we don't know the inputs to it.
// We can prove a transaction happened before a certain date by showing it was a leaf of Merkle Tree
// that has already been published.
// Merkle Trees provide an efficient way to prove this inclusion in the tree.
// It is enough to show a path of neighbour-nodes from the leaf to the root.
// That is, a list that includes the sibling of the leaf node, then the sibling of its parent and so on
// until the root is reached.

// The code in this file represents a library that can create Merkle trees or return a proof of inclusion
// of a given leaf in a Merkle tree.
// It also includes tests to demonstrate how to use the library functions.

// EXERCISE:

// Your goal in this exercise is two-fold:
// 1. Imagine you receive this code in a Github Pull-Request submitted by one of your team mates.
// Write a code review for it with comments as you see fit.
//
// 2. Improve the code if you are able. Ensure it builds and runs, and that tests pass.

// New: Tests in separated module
#[cfg(test)]
mod tests;

// ----------------- Library code ----------------------------

use jsonrpc_core::{Error, ErrorCode};
use serde::{Deserialize, Serialize};
use sp_core::{hashing::keccak_256, H256};
use std::fs;

// New: ADT
pub trait AbstractMerkleTree {
    fn generate_tree_root(leaves_data: Vec<Vec<u8>>) -> Result<H256, Error>;
    fn gnereate_merkle_path(leaf: &Vec<u8>, leaves_data: Vec<Vec<u8>>) -> Result<Vec<H256>, Error>;
    // New: Possible new function for the abstract data type
    fn merkle_proof(leaf: Vec<u8>, leaves_data: Vec<Vec<u8>>) -> Result<MerklePathData, Error>;
}

// New: Concrete type
pub struct MerkleTree;

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

// New: Implementation of ADT by concrete type
impl AbstractMerkleTree for MerkleTree {
    // Generates a merkle tree and returns the root hash
    fn generate_tree_root(leaves_data: Vec<Vec<u8>>) -> Result<H256, Error> {
        let mut nodes_hashes: Vec<H256> = leaves_data
            .into_iter()
            .map(|data| H256::from_slice(&keccak_256(&data)))
            .collect::<Vec<H256>>();

        let result = MerkleTree::recurse(&mut nodes_hashes);

        if result.len() != 1 {
            return Err(Error {
                code: ErrorCode::ServerError(1),
                message: "Error generating merkle root".into(),
                data: None,
            });
        }

        return Ok(result[0]);
    }

    // Generates a merkle tree using `leaves_data` and returns the path from the specified `leaf_data` to the root
    fn gnereate_merkle_path(leaf: &Vec<u8>, leaves_data: Vec<Vec<u8>>) -> Result<Vec<H256>, Error> {
        let mut merkle_path: Vec<H256> = vec![];

        if leaf.is_empty() {
            return Err(Error {
                code: ErrorCode::ServerError(2),
                message: "Error generating merkle path".into(),
                data: None,
            });
        }

        if leaves_data.is_empty() {
            return Err(Error {
                code: ErrorCode::ServerError(3),
                message: "Error generating merkle path".into(),
                data: None,
            });
        }

        let mut node_hash_in_leaf_branch = H256::from_slice(&keccak_256(leaf));
        let nodes_hashes = &leaves_data
            .into_iter()
            .map(|data| H256::from_slice(&keccak_256(&data)))
            .collect::<Vec<H256>>();

        MerkleTree::recruse_for_path(
            &mut node_hash_in_leaf_branch,
            nodes_hashes,
            &mut merkle_path,
        );

        return Ok(merkle_path);
    }

    fn merkle_proof(
        encoded_leaf: Vec<u8>,
        leaves_data: Vec<Vec<u8>>,
    ) -> Result<MerklePathData, Error> {
        let merkle_path = MerkleTree::gnereate_merkle_path(&encoded_leaf, leaves_data)?;
        Ok(MerklePathData {
            encoded_leaf,
            merkle_path,
        })
    }
}

// New: Associated functions of concrete type
impl MerkleTree {

    // New: Loop instead of recursive call
    // Keys:
    //  N: number of nodes
    //  i: index of sum
    // Sum_i=1^log(N) O(N/2^(i+1)) = O(N)
    fn recurse(nodes: &mut Vec<H256>) -> Vec<H256> {
        let mut processed_nodes = MerkleTree::process_nodes_in_pairs(nodes);

        while processed_nodes.len() > 1 {
            processed_nodes = MerkleTree::process_nodes_in_pairs(&mut processed_nodes);
        }

        return processed_nodes;
    }

    // Keys: N - number of nodes
    // O(N/2)
    fn process_nodes_in_pairs(nodes: &mut Vec<H256>) -> Vec<H256> {
        let mut processed_nodes: Vec<H256> = vec![];
        for index in 0..nodes.len() / 2 {
            let left_node = nodes[2 * index];
            let right_node = nodes[2 * index + 1];

            let temp: Vec<H256>;
            if left_node <= right_node {
                temp = vec![left_node, right_node];
            } else {
                temp = vec![right_node, left_node];
            }

            let node = temp
                .into_iter()
                .map(|h| h.to_fixed_bytes().to_vec())
                .flatten()
                .collect::<Vec<u8>>();
            processed_nodes.push(H256::from_slice(&keccak_256(&node)));
        }

        if nodes.len() % 2 == 1 {
            processed_nodes.push(*nodes.last().unwrap());
        }

        return processed_nodes;
    }

    // New: Loop instead of recursive call
    fn recruse_for_path(
        node_hash_in_leaf_branch: &mut H256,
        nodes: &Vec<H256>,
        merkle_path: &mut Vec<H256>,
    ) -> Vec<H256> {
        let mut processed_nodes = MerkleTree::process_nodes_in_pairs_for_path(
            node_hash_in_leaf_branch,
            nodes,
            merkle_path,
        );

        while processed_nodes.len() > 1 {
            processed_nodes = MerkleTree::process_nodes_in_pairs_for_path(
                node_hash_in_leaf_branch,
                &processed_nodes,
                merkle_path,
            );
        }

        return processed_nodes;
    }

    fn process_nodes_in_pairs_for_path(
        node_hash_in_leaf_branch: &mut H256,
        nodes: &Vec<H256>,
        merkle_path: &mut Vec<H256>,
    ) -> Vec<H256> {
        let mut processed_nodes: Vec<H256> = vec![];
        for index in 0..nodes.len() / 2 {
            let left_node = nodes[2 * index];
            let right_node = nodes[2 * index + 1];

            let temp: Vec<H256>;
            if left_node <= right_node {
                temp = vec![left_node, right_node];
            } else {
                temp = vec![right_node, left_node];
            }

            let node = temp
                .into_iter()
                .map(|h| h.to_fixed_bytes().to_vec())
                .flatten()
                .collect::<Vec<u8>>();
            let node_hash = H256::from_slice(&keccak_256(&node));
            if *node_hash_in_leaf_branch == left_node || *node_hash_in_leaf_branch == right_node {
                if *node_hash_in_leaf_branch == left_node {
                    //merkle_path.insert(0, right_node);
                    // New: push instead of insert
                    merkle_path.push(right_node);
                } else {
                    //merkle_path.insert(0, left_node);
                    // New: push instead of insert
                    merkle_path.push(left_node);
                }
                *node_hash_in_leaf_branch = node_hash;
            }
            processed_nodes.push(node_hash);
        }

        if nodes.len() % 2 == 1 {
            processed_nodes.push(*nodes.last().unwrap());
        }

        return processed_nodes;
    }
}
