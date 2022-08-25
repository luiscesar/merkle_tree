use std::cell::RefCell;
use std::rc::Rc;
use std::time::Instant;

use primitive_types::H256;

use super::MerkleTree;
use super::MerkleTreeADT;


// ----------------- Tests -----------------------------------
#[test]
fn test_generate_root_1() {
    let mock_trees_roots = [
        "bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a",
        "b2521d64679bc4720dabfbae7ce17947a5d373d987d3b0cc1e3042ba2054da4a",
        "d359d2743bb3a93ded4c902716931497ae3080f478c14e7af96344a92e9ddd51",
        "fecce4ac8ed6fc57f4d880d6af2b443418d564df8f5d52c6782e952564ed79eb",
        "11aeafa56c9b34805cc86b1c320c9331672c07e600f0a44317051cfa05a0c296",
        "ce24ba488022147ace7a2a962b481707002c079d7c7ca85b108f7489aaedabba",
        "49b36fbd8a6e3a5ea292f621a38d0afa8ac580c56090a9b0d93e0d06b37d1a89",
        "49c6f5244cba156c2170135c98a77f6fc9b812eb201aefcd3e32c38dfcec711a",
        "f54e6007f25df4d2c75a2ec526e4a635dac09b622497862f6062f9340f25ca81",
        "29da8f3f81c6c9dc74665e28dcbfc1645629746613cccbd76c3f8ccd6b1488ae",
    ];

    for x in 1..=mock_trees_roots.len() {
        let nodes = get_n_nodes(x as u32);
        println!("Running test {x}");
        assert_eq!(
            MerkleTree::generate_tree_root(nodes).unwrap(),
            H256(to_array(
                32,
                hex::decode(mock_trees_roots[(x - 1) as usize])
                    .unwrap()
                    .as_slice()
            ))
        );
    }
}

#[test]
fn test_generate_root_2() {
    assert!(MerkleTree::generate_tree_root(get_n_nodes(0)).is_err());
}

#[test]
fn test_generate_root_3() {
    // resistance test
    let now = Instant::now();
    let longTest = get_n_nodes(1000000);
    println!("Number of nodes: {}", longTest.len());
    println!("Creating leaves: {}", now.elapsed().as_millis());

    assert!(!MerkleTree::generate_tree_root(longTest).is_err());
    println!("Building tree: {}", now.elapsed().as_millis());
}

