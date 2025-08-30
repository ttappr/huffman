use std::collections::HashMap;
use std::cmp::Ordering;

pub use heapq::*;


#[derive(Clone, Copy, PartialEq)]
#[repr(transparent)]
struct Handle(u16);

const HNONE: Handle = Handle(u16::MAX);

impl Handle {
    fn idx(&self) -> usize {
        self.0 as usize
    }
}


struct Node {
    char_   : Option<char>,
    freq    : usize,
    left    : Handle,
    right   : Handle,
}

impl Node {
    fn new(char_: Option<char>, freq: usize, left: Handle, right: Handle) 

        -> Self 
    {
        Self { char_, freq, left, right }
    }
}


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
    fn new_node(&mut self, 
                char_ : Option<char>, 
                freq  : usize, 
                left  : Handle, 
                right : Handle) 

        -> Handle 
    {
        self.nodes.push(Node::new(char_, freq, left, right));
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


fn create_freq_nodes(data: &str) -> NodeMem {
    let mut freqs = HashMap::new();
    let mut nodes = NodeMem::new();

    for b in data.chars() {
        *freqs.entry(b).or_insert(0) += 1;
    }
    for (c, f) in freqs {
        nodes.new_node(Some(c), f, HNONE, HNONE);
    }
    nodes
}

fn build_huffman_tree(nodes: &mut NodeMem) -> Handle {
    // `heap` holds instances of `Handle`, which are basically just indexes into
    // `nodes`.
    let mut heap = (0..nodes.len() as u16).map(Handle).collect::<Vec<_>>();

    fn cmp(a: &Handle, b: &Handle, nodes: &NodeMem) -> Ordering {
        nodes.h2node(*a).freq.cmp(&nodes.h2node(*b).freq)
    }

    heapify_with_aux(&mut heap, cmp, nodes);

    loop {
        match (heap_pop_with_aux(&mut heap, cmp, nodes), 
               heap_pop_with_aux(&mut heap, cmp, nodes)) {

            (Some(left), Some(right)) => {
                let freq = nodes.h2node(left).freq + nodes.h2node(right).freq;
                let merged = nodes.new_node(None, freq, left, right);

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

fn generate_huffman_code(node  : Handle, 
                         code  : &mut String,
                         huff  : &mut HashMap<char, String>,
                         nodes : &NodeMem) 
{
    if node != HNONE {
        if let Some(c) = nodes.h2node(node).char_ {
            huff.insert(c, code.clone());
        }
        code.push('0');
        generate_huffman_code(nodes.h2node(node).left, code, huff, nodes);
        code.pop();

        code.push('1');
        generate_huffman_code(nodes.h2node(node).right, code, huff, nodes);
        code.pop();
    }
}

pub fn huffman_encoding(data: &str) -> HashMap<char, String> {
    let mut nodes = create_freq_nodes(data);
    let     tree  = build_huffman_tree(&mut nodes);

    let mut huff = HashMap::new();
    let mut code = String::new();

    generate_huffman_code(tree, &mut code, &mut huff, &nodes);

    huff
}


#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use super::*;

    #[test]
    fn melville() {
        let text = read_to_string("data/moby_dick.txt").unwrap();
        let huff = huffman_encoding(&text);

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