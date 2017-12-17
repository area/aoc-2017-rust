use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn main() {

    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let instructions = input.trim().to_string();
    // let mut mapping: HashMap<String, String> = HashMap::new();
    let mut state_counter: HashMap<String, usize> = HashMap::new();

    let mut programs: Vec<char> = vec!['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p'];
    let mut loop_length = 0;
    let mut loop_start = "";
    let mut loop_counted = false;
    let mut remaining_steps = 0;
    let mut iterations = 0;
    for i in 0..1000000000{
        let startpos = programs.iter().collect::<String>();
        for instruction in instructions.split(","){
            let (command, args) = instruction.split_at(1);
            // println!("parts: {:?} {:?}", command, args);
            match command {
                "s" => spin(&mut programs, args),
                "x" => exchange(&mut programs, args),
                "p" => partner(&mut programs, args),
                _ => println!("unknown"),
            }
        }
        let endpos = programs.iter().collect::<String>();
        iterations +=1;
        // mapping.insert(startpos, endpos);
        if state_counter.contains_key(&endpos){
            let loop_length = i- state_counter.get(&endpos).unwrap()+1;
            remaining_steps = (1000000000-i-1) % loop_length;
            break;
        }
        state_counter.insert(startpos, i);
        if (i==0){
            println!("programs after 1 iteration: {}", programs.iter().collect::<String>());
        }
    }
    // We broke out of the loop. now just rpun it the last few steps
    for j in 0..remaining_steps{
        for instruction in instructions.split(","){
            let (command, args) = instruction.split_at(1);
            match command {
                "s" => spin(&mut programs, args),
                "x" => exchange(&mut programs, args),
                "p" => partner(&mut programs, args),
                _ => println!("unknown"),
            }
        }
        iterations+=1;
    }

    println!("programs after 1000000000 iterations: {}", programs.iter().collect::<String>());

}

fn partner(programs: &mut Vec<char>, args: &str){
    let arguments: Vec<&str> = args.split("/").collect();
    // println!("arguments for partner {:?}", arguments);
    let mut idx1=0;
    let mut idx2=0;
    for i in 0..programs.len(){
        if programs[i] == arguments[0].chars().next().unwrap(){
            idx1 = i;
        }
        if programs[i] == arguments[1].chars().next().unwrap(){
            idx2 = i;
        }
    }
    let tmp = programs[idx2];
    programs[idx2] = programs[idx1];
    programs[idx1] = tmp;
}

fn exchange(programs: &mut Vec<char>, args: &str){
    let arguments: Vec<&str> = args.split("/").collect();
    let a: usize = arguments[0].parse::<usize>().unwrap();
    let b: usize = arguments[1].parse::<usize>().unwrap();

    let tmp = programs[a];
    programs[a] = programs[b];
    programs[b] = tmp;
}

fn spin(programs: &mut Vec<char>, args: &str){
    let len = programs.len();
    let amount = args.parse::<usize>().unwrap();
    for i in 0..amount {
        let mut tmp = programs[1];
        for j in 0..len{
            let idx1 = j;
            let idx2 = (j+1)%len;
            let mut tmp2 = programs[idx2];
            programs[idx2] = tmp;
            tmp = tmp2;
        }
        programs[1] = tmp;
    }
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