#[test]
fn test_generate_path_1() {
    let mocked_merkle_paths: Vec<(u8, usize, Vec<H256>)> = vec![
        // Mocked one node tree data [0]
        (1, 0, vec![]),
        // Mocked two nodes tree data [0, 1]
        (2, 0, vec![H256(to_array(32, hex::decode("5fe7f977e71dba2ea1a68e21057beebb9be2ac30c6410aa38d4f3fbe41dcffd2").unwrap().as_slice()))]),
        (2, 1, vec![H256(to_array(32, hex::decode("bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a").unwrap().as_slice()))]),
        // Mocked three nodes tree data [0, 1, 2]
        (3, 0, vec![H256(to_array(32, hex::decode("5fe7f977e71dba2ea1a68e21057beebb9be2ac30c6410aa38d4f3fbe41dcffd2").unwrap().as_slice())),
            H256(to_array(32, hex::decode("f2ee15ea639b73fa3db9b34a245bdfa015c260c598b211bf05a1ecc4b3e3b4f2").unwrap().as_slice()))
        ]),
        (3, 1, vec![H256(to_array(32, hex::decode("bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a").unwrap().as_slice())),H256(to_array(32, hex::decode("f2ee15ea639b73fa3db9b34a245bdfa015c260c598b211bf05a1ecc4b3e3b4f2").unwrap().as_slice()))
        ]),
        (3, 2, vec![H256(to_array(32, hex::decode("b2521d64679bc4720dabfbae7ce17947a5d373d987d3b0cc1e3042ba2054da4a").unwrap().as_slice()))]),
        // Mocked five nodes tree data [0, 1, 2, 3, 4]
        (5, 0, vec![H256(to_array(32, hex::decode("5fe7f977e71dba2ea1a68e21057beebb9be2ac30c6410aa38d4f3fbe41dcffd2").unwrap().as_slice())),H256(to_array(32, hex::decode("c144ad52449a5832e51e7d4daca4c86a9aafc33d89ef15ff7908956d0edb977d").unwrap().as_slice())),H256(to_array(32, hex::decode("f343681465b9efe82c933c3e8748c70cb8aa06539c361de20f72eac04e766393").unwrap().as_slice()))
        ]),
        (5, 1, vec![H256(to_array(32, hex::decode("bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a").unwrap().as_slice())),
            H256(to_array(32, hex::decode("c144ad52449a5832e51e7d4daca4c86a9aafc33d89ef15ff7908956d0edb977d").unwrap().as_slice())), H256(to_array(32, hex::decode("f343681465b9efe82c933c3e8748c70cb8aa06539c361de20f72eac04e766393").unwrap().as_slice()))
        ]),
        (5, 2, vec![H256(to_array(32, hex::decode("69c322e3248a5dfc29d73c5b0553b0185a35cd5bb6386747517ef7e53b15e287").unwrap().as_slice())),
            H256(to_array(32, hex::decode("b2521d64679bc4720dabfbae7ce17947a5d373d987d3b0cc1e3042ba2054da4a").unwrap().as_slice())), H256(to_array(32, hex::decode("f343681465b9efe82c933c3e8748c70cb8aa06539c361de20f72eac04e766393").unwrap().as_slice()))
        ]),
        (5, 3, vec![H256(to_array(32, hex::decode("f2ee15ea639b73fa3db9b34a245bdfa015c260c598b211bf05a1ecc4b3e3b4f2").unwrap().as_slice())),
            H256(to_array(32, hex::decode("b2521d64679bc4720dabfbae7ce17947a5d373d987d3b0cc1e3042ba2054da4a").unwrap().as_slice())), H256(to_array(32, hex::decode("f343681465b9efe82c933c3e8748c70cb8aa06539c361de20f72eac04e766393").unwrap().as_slice()))
        ]),
        (5, 4, vec![H256(to_array(32, hex::decode("fecce4ac8ed6fc57f4d880d6af2b443418d564df8f5d52c6782e952564ed79eb").unwrap().as_slice()))
        ])
    ];

    for (n, i, paht) in mocked_merkle_paths {
        let nodes = get_n_nodes(n as u32);
        println!("test case {}", format!("({}, {}, {:?})", n, i, paht));
        let computed_path = MerkleTree::gnereate_merkle_path(&nodes[i].clone(), nodes).unwrap();
        println!("computed_path {:?}", computed_path);
        assert_eq!(computed_path, paht);
    }
}

#[test]
fn test_generate_path_2() {
    assert!(MerkleTree::gnereate_merkle_path(&vec![0], get_n_nodes(0)).is_err());
}

#[test]
fn test_generate_path_3() {
    assert!(MerkleTree::gnereate_merkle_path(&vec![], get_n_nodes(1)).is_err());
}

