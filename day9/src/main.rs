use std::io::Read;
use std::fs::File;

fn main() {
    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let stream: Vec<char> = input.trim().chars().collect();
    let mut idx = 0;
    let mut group = 0;
    let mut in_garbage = false;
    let mut garbage_count = 0;
    let mut score = 0;
    while idx < stream.len() {
        if stream[idx]=='!'{
            idx+=2;
            continue;
        }
        if in_garbage {
            if stream[idx]=='>'{
                in_garbage = false;
            } else{
                garbage_count +=1;
            }
        } else {
            if stream[idx]=='{' {
                group +=1;
            } else if stream[idx]=='<'{
                in_garbage = true;
            } else if stream[idx]=='}' {
                score+=group;
                group-=1;
            }
        }
        idx +=1;
    }
    println!("Score: {}", score);
    println!("Garbage characters: {}", garbage_count);

}
