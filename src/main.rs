use crypto::digest::Digest;
use crypto::sha2::Sha256;

#[derive(Debug)]
struct MerkleTree {
    root: Option<Box<MerkleNode>>,
}

#[derive(Debug, Clone)]
struct MerkleNode {
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>,
    hash: Vec<u8>,
}

impl MerkleTree {
    fn new(datas: &[Vec<u8>]) -> Self {
        let leaf_nodes = datas
            .iter()
            .map(|data| Self::create_new_data_node(data))
            .collect::<Vec<MerkleNode>>();

        let root = Self::build_tree(&leaf_nodes);
        MerkleTree { root }
    }

    fn build_tree(nodes: &[MerkleNode]) -> Option<Box<MerkleNode>> {
        if nodes.is_empty() {
            return None;
        }
        if nodes.len() == 1 {
            return Some(Box::new(nodes[0].clone()));
        }

        let mid_node = nodes.len() / 2;
        let left_child = Self::build_tree(&nodes[..mid_node]);
        let right_child = Self::build_tree(&nodes[mid_node..]);

        let datas = [
            &left_child.as_ref().unwrap().hash,
            &right_child.as_ref().unwrap().hash,
        ];
        let hash = Self::sha256_hasher(&datas);

        Some(Box::new(MerkleNode {
            left: left_child,
            right: right_child,
            hash,
        }))
    }

    fn insert(&mut self, data: &Vec<u8>) {
        let new_node = Self::create_new_data_node(data);
        let current_root = self.root.take();

        self.root = self.insert_node(new_node, current_root);
    }

    fn insert_node(
        &self,
        new_node: MerkleNode,
        current_root: Option<Box<MerkleNode>>,
    ) -> Option<Box<MerkleNode>> {
        match current_root {
            None => Some(Box::new(new_node)),
            Some(mut node) => {
                if node.left.is_none() && node.right.is_none() {
                    let datas = [&node.hash, &new_node.hash];
                    let hash = Self::sha256_hasher(&datas);

                    return Some(Box::new(MerkleNode {
                        left: Some(node),
                        right: Some(Box::new(new_node)),
                        hash,
                    }));
                } else {
                    let child_side = if node.left.is_some() {
                        &mut node.left
                    } else {
                        &mut node.right
                    };
                    *child_side = self.insert_node(new_node, child_side.take());
                    let datas = [
                        &node.left.as_ref().unwrap().hash,
                        &node.right.as_ref().unwrap().hash,
                    ];
                    let hash = Self::sha256_hasher(&datas);
                    return Some(Box::new(MerkleNode {
                        left: node.left,
                        right: node.right,
                        hash,
                    }));
                }
            }
        }
    }

    fn contains(&self, data: &Vec<u8>) -> bool {
        let data_hash = Self::sha256_hasher(&[data]);
        self.contains_hash(&self.root, &data_hash)
        
    }

    fn contains_hash(&self, node: &Option<Box<MerkleNode>>, data_hash: &Vec<u8>) -> bool {
        match node {
            None => false,
            Some(n) => {
                if &n.hash == data_hash {
                    return true;
                } else {
                    let in_left_node = self.contains_hash(&n.left, &data_hash);
                    let in_right_node = self.contains_hash(&n.right, &data_hash);
                    return in_left_node || in_right_node
                }
            }
        }
    }

    fn sha256_hasher(datas: &[&Vec<u8>]) -> Vec<u8> {
        let mut hasher = Sha256::new();
        for data in datas.into_iter() {
            hasher.input(data)
        }
        hasher.result_str().as_bytes().to_vec()
    }

    fn create_new_data_node(data: &Vec<u8>) -> MerkleNode {
        let hash = Self::sha256_hasher(&[data]);
        MerkleNode {
            left: None,
            right: None,
            hash,
        }
    }
}

fn main() {
    let data = vec![
        "hello".as_bytes().to_vec(),
        "world".as_bytes().to_vec(),
        "whatsup".as_bytes().to_vec(),
        "merkle".as_bytes().to_vec(),
    ];
    let mut merkle_tree = MerkleTree::new(&data);
    // dbg!(&merkle_tree.root.unwrap().right);
    let new_data = "tree".as_bytes().to_vec();
    merkle_tree.insert(&new_data);
    
    let is_present = merkle_tree.contains(&"hello".as_bytes().to_vec());
    dbg!(is_present);
}
