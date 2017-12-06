use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn main() {

    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let m = input.trim().to_string();
    let mut memory: Vec<usize> = m.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let mut history = HashMap::new();
    let mut count = 0;
    let loop_length;
    loop {
        reallocate(&mut memory);
        count +=1;
        let str_representation: String = memory.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");

        if history.contains_key(&str_representation){
            loop_length = count - history[&str_representation];
            break;
        }
        history.insert(str_representation, count);
    }
    println!("Found loop after {} iterations", count);
    println!("Loop is {} long", loop_length);
}

fn reallocate(mut memory: &mut Vec<usize>){
    let mut slot = get_slot_to_reallocate(&memory);
    reallocate_slot(&mut memory, slot);
}

fn get_slot_to_reallocate(memory: &Vec<usize>) -> usize {
    let mut i=0;
    let mut max = 0;
    let mut loc = 0;
    while i < memory.len(){
        if memory[i] > max {
            max = memory[i];
            loc = i;
        }
        i+=1;
    }
    loc
}

fn reallocate_slot(memory: &mut Vec<usize>, slot: usize) {
    let mut nblocks = memory[slot];
    let mut head = slot;
    memory[head] = 0;
    while nblocks > 0 {
        head +=1;
        head = head % memory.len();
        memory[head] +=1;
        nblocks -=1;
    }
}
