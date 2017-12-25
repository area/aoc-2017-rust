use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn main() {


    // let mut f = File::open("input").unwrap();
    // let mut input = String::new();
    // let _ = f.read_to_string(&mut input);
    // let instructions = input.trim().to_string();
    // let instruction_list = instructions.split("\n").collect::<Vec<&str>>();
    let mut tape: HashMap<isize, isize> = HashMap::new();
    let mut head: isize = 0;
    let mut one_counter = 0;
    let mut instruction_counter = 0;
    let mut state = 'a';
    while instruction_counter< 12317297{
        let current_value = get_tape_value(&tape, head);
        if state=='a'{
            if current_value==0{
                change_tape_value(&mut tape, head, 1);
                one_counter+=1;
                head+=1;
                state='b';
            } else {
                change_tape_value(&mut tape, head, 0);
                one_counter-=1;
                head-=1;
                state='d';
            }
        } else if state=='b'{
            if current_value==0{
                change_tape_value(&mut tape, head, 1);
                one_counter+=1;
                head+=1;
                state='c';
            } else{
                change_tape_value(&mut tape, head, 0);
                one_counter-=1;
                head+=1;
                state='f';
            }
        } else if state=='c'{
            if current_value==0{
                change_tape_value(&mut tape, head, 1);
                one_counter+=1;
                head-=1;
                state='c';
            } else {
                head-=1;
                state='a';
            }
        } else if state=='d'{
            if current_value==0{
                head-=1;
                state='e';
            }else{
                head+=1;
                state='a';
            }
        } else if state=='e'{
            if current_value==0{
                change_tape_value(&mut tape, head, 1);
                one_counter+=1;
                head-=1;
                state='a';
            } else {
                change_tape_value(&mut tape, head, 0);
                one_counter-=1;
                head+=1;
                state='b'
            }
        } else if state=='f'{
            if current_value==0{
                head+=1;
                state='c';
            } else {
                change_tape_value(&mut tape, head, 0);
                one_counter-=1;
                head+=1;
                state='e';
            }
        }
        instruction_counter+=1;
    }
    println!("When the debug ran, there were {} times 1 occurred on the tape", one_counter);
    // println!("When program finished executing, register h had value {}", get_register_value(&registers, "h"));

}


fn get_tape_value(tape: &HashMap<isize, isize>, head: isize) -> isize{
    if !tape.contains_key(&head){
        return 0;
    }
    *tape.get(&head).unwrap()
}

fn change_tape_value(tape: &mut HashMap<isize,isize>, head: isize, value: isize){
    if !tape.contains_key(&head){
        tape.insert(head, 0);
    }
    let location = tape.get_mut(&head).unwrap();
    *location = value;
}
