use super::{AbstractMerkleTree, MerklePathData};
use jsonrpc_core::{Error, ErrorCode};
use sp_core::{hashing::keccak_256, H256};

pub struct BalancedMerkleTree;

impl AbstractMerkleTree for BalancedMerkleTree {
    // Generates a merkle tree and returns the root hash
    fn generate_tree_root(leaves_data: Vec<Vec<u8>>) -> Result<H256, Error> {
        let mut nodes_hashes: Vec<H256> = leaves_data
            .into_iter()
            .map(|data| H256::from_slice(&keccak_256(&data)))
            .collect::<Vec<H256>>();

        let result = BalancedMerkleTree::recurse(&mut nodes_hashes);

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

        BalancedMerkleTree::recruse_for_path(
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
        let merkle_path = BalancedMerkleTree::gnereate_merkle_path(&encoded_leaf, leaves_data)?;
        Ok(MerklePathData {
            encoded_leaf,
            merkle_path,
        })
    }
}

impl BalancedMerkleTree {
    fn recurse(nodes: &mut Vec<H256>) -> Vec<H256> {
        if nodes.len() == 0 {
            return vec![];
        }
        let number_last_level_leaves = get_number_last_level_leaves(nodes.len());

        let mut processed_nodes =
            BalancedMerkleTree::process_nodes_in_pairs(&nodes[0..number_last_level_leaves]);

        processed_nodes.extend_from_slice(&nodes[number_last_level_leaves..]);

        while processed_nodes.len() > 1 {
            processed_nodes = BalancedMerkleTree::process_nodes_in_pairs(&mut processed_nodes);
        }

        return processed_nodes;
    }

    fn process_nodes_in_pairs(nodes: &[H256]) -> Vec<H256> {
        let mut processed_nodes: Vec<H256> = vec![];
        if nodes.len() == 1 {
            processed_nodes.push(nodes[0]);
            return processed_nodes;
        }
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

        return processed_nodes;
    }

    fn recruse_for_path(
        node_hash_in_leaf_branch: &mut H256,
        nodes: &Vec<H256>,
        merkle_path: &mut Vec<H256>,
    ) -> Vec<H256> {
        if nodes.len() == 0 {
            return vec![];
        }
        let number_last_level_leaves = get_number_last_level_leaves(nodes.len());

        let mut processed_nodes = BalancedMerkleTree::process_nodes_in_pairs_for_path(
            node_hash_in_leaf_branch,
            &nodes[0..number_last_level_leaves],
            merkle_path,
        );

        processed_nodes.extend_from_slice(&nodes[number_last_level_leaves..]);

        while processed_nodes.len() > 1 {
            processed_nodes = BalancedMerkleTree::process_nodes_in_pairs_for_path(
                node_hash_in_leaf_branch,
                &processed_nodes,
                merkle_path,
            );
        }

        return processed_nodes;
    }

    fn process_nodes_in_pairs_for_path(
        node_hash_in_leaf_branch: &mut H256,
        nodes: &[H256],
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
                    merkle_path.push(right_node);
                } else {
                    merkle_path.push(left_node);
                }
                *node_hash_in_leaf_branch = node_hash;
            }
            processed_nodes.push(node_hash);
        }

        return processed_nodes;
    }
}

fn get_number_last_level_leaves(n: usize) -> usize {
    let t = f64::log2(n as f64) as usize;
    if n == usize::pow(2, t as u32) {
        n
    } else {
        let m = 2 * n - usize::pow(2, (t + 1) as u32);
        m
    }
}

#[cfg(test)]
mod tests;
