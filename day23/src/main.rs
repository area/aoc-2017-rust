use std::io::Read;
use std::fs::File;
use std::collections::HashMap;

fn main() {


    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let instructions = input.trim().to_string();
    let instruction_list = instructions.split("\n").collect::<Vec<&str>>();
    let mut registers: HashMap<String, isize> = HashMap::new();
    let mut head: isize = 0;
    let mut mul_counter = 0;
    // change_register_value(&mut registers, "a", 1);
    loop {
        if head < 0 || head >= instruction_list.len() as isize{
            break;
        }
        // Program 0
        let instruction = instruction_list[head as usize];
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        let op = parts[0];
        let arg1 = parts[1];
        let mut arg2 = "";
        if parts.len()==3{
            arg2 = parts[2];
        }
        if op == "mul" {
            mul_counter+=1;
        }
        match op {
            "set" => set(&mut registers, arg1, arg2),
            "add" => add(&mut registers, arg1, arg2),
            "mul" => mul(&mut registers, arg1, arg2),
            "sub" => sub(&mut registers, arg1, arg2),
            // "mod" => cmod(&mut registers, arg1, arg2),
            // "rcv" => rcv(&mut registers, &mut queue0, arg1),
            // "snd" => snd(&mut registers, &mut queue1, arg1),
            // "jgz" => jgz(&mut registers, &mut head, arg1, arg2),
            "jnz" => jnz(&mut registers, &mut head, arg1, arg2),
            _ => println!("{} {} {}", op, arg1, arg2),
        }
        head +=1;

    }
    println!("When program finished executing, mul had been called {} times", mul_counter);
    // println!("When program finished executing, register h had value {}", get_register_value(&registers, "h"));
    let mut non_primes = 0;
    let mut b = 107900;
    while b < 124901{
        let mut x = 2;
        while (x as f64) < (b as f64).sqrt(){
            if b%x==0{
                non_primes+=1;
                break;
            }
            x+=1;
        }
        b+=17;
    }
    println!("When program finished executing in debug mode, register h would have value {}", non_primes);

}

fn jnz(registers: &mut HashMap<String, isize>, head: &mut isize,  arg1: &str, arg2: &str){
    let val_x = get_value_if_register(&registers, arg1);
    let val_y = get_value_if_register(&registers, arg2);
    if val_x != 0{
        *head += val_y - 1 //-1 as we add 1 to head after every instructino in this implementation, but
        //we're not meant to if we've jumped
    }
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

fn sub(mut registers: &mut HashMap<String, isize>, register: &str, value: &str) {
    let argument = get_value_if_register(&registers, value);
    let register_current_value = get_register_value(&registers, register);
    change_register_value(&mut registers, register, register_current_value - argument);
}

fn mul(mut registers: &mut HashMap<String, isize>, register: &str, value: &str) {
    let argument = get_value_if_register(&registers, value);
    let register_current_value = get_register_value(&registers, register);
    change_register_value(&mut registers, register, argument*register_current_value);
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