#[test]
fn test_merkle_proof_1() {
    let mocked_merkle_paths: Vec<(u8, usize, Vec<H256>)> = vec![
        // Mocked one node tree data [0]
        (1, 0, vec![]),
        // Mocked two nodes tree data [0, 1]
        (2, 0, vec![H256(to_array(32, hex::decode("5fe7f977e71dba2ea1a68e21057beebb9be2ac30c6410aa38d4f3fbe41dcffd2").unwrap().as_slice()))]),
        (2, 1, vec![H256(to_array(32, hex::decode("bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a").unwrap().as_slice()))]),
        // Mocked three nodes tree data [0, 1, 2]
        (3, 0, vec![H256(to_array(32, hex::decode("5fe7f977e71dba2ea1a68e21057beebb9be2ac30c6410aa38d4f3fbe41dcffd2").unwrap().as_slice())),
            H256(to_array(32, hex::decode("f2ee15ea639b73fa3db9b34a245bdfa015c260c598b211bf05a1ecc4b3e3b4f2").unwrap().as_slice()))
        ]),
        (3, 1, vec![H256(to_array(32, hex::decode("bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a").unwrap().as_slice())),H256(to_array(32, hex::decode("f2ee15ea639b73fa3db9b34a245bdfa015c260c598b211bf05a1ecc4b3e3b4f2").unwrap().as_slice()))
        ]),
        (3, 2, vec![H256(to_array(32, hex::decode("b2521d64679bc4720dabfbae7ce17947a5d373d987d3b0cc1e3042ba2054da4a").unwrap().as_slice()))]),
        // Mocked five nodes tree data [0, 1, 2, 3, 4]
        (5, 0, vec![H256(to_array(32, hex::decode("5fe7f977e71dba2ea1a68e21057beebb9be2ac30c6410aa38d4f3fbe41dcffd2").unwrap().as_slice())),H256(to_array(32, hex::decode("c144ad52449a5832e51e7d4daca4c86a9aafc33d89ef15ff7908956d0edb977d").unwrap().as_slice())),H256(to_array(32, hex::decode("f343681465b9efe82c933c3e8748c70cb8aa06539c361de20f72eac04e766393").unwrap().as_slice()))
        ]),
        (5, 1, vec![H256(to_array(32, hex::decode("bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a").unwrap().as_slice())),
            H256(to_array(32, hex::decode("c144ad52449a5832e51e7d4daca4c86a9aafc33d89ef15ff7908956d0edb977d").unwrap().as_slice())), H256(to_array(32, hex::decode("f343681465b9efe82c933c3e8748c70cb8aa06539c361de20f72eac04e766393").unwrap().as_slice()))
        ]),
        (5, 2, vec![H256(to_array(32, hex::decode("69c322e3248a5dfc29d73c5b0553b0185a35cd5bb6386747517ef7e53b15e287").unwrap().as_slice())),
            H256(to_array(32, hex::decode("b2521d64679bc4720dabfbae7ce17947a5d373d987d3b0cc1e3042ba2054da4a").unwrap().as_slice())), H256(to_array(32, hex::decode("f343681465b9efe82c933c3e8748c70cb8aa06539c361de20f72eac04e766393").unwrap().as_slice()))
        ]),
        (5, 3, vec![H256(to_array(32, hex::decode("f2ee15ea639b73fa3db9b34a245bdfa015c260c598b211bf05a1ecc4b3e3b4f2").unwrap().as_slice())),
            H256(to_array(32, hex::decode("b2521d64679bc4720dabfbae7ce17947a5d373d987d3b0cc1e3042ba2054da4a").unwrap().as_slice())), H256(to_array(32, hex::decode("f343681465b9efe82c933c3e8748c70cb8aa06539c361de20f72eac04e766393").unwrap().as_slice()))
        ]),
        (5, 4, vec![H256(to_array(32, hex::decode("fecce4ac8ed6fc57f4d880d6af2b443418d564df8f5d52c6782e952564ed79eb").unwrap().as_slice()))
        ])
    ];

    for (n, i, paht) in mocked_merkle_paths {
        let nodes = get_n_nodes(n as u32);
        let leaf = nodes[i].clone();
        let merkle_path_data = 
            MerkleTree::merkle_proof(nodes[i].clone(), nodes).unwrap();
        assert_eq!(merkle_path_data.merkle_path, paht);
        assert_eq!(merkle_path_data.encoded_leaf, leaf);
    }
}

#[test]
fn test_merkle_proof_2() {
    assert!(MerkleTree::merkle_proof(vec![0], get_n_nodes(0)).is_err());
}

#[test]
fn test_merkle_proof_3() {
    assert!(MerkleTree::merkle_proof(vec![], get_n_nodes(1)).is_err());
}

fn get_n_nodes(n: u32) -> Vec<Vec<u8>> {
    let mut nodes: Vec<Vec<u8>> = vec![];
    for number in 0..n {
        nodes.push(vec![(number % 256) as u8]);
    }
    return nodes;
}

fn to_array(_size: u8, bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &bytes[..];
    array.copy_from_slice(bytes);
    array
}
