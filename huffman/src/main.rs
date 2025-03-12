use std::fs::File;
use std::io::{Write, Read};
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct HuffmanNode {
    frequency: usize,
    symbol: Option<u8>,
    left: Option<Box<HuffmanNode>>,
    right: Option<Box<HuffmanNode>>,
}

impl HuffmanNode {
    fn new(frequency: usize, symbol: Option<u8>) -> Self {
        HuffmanNode {
            frequency,
            symbol,
            left: None,
            right: None,
        }
    }
}

fn build_huffman_tree(frequencies: &[(u8, usize)]) -> Option<Box<HuffmanNode>> {
    let mut nodes: Vec<Box<HuffmanNode>> = frequencies
        .iter()
        .map(|&(symbol, frequency)| Box::new(HuffmanNode::new(frequency, Some(symbol))))
        .collect();

    while nodes.len() > 1 {
        nodes.sort_by_key(|n| n.frequency);
        let left = nodes.remove(0);
        let right = nodes.remove(0);
        let merged_frequency = left.frequency + right.frequency;
        let merged_node = Box::new(HuffmanNode {
            frequency: merged_frequency,
            symbol: None,
            left: Some(left),
            right: Some(right),
        });
        nodes.push(merged_node);
    }
    nodes.pop()
}

fn generate_codes(node: &Option<Box<HuffmanNode>>, prefix: String, codes: &mut BTreeMap<u8, String>) {
    if let Some(n) = node {
        if let Some(symbol) = n.symbol {
            codes.insert(symbol, prefix);
        } else {
            generate_codes(&n.left, format!("{}0", prefix), codes);
            generate_codes(&n.right, format!("{}1", prefix), codes);
        }
    }
}

fn calculate_frequencies(data: &[u8]) -> Vec<(u8, usize)> {
    let mut frequencies = BTreeMap::new();
    for &byte in data {
        *frequencies.entry(byte).or_insert(0) += 1;
    }
    frequencies.into_iter().collect()
}

fn serialize_tree(node: &Option<Box<HuffmanNode>>, output: &mut Vec<u8>) {
    if let Some(n) = node {
        if let Some(symbol) = n.symbol {
            output.push(1);
            output.push(symbol);
        } else {
            output.push(0);
            serialize_tree(&n.left, output);
            serialize_tree(&n.right, output);
        }
    }
}

fn encode_data(data: &[u8], codes: &BTreeMap<u8, String>) -> Vec<u8> {
    let bit_string: String = data.iter().map(|&b| codes.get(&b).unwrap().clone()).collect();
    let mut compressed_data = Vec::new();
    let mut byte = 0u8;
    let mut count = 0;
    
    for bit in bit_string.chars() {
        byte = (byte << 1) | (bit as u8 - b'0');
        count += 1;
        if count == 8 {
            compressed_data.push(byte);
            byte = 0;
            count = 0;
        }
    }
    
    if count > 0 {
        compressed_data.push(byte << (8 - count));
    }
    compressed_data
}

fn write_binary_file(filename: &str, tree_data: &[u8], data: &[u8]) {
    let mut file = File::create(filename).expect("Unable to create file");
    file.write_all(tree_data).expect("Unable to write tree data");
    file.write_all(data).expect("Unable to write compressed data");
}

fn read_binary_file(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).expect("Unable to read data");
    contents
}

fn deserialize_tree(data: &mut &[u8]) -> Option<Box<HuffmanNode>> {
    if data.is_empty() {
        return None;
    }

    let is_leaf = data[0];
    *data = &data[1..];

    if is_leaf == 1 {
        let symbol = data[0];
        *data = &data[1..];
        Some(Box::new(HuffmanNode::new(0, Some(symbol))))
    } else {
        let left = deserialize_tree(data);
        let right = deserialize_tree(data);
        Some(Box::new(HuffmanNode {
            frequency: 0,
            symbol: None,
            left,
            right,
        }))
    }
}

fn decode_data(compressed_data: &[u8], root: &Option<Box<HuffmanNode>>) -> Vec<u8> {
    let mut decoded_data = Vec::new();
    let mut current_node = root.clone();
    let mut bit_buffer = 0u8;
    let mut bit_count = 0;

    for byte in compressed_data {
        for i in (0..8).rev() {
            let bit = (byte >> i) & 1;
            bit_buffer = (bit_buffer << 1) | bit;
            bit_count += 1;

            if let Some(node) = &current_node {
                if let Some(symbol) = node.symbol {
                    decoded_data.push(symbol);
                    current_node = root.clone();
                } else {
                    current_node = if bit_buffer & (1 << (bit_count - 1)) != 0 {
                        node.right.clone()
                    } else {
                        node.left.clone()
                    };
                }
            }
            if bit_count == 8 {
                bit_count = 0;
                bit_buffer = 0;
            }
        }
    }
    decoded_data
}


fn main() {
    let input_filename = "input.txt";
    let data = read_binary_file(input_filename);
    println!("Original data size: {} bytes", data.len());

    let frequencies = calculate_frequencies(&data);
    let huffman_tree = build_huffman_tree(&frequencies);
    let mut codes = BTreeMap::new();
    generate_codes(&huffman_tree, String::new(), &mut codes);

    let mut tree_data = Vec::new();
    serialize_tree(&huffman_tree, &mut tree_data);

    let compressed_data = encode_data(&data, &codes);
    let compressed_filename = "compressed.bin";
    write_binary_file(compressed_filename, &tree_data, &compressed_data);
    println!("Compressed data written to {}", compressed_filename);
    // let compressed_file = read_binary_file("compressed.bin");
    // println!("Compressed file content: {:?}", compressed_file);
}

