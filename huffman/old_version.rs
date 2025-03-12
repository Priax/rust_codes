use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::rc::Rc;
use std::cell::RefCell;
use std::fs::File;
use std::io::{Write, Read, BufReader, BufWriter};

#[derive(Debug, Clone)]
struct HuffmanNode {
    frequency: usize,
    symbol: Option<String>,
    left: Option<Rc<RefCell<HuffmanNode>>>,
    right: Option<Rc<RefCell<HuffmanNode>>>,
}

impl HuffmanNode {
    fn new(frequency: usize, symbol: Option<String>) -> Self {
        HuffmanNode {
            frequency,
            symbol,
            left: None,
            right: None,
        }
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl Eq for HuffmanNode {}

fn build_huffman_tree(frequencies: &[(String, usize)]) -> Option<Rc<RefCell<HuffmanNode>>> {
    let mut heap = BinaryHeap::new();

    for (symbol, frequency) in frequencies {
        heap.push(Rc::new(RefCell::new(HuffmanNode::new(*frequency, Some(symbol.clone())))));
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        let merged_frequency = left.borrow().frequency + right.borrow().frequency;
        let merged_node = Rc::new(RefCell::new(HuffmanNode::new(merged_frequency, None)));
        merged_node.borrow_mut().left = Some(left);
        merged_node.borrow_mut().right = Some(right);
        heap.push(merged_node);
    }

    heap.pop()
}

fn generate_codes(node: Option<Rc<RefCell<HuffmanNode>>>, prefix: String, codes: &mut HashMap<String, String>) {
    if let Some(n) = node {
        let n = n.borrow();
        if let Some(ref symbol) = n.symbol {
            codes.insert(symbol.clone(), prefix);
        } else {
            generate_codes(n.left.clone(), format!("{}0", prefix), codes);
            generate_codes(n.right.clone(), format!("{}1", prefix), codes);
        }
    }
}

fn calculate_frequencies(text: &str) -> HashMap<String, usize> {
    let mut frequencies = HashMap::new();
    for c in text.chars() {
        let symbol = c.to_string();
        *frequencies.entry(symbol).or_insert(0) += 1;
    }
    frequencies
}


fn encode_text(text: &str, codes: &HashMap<String, String>) -> Vec<u8> {
    let mut encoded_bits = String::new();
    for c in text.chars() {
        let symbol = c.to_string();
        if let Some(code) = codes.get(&symbol) {
            encoded_bits.push_str(code);
        }
    }

    let mut result = Vec::new();
    let mut byte = 0u8;
    let mut bit_count = 0;

    for bit in encoded_bits.chars() {
        byte <<= 1;
        if bit == '1' {
            byte |= 1;
        }
        bit_count += 1;

        if bit_count == 8 {
            result.push(byte);
            byte = 0;
            bit_count = 0;
        }
    }

    if bit_count > 0 {
        byte <<= 8 - bit_count;
        result.push(byte);
    }
    result
}

fn decode_text(encoded_bits: Vec<u8>, codes: &HashMap<String, String>, bit_length: usize) -> String {
    let mut decoded_text = String::new();
    let mut current_code = String::new();
    let mut reverse_codes: HashMap<String, String> = HashMap::new();

    for (symbol, code) in codes {
        reverse_codes.insert(code.clone(), symbol.clone());
    }

    let mut bit_string = String::new();
    for byte in encoded_bits {
        for i in (0..8).rev() {
            bit_string.push(if byte & (1 << i) != 0 { '1' } else { '0' });
        }
    }

    bit_string.truncate(bit_length);

    for bit in bit_string.chars() {
        current_code.push(bit);
        if let Some(symbol) = reverse_codes.get(&current_code) {
            decoded_text.push_str(symbol);
            current_code.clear();
        }
    }

    decoded_text
}

fn get_encoded_bit_length(encoded_text: &[u8]) -> usize {
    if encoded_text.is_empty() {
        return 0;
    }

    let mut last_byte_bits = 8;
    let last_byte = *encoded_text.last().unwrap();
    for i in 0..8 {
        if (last_byte & (1 << i)) != 0 {
            break;
        }
        last_byte_bits -= 1;
    }

    (encoded_text.len() - 1) * 8 + last_byte_bits
}

fn write_to_file(filename: &str, data: &[u8]) {
    let file = File::create(filename).expect("Unable to create file");
    let mut writer = BufWriter::new(file);
    writer.write_all(data).expect("Unable to write data");
}

fn read_from_file(filename: &str) -> Vec<u8> {
    let file = File::open(filename).expect("Unable to open file");
    let mut reader = BufReader::new(file);
    let mut contents = Vec::new();
    reader.read_to_end(&mut contents).expect("Unable to read data");
    contents
}

fn main() {
    let input_filename = "input.txt";
    let text_bytes = read_from_file(input_filename);
    let text = String::from_utf8(text_bytes.clone()).expect("Failed to convert file to string");
    println!("Original text: {}", text);

    let frequencies = calculate_frequencies(&text);
    let mut frequency_vec: Vec<(String, usize)> = frequencies.into_iter().collect();
    frequency_vec.sort_by(|a, b| b.1.cmp(&a.1)); // Sort by frequency in descending order

    let huffman_tree = build_huffman_tree(&frequency_vec);
    let mut codes = HashMap::new();

    if let Some(tree) = huffman_tree {
        generate_codes(Some(tree), String::new(), &mut codes);
    }

    for (symbol, code) in &codes {
        println!("Symbol: {}, Code: {}", symbol, code);
    }

    let encoded_text = encode_text(&text, &codes);
    println!("Encoded text (in bits): {:?}", encoded_text);

    let compressed_filename = "compressed.bin";
    write_to_file(compressed_filename, &encoded_text);
    println!("Compressed text written to {}", compressed_filename);

    let read_encoded_text = read_from_file(compressed_filename);
    println!("Read encoded text: {:?}", read_encoded_text);

    let read_encoded_text_bit_length = get_encoded_bit_length(&read_encoded_text.clone());
    let decoded_text = decode_text(read_encoded_text.clone(), &codes, read_encoded_text_bit_length);


    println!("Decoded text: {}", decoded_text);
}

