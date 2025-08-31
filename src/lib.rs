//! An implementation of a Huffman encoder that produces strings representing 
//! the binary Huffman codes assigned to characters based on their frequency
//! in the provided text. This isn't a practical compression library; it's a 
//! demonstration of how Huffman codes can be calculated.
//! 
//! Actual compression of text would take two passes. One pass is needed to get
//! the character frquencies, and another pass to convert characters to Huffman
//! codes.
//! 
//! How decompression could be imlemented isn't addressed by the code. A state
//! machine may be the most efficient way to convert a stream of binary values
//! back to characters.
//! 

use std::collections::HashMap;
use std::cmp::Ordering;

pub use heapq::*;


/// A handle to a `Node`. It holds an index in to the vector that holds the 
/// nodes.
/// 
#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
struct Handle(u16);

const HNONE: Handle = Handle(u16::MAX);

impl Handle {
    fn idx(&self) -> usize {
        self.0 as usize
    }
}


/// Represents the nodes of the Huffman tree used to generate the codes for
/// characters.
/// 
enum Node {
    Leaf   { char_: char, freq: usize },
    Branch { freq: usize, left: Handle, right: Handle }
}

impl Node {
    fn new_leaf(char_: char, freq: usize) -> Self {
        Node::Leaf{ char_, freq }
    }
    fn new_branch(freq: usize, left: Handle, right: Handle) -> Self {
        Node::Branch { freq, left, right }
    }
    fn freq(&self) -> usize {
        match self {
            Node::Leaf   { freq, .. } |
            Node::Branch { freq, .. } => *freq,
        }
    }
}


/// Holds all the nodes of the Huffman tree in continguous memory. This is a
/// cache efficent way to process them.
/// 
struct NodeMem {
    nodes: Vec<Node>,
}

impl NodeMem {
    fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    fn len(&self) -> usize {
        self.nodes.len()
    }
    fn new_leaf(&mut self, char_: char, freq: usize) -> Handle {
        self.nodes.push(Node::new_leaf(char_, freq));
        Handle(self.nodes.len() as u16 - 1)
    }
    fn new_branch(&mut self, freq: usize, left: Handle, right: Handle) 

        -> Handle 
    {
        self.nodes.push(Node::new_branch(freq, left, right));
        Handle(self.nodes.len() as u16 - 1)
    }
    fn h2node(&self, handle: Handle) -> &Node {
        &self.nodes[handle.idx()]
    }
    #[allow(dead_code)]
    fn h2node_mut(&mut self, handle: Handle) -> &mut Node {
        &mut self.nodes[handle.idx()]
    }
}


/// Create the initial leaf nodes that have the frequencies of each character.
/// 
fn create_freq_nodes(data: &str) -> NodeMem {
    let mut freqs = HashMap::new();
    let mut nodes = NodeMem::new();

    for b in data.chars() {
        *freqs.entry(b).or_insert(0) += 1;
    }
    for (c, f) in freqs {
        nodes.new_leaf(c, f);
    }
    nodes
}

/// Constructs the tree used to produce Huffman codes.
/// 
fn build_huffman_tree(nodes: &mut NodeMem) -> Handle {
    // `heap` holds instances of `Handle`, which are basically just indexes into
    // `nodes`.
    let mut heap = (0..nodes.len() as u16).map(Handle).collect::<Vec<_>>();

    fn cmp(a: &Handle, b: &Handle, nodes: &NodeMem) -> Ordering {
        nodes.h2node(*a).freq().cmp(&nodes.h2node(*b).freq())
    }

    heapify_with_aux(&mut heap, cmp, nodes);

    loop {
        match (heap_pop_with_aux(&mut heap, cmp, nodes), 
               heap_pop_with_aux(&mut heap, cmp, nodes)) {

            (Some(left), Some(right)) => {
                let freq = nodes.h2node(left).freq() 
                            + nodes.h2node(right).freq();
                let merged = nodes.new_branch(freq, left, right);

                heap_push_with_aux(&mut heap, merged, cmp, nodes);    
            },
            (Some(left), None) => {
                return left;
            },
            _ => { 
                return HNONE; 
            }
        }
    }
}

/// Generates human-readable binary strings representing Huffman codes. The
/// dictionary passed to `huff` will be updated with these codes. The dictionary
/// can then be printed and examined.
/// 
fn generate_huffman_codes_recurs(node  : Handle, 
                                 code  : &mut String,
                                 huff  : &mut HashMap<char, String>,
                                 nodes : &NodeMem) 
{
    if node != HNONE { 
        match nodes.h2node(node) {
            Node::Leaf { char_, .. } => {
                huff.insert(*char_, code.clone());
            },
            Node::Branch { left, right, .. } => {
                code.push('0');
                generate_huffman_codes_recurs(*left, code, huff, nodes);
                code.pop();

                code.push('1');
                generate_huffman_codes_recurs(*right, code, huff, nodes);
                code.pop();
            }
        }
    }
}

/// Generates a mapping of characters to string representations of their Huffman
/// codes.
/// 
pub fn generate_huffman_codes(data: &str) -> HashMap<char, String> {
    let mut nodes = create_freq_nodes(data);
    let     tree  = build_huffman_tree(&mut nodes);

    let mut huff = HashMap::new();
    let mut code = String::new();

    generate_huffman_codes_recurs(tree, &mut code, &mut huff, &nodes);

    huff
}


#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    #[test]
    fn melville() {
        let text = read_to_string("data/moby_dick.txt").unwrap();
        let huff = generate_huffman_codes(&text);

        println!("\nHUFFMAN CODE: {:?}\n", huff);

        let mut freq = HashMap::new();
        let mut compressed_size = 0;

        for ch in text.chars() {
            *freq.entry(ch).or_insert(0) += 1;
        }

        for (c, f) in freq {
            compressed_size += huff[&c].len() * f;
        }

        println!("\nCOMPRESSION RATIO: {}\n", 
                 text.len() as f32 * 8.0 / compressed_size as f32);
    }
}