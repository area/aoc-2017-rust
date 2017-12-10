use std::io::Read;
use std::fs::File;

fn main() {


    let mut f = File::open("input").unwrap();
    let mut input = Vec::new();
    let _ = f.read_to_end(&mut input);
    input.pop(); // Stupid newline floating around
    input.extend(vec![17,31,73,47,23]);
    // input = vec![17,31,73,47,23];
    let mut elements = (0..256).collect::<Vec<usize>>();
    let mut cur_pos = 0;
    let mut skip_size = 0;
    // Execute swaps
    let mut round_number = 0;
    while round_number < 64 {
        for swap in input.iter(){
                reverse_section(&mut elements, cur_pos, swap);
                cur_pos += (*swap as usize + skip_size) % 256;
                skip_size += 1;
        }
        round_number +=1;
    }
    // Find the sparse hash
    let mut hash = String::new();
    let mut i =0;
    while i < 16{
        let mut j =0;
        let mut result = 0;
        while  j < 16 {
            result  = result ^ elements[i*16 + j];
            j+=1;
        }
        hash.push_str(&format!("{:x}", result));
        i+=1;
    }
    println!("The dense hash is {}", hash);

}

fn reverse_section(elements: &mut Vec<usize>, from: usize, length: &u8) {
    let mut idx = 0;
    // println!("{:?} {} {}", elements, from, length);
    while idx < *length as usize / 2{
        let target = (from + idx) % 256;
        let opposite = (from + *length as usize - idx - 1) % 256;
        let tmp = elements[opposite];
        elements[opposite] = elements[target];
        elements[target] = tmp;
        idx+=1;
    }
}
