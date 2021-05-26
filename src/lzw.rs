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

                    // reduce write bit
                    write_bit -= remaining_capacity;

                    // reset buffer
                    bit_buffer = 0;
                    remaining_capacity = 8;
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
    // unwrapping since buffer is in encode dict for certain
    let encode = encode_dict.get(&buffer).unwrap() << (16 - curr_code_length);
    encoded_string.push((encode >> 8) as u8);
    encoded_string.push(encode as u8);

    Ok(encoded_string)
}

pub fn lzw_decode(input: Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut decode_dict = default_decode_dict();
    let mut curr_code: u16 = 0b11111111;
    let mut curr_code_length: u8 = 9;

    let mut curr_pointer = 0;
    let mut conjecture: Vec<u8> = Vec::new();
    let mut decoded_string: Vec<u8> = Vec::new();

    let mut temp_buffer: u8 = 0;
    let mut temp_size: u8 = 0;

    while curr_pointer < input.len() {
        if conjecture.len() != 0 {
            // do nothing if we hit the u16 limit
            if curr_code < std::u16::MAX - 1 {
                // increment curr_code
                curr_code += 1;

                // if we reach bit limit increase curr_code_length
                if (curr_code & (curr_code + 1)) == 0 {
                    curr_code_length += 1;
                };
            }
        }

        // decode current and output to decoded text
        let mut read_bit = curr_code_length;
        let mut decode: u16 = 0;
        while read_bit > 0 { // still need to read
            if temp_size > 0 { // if there's leftover in temp_buffer
                decode ^= temp_buffer as u16; // read out the leftover
                read_bit -= temp_size;
                temp_size = 0;
            } else { // nothing in temp_buffer
                if curr_pointer < input.len() { // while we still can read
                    if read_bit >= 8 { // read 8 at a time
                        decode <<= 8;
                        decode ^= input[curr_pointer] as u16;
                        curr_pointer += 1;
                        read_bit -= 8;
                    } else { //read leftover into temp_buffer
                        decode <<= read_bit;
                        decode ^= (input[curr_pointer] >> (8 - read_bit)) as u16;
                        temp_size = 8 - read_bit;
                        temp_buffer = input[curr_pointer] & ((1 << temp_size) - 1);
                        curr_pointer += 1;
                        break;
                    }
                } else {
                    break; // file's over
                }
            }
        };

        let mut curr_decoded: Vec<u8> = Vec::new();
        let mut new_entry: Vec<u8> = Vec::new();
        match decode_dict.get(&decode) {
            Some(a) => {
                curr_decoded = (*a).clone();
                conjecture.push(curr_decoded[0]);
                new_entry = conjecture.clone();
            },
            None => {
                conjecture.push(conjecture[0]);
                curr_decoded = conjecture.clone();
                new_entry = conjecture.clone();
            },
        };

        // reconstruct conjecture
        conjecture = curr_decoded.clone();

        // append to decoded string
        //if curr_pointer < input.len() {
        //    decoded_string.append(&mut curr_decoded);
        //}
        decoded_string.append(&mut curr_decoded);

        // add to dictionary constructed conjecture
        decode_dict.insert(curr_code, new_entry);
    }

    Ok(decoded_string)
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
