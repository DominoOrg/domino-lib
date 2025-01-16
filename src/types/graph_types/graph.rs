use std::collections::HashMap;

use super::{Arc, Node};

#[derive(Debug, Default, Clone)]
pub(crate) struct Graph {
    pub(crate) nodes: Vec<Node>,
    pub(crate) adjacency: HashMap<Node, Vec<Arc>>
}

impl Graph {
    pub fn regular(n: usize) -> Self {
        Graph {
            nodes: (0..(n+1) as i32).into_iter().map(|node| Node::Regular(node)).collect::<Vec<Node>>(),
            adjacency: (0..(n+1) as i32)
                .map(|source_node| {
                    let edges = (0..(n+1)).filter_map(|destination_node| {
                        if n % 2 == 0 || (source_node as f32 - destination_node as f32).abs() as i32 != ((n as i32 + 1)/ 2) {
                            Some(Arc {
                                source: source_node.into(),
                                destination: (destination_node as i32).into(),
                                position: None
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Arc>>();
                    (Node::Regular(source_node), edges)
                })
                .collect()
        }
    }

}