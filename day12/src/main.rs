use std::io::Read;
use std::fs::File;
use std::collections::HashSet;

fn main() {


    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let program_relations = input.trim().to_string();

    let mut programs_in_current_set: HashSet<usize> = HashSet::new();
    let mut programs_in_all_sets: HashSet<usize> = HashSet::new();

    let n_programs = program_relations.split("\n").collect::<Vec<&str>>().len();
    println!("Found {} ", n_programs);
    programs_in_current_set.insert(0);
    programs_in_all_sets.insert(0);
    let mut nSets = 1;
    while programs_in_all_sets.len() < n_programs{
        let mut added = true;
        while added{
            added = false;
            for program_relation in program_relations.split("\n"){
                let parts: Vec<&str> = program_relation.split_whitespace().collect();
                let target = parts[0].parse::<usize>().unwrap();
                let pipes: Vec<String> = parts[2..].iter().map(|x| x.to_string()).collect();
                // println!("{:?}", pipes);
                if programs_in_current_set.contains(&target){
                    for pipe in pipes{
                        let pipe2: String = pipe.split(",").collect();
                        let canonical_pipe = pipe2.parse::<usize>().unwrap();
                        // let canonical_pipe: usize = pipe.split(",").collect()[0].parse::<usize>().unwrap();
                        if !programs_in_current_set.contains(&canonical_pipe){
                            programs_in_current_set.insert(canonical_pipe);
                            programs_in_all_sets.insert(canonical_pipe);
                            added=true;
                        }
                    }
                }
            }
        }
        if programs_in_current_set.contains(&0){
            println!("Number of programs in zero set: {} ", programs_in_current_set.len());
        }
        programs_in_current_set = HashSet::new();
        //What is the first program not in the current set?
        let mut program = 0;
        while program < n_programs{
            if !programs_in_all_sets.contains(&program){
                programs_in_current_set.insert(program);
                programs_in_all_sets.insert(program);
                nSets += 1;
                break;
            }
            program +=1;
        }
    }
    println!("Total number of sets: {} ", nSets);

}
