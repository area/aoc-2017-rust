use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {


    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let instructions = input.trim().to_string();
    let mut registers: HashMap<String, isize> = HashMap::new();
    let mut register_list: HashSet<String> = HashSet::new();
    let mut max_during = 0;
    let instruction_list = instructions.split("\n").collect::<Vec<&str>>();
    let mut head: isize = 0;
    while head >=0 && head < instruction_list.len() as isize{
        let instruction = instruction_list[head as usize];
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        let op = parts[0];
        let arg1 = parts[1];
        let mut arg2 = "";
        if parts.len()==3{
            arg2 = parts[2];
        }
        match op {
            "set" => set(&mut registers, arg1, arg2),
            "add" => add(&mut registers, arg1, arg2),
            "mul" => mul(&mut registers, arg1, arg2),
            "mod" => cmod(&mut registers, arg1, arg2),
            "rcv" => rcv(&mut registers, arg1),
            "snd" => snd(&mut registers, arg1),
            "jgz" => jgz(&mut registers, &mut head, arg1, arg2),
            _ => println!("{} {} {}", op, arg1, arg2),
        }
        head+=1;
        if get_register_value(&registers, "rcv") != 0{
            println!("When RCV was executed for the first time, value {}  was recovered ", get_register_value(&registers, "rcv"));
            break;
        }
    }
}


fn jgz(mut registers: &mut HashMap<String, isize>, head: &mut isize,  arg1: &str, arg2: &str){
    let val_x = get_value_if_register(&registers, arg1);
    let val_y = get_value_if_register(&registers, arg2);
    if (val_x > 0){
        *head += val_y - 1 //-1 as we add 1 to head after every instructino in this implementation, but
        //we're not meant to if we've jumped
    }
}


fn rcv(mut registers: &mut HashMap<String, isize>, register: &str){
    let mut val_x = get_register_value(&registers, register);
    if (val_x != 0){
        let last_freq_played = get_register_value(&registers, "snd");
        change_register_value(&mut registers, "rcv", last_freq_played);
    }
}

fn snd(mut registers: &mut HashMap<String, isize>, register: &str){
    let mut val_x = get_register_value(&registers, register);
    change_register_value(&mut registers, "snd", val_x);
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
        Ok(val) => numerical_value = num.unwrap(),
        Err(why) => numerical_value = get_register_value(&registers, register),
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
