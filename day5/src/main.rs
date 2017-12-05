use std::io::Read;
use std::fs::File;

fn main() {

    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let program = input.trim().to_string();

    run_program(&program);
    run_program2(&program);
}

fn run_program(program: &str){
    let mut count = 0;
    let mut head: isize = 0;
    let mut vec = program.split("\n").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    while head < vec.len() as isize {
    	vec[head as usize] += 1;
    	head += vec[head as usize]  - 1;
    	count +=1;
    }
    println!("Program took: {:?} instructions to execute", count );
}


fn run_program2(program: &str){
    let mut count = 0;
    let mut head: isize = 0;
    let mut vec = program.split("\n").map(|x| x.parse::<isize>().unwrap()).collect::<Vec<isize>>();
    while head < vec.len() as isize {
    	let delta;
    	if (vec[head as usize]) >=3 {
    		delta = -1;
    	} else {
    		delta = 1;
    	}
    	vec[head as usize] += delta;
    	head += vec[head as usize]  - delta;
    	count +=1;
    }
    println!("Program2 took: {:?} instructions to execute", count );
}