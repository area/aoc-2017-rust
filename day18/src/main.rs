use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn main() {


    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let instructions = input.trim().to_string();
    let instruction_list = instructions.split("\n").collect::<Vec<&str>>();

    let mut registers0: HashMap<String, isize> = HashMap::new();
    let mut registers1: HashMap<String, isize> = HashMap::new();
    let mut queue0 = vec![];
    let mut queue1 = vec![];
    let mut head0: isize = 0;
    let mut head1: isize = 0;
    let mut deadlocked = false;
    let mut program1_send_counter = 0;

    //Initialise programs with ids
    change_register_value(&mut registers0, "p", 0);
    change_register_value(&mut registers1, "p", 1);
    while !deadlocked{
        // Program 0
        let instruction = instruction_list[head0 as usize];
        let mut waiting0: bool = false;
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        let op = parts[0];
        let arg1 = parts[1];
        let mut arg2 = "";
        if parts.len()==3{
            arg2 = parts[2];
        }
        if op=="rcv" && queue0.len()==0{
            waiting0 = true;
        } else {
            match op {
                "set" => set(&mut registers0, arg1, arg2),
                "add" => add(&mut registers0, arg1, arg2),
                "mul" => mul(&mut registers0, arg1, arg2),
                "mod" => cmod(&mut registers0, arg1, arg2),
                "rcv" => rcv(&mut registers0, &mut queue0, arg1),
                "snd" => snd(&mut registers0, &mut queue1, arg1),
                "jgz" => jgz(&mut registers0, &mut head0, arg1, arg2),
                _ => println!("{} {} {}", op, arg1, arg2),
            }
            head0 +=1;
        }

        // Program 1
        let instruction = instruction_list[head1 as usize];
        let mut waiting1 = false;
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        let op = parts[0];
        let arg1 = parts[1];
        let mut arg2 = "";
        if parts.len()==3{
            arg2 = parts[2];
        }
        if op=="rcv" && queue1.len()==0{
            waiting1 = true;
        } else {
            match op {
                "set" => set(&mut registers1, arg1, arg2),
                "add" => add(&mut registers1, arg1, arg2),
                "mul" => mul(&mut registers1, arg1, arg2),
                "mod" => cmod(&mut registers1, arg1, arg2),
                "rcv" => rcv(&mut registers1, &mut queue1, arg1),
                "snd" => snd(&mut registers1, &mut queue0, arg1),
                "jgz" => jgz(&mut registers1, &mut head1, arg1, arg2),
                _ => println!("{} {} {}", op, arg1, arg2),
            }
            head1+=1;
        }
        if op=="snd"{
            program1_send_counter+=1;
        }

        deadlocked = waiting1&&waiting0;
    }
    println!("When programs deadlocked, program 1 had sent {} times", program1_send_counter);
}


fn jgz(registers: &mut HashMap<String, isize>, head: &mut isize,  arg1: &str, arg2: &str){
    let val_x = get_value_if_register(&registers, arg1);
    let val_y = get_value_if_register(&registers, arg2);
    if val_x > 0{
        *head += val_y - 1 //-1 as we add 1 to head after every instructino in this implementation, but
        //we're not meant to if we've jumped
    }
}


fn rcv(mut registers: &mut HashMap<String, isize>, queue: &mut Vec<isize>, register: &str){
    let val = queue.remove(0);
    change_register_value(&mut registers, register, val);
}

fn snd(registers: &mut HashMap<String, isize>, queue: &mut Vec<isize>, register: &str){
    let val_x = get_value_if_register(&registers, register);
    queue.push(val_x);
}

fn set(mut registers: &mut HashMap<String, isize>, register: &str, value: &str) {
    let argument = get_value_if_register(&registers, value);
    change_register_value(&mut registers, register, argument);
}

fn add(mut registers: &mut HashMap<String, isize>, register: &str, value: &str) {
    let argument = get_value_if_register(&registers, value);
    let register_current_value = get_register_value(&registers, register);
    change_register_value(&mut registers, register, argument+register_current_value);
}

fn mul(mut registers: &mut HashMap<String, isize>, register: &str, value: &str) {
    let argument = get_value_if_register(&registers, value);
    let register_current_value = get_register_value(&registers, register);
    change_register_value(&mut registers, register, argument*register_current_value);
}

fn cmod(mut registers: &mut HashMap<String, isize>, register: &str, value: &str) {
    let argument = get_value_if_register(&registers, value);
    let register_current_value = get_register_value(&registers, register);
    change_register_value(&mut registers, register, register_current_value % argument);
}

fn get_value_if_register(registers: &HashMap<String, isize>, register: &str) -> isize {
    let num = register.parse::<isize>();
    let numerical_value;
    match num {
        Ok(_val) => numerical_value = num.unwrap(),
        Err(_why) => numerical_value = get_register_value(&registers, register),
    }
    numerical_value
}

fn get_register_value(registers: &HashMap<String, isize>, register: &str) -> isize{
    if !registers.contains_key(register){
        return 0;
    }
    *registers.get(register).unwrap()
}

fn change_register_value(registers: &mut HashMap<String,isize>, register_name: &str, amount: isize){
    if !registers.contains_key(register_name){
        registers.insert(register_name.to_string(), 0);
    }
    let register = registers.get_mut(register_name).unwrap();
    *register = amount;
}
