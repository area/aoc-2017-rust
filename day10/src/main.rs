use std::io::Read;
use std::fs::File;

fn main() {


    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let swaps = input.trim().to_string();
    let mut elements = (0..256).collect::<Vec<usize>>();
    let mut cur_pos = 0;
    let mut skip_size = 0;
    // Execute swaps
    for swap in swaps.split(","){
        let swap_val = swap.parse().unwrap();
        reverse_section(&mut elements, cur_pos, swap_val);
        cur_pos += (swap_val + skip_size) % elements.len();
        skip_size += 1;
    }

    println!("The product of first two entries after operations is {}", elements[0] * elements[1]);

}

fn reverse_section(elements: &mut Vec<usize>, from: usize, length: usize) {
    let mut idx = 0;
    while idx < length / 2{
        let target = (from + idx) % elements.len();
        let opposite = (from + length - idx - 1) % elements.len();
        let tmp = elements[opposite];
        elements[opposite] = elements[target];
        elements[target] = tmp;
        idx+=1;
    }
}
