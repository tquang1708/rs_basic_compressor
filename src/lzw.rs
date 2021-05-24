use std::collections::HashMap;

pub fn lzw_encode(input: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut encode_dict = default_encode_dict();
    let mut curr_code: u16 = 0b11111111;
    let mut curr_code_length: u8 = 9;

    let mut buffer: Vec<u8> = Vec::new();
    let mut encoded_string: Vec<u8> = Vec::new();
    let mut bit_buffer: u8 = 0;
    let mut remaining_capacity: u8 = 8;

    for i in input.iter() {
        buffer.push(*i);

        if !encode_dict.contains_key(&buffer) {
            // append encoding to encoded_string
            // mask the value to the current code length then bitmanip it to fit the u8 buffer
            let encode = encode_dict.get(&buffer[0..buffer.len()-1]).expect("buffer not found in dict");
            let mut write_bit = curr_code_length;

            while write_bit > 0 {
                // writing to fit in buffer
                if write_bit >= remaining_capacity {
                    bit_buffer ^= ((encode >> (write_bit - remaining_capacity)) & ((1 << remaining_capacity) - 1)) as u8;
                    encoded_string.push(bit_buffer);

                    // reset buffer
                    bit_buffer = 0;
                    remaining_capacity = 8;

                    // reduce write bit
                    write_bit -= remaining_capacity;
                // writing at beginning of buffer
                } else {
                    bit_buffer ^= ((encode & ((1 << write_bit) - 1)) << (8 - write_bit)) as u8;
                    remaining_capacity = 8 - write_bit;
                    break;
                }
            };

            // do nothing if we hit the u16 limit
            if curr_code < std::u16::MAX - 1 {
                // increment curr_code
                curr_code += 1;

                // if we reach bit limit increase curr_code_length
                if (curr_code & (curr_code + 1)) == 0 {
                    curr_code_length += 1;
                };

                // update dict with new code
                encode_dict.insert(buffer.clone(), curr_code);
            }

            // reset buffer
            buffer = vec![*i];
        }
    }

    // add what's remaining to the output
    encoded_string.push(bit_buffer);

    Ok(encoded_string)
}

pub fn lzw_decode(input: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut decode_dict = default_decode_dict();
    let mut curr_code = 0b11111111;
    let mut curr_pointer = 0;
    let mut curr_code_length = 9;
    let mut conjecture: Vec<u8> = Vec::new();
    let mut decoded_string: Vec<u8> = Vec::new();

    while curr_pointer < len(input):
        //

    Ok(input)
}

// default code dictionary
// using 16-bit as our upper bound
// https://stackoverflow.com/questions/40054218/what-if-dictionary-size-in-lzw-algorithm-is-full
// https://www.cplusplus.com/articles/iL18T05o/
fn default_encode_dict() -> HashMap<Vec<u8>, u16> {
    let mut out = HashMap::new();
    for i in 0..255 {
        out.insert(vec![i], i as u16);
    };
    out
}

fn default_decode_dict() -> HashMap<u16, Vec<u8>> {
    let mut out = HashMap::new();
    for i in 0..255 {
        out.insert(i as u16, vec![i]);
    };
    out
}
