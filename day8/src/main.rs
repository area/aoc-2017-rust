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
    for instruction in instructions.split("\n"){
        let parts: Vec<&str> = instruction.split_whitespace().collect();
        let target = parts[0];
        let op = parts[1];
        let amount = parts[2].parse::<isize>().unwrap();
        let predicate_register = parts[4];
        let predicate_operator = parts[5];
        let predicate_comparison_value = parts[6].parse::<isize>().unwrap();
        register_list.insert(target.to_string());
        // Is the predicate met?
        if evaluate_predicate(&registers, predicate_register, predicate_operator, predicate_comparison_value){
            change_register_value(&mut registers, target, op, amount);
            if get_register_value(&registers, &target) > max_during {
                max_during = get_register_value(&registers, &target);
            }

        }
    }
    let mut max_val = 0;
    for register in register_list{
        if get_register_value(&registers, &register) > max_val{
            max_val = get_register_value(&registers, &register);
        }
    }
    println!("After executing commands, largest value in a register is {}", max_val);
    println!("During execution, largest value in a register was {}", max_during);

}

fn evaluate_predicate(registers: &HashMap<String, isize>, register: &str, operator: &str, comp_value: isize) -> bool {
    if operator == ">" {
        return get_register_value(registers, register) > comp_value;
    } else if operator == "<" {
        return get_register_value(registers, register) < comp_value;
    } else if operator == ">=" {
        return get_register_value(registers, register) >= comp_value;
    } else if operator == "<=" {
        return get_register_value(registers, register) <= comp_value;
    } else if operator == "==" {
        return get_register_value(registers, register) == comp_value;
    } else { // operator == "!=", I hope!
        return get_register_value(registers, register) != comp_value;
    }
}

fn get_register_value(registers: &HashMap<String, isize>, register: &str) -> isize{
    if !registers.contains_key(register){
        return 0;
    }
    *registers.get(register).unwrap()
}

fn change_register_value(registers: &mut HashMap<String,isize>, register_name: &str, op: &str, amount: isize){
    if !registers.contains_key(register_name){
        registers.insert(register_name.to_string(), 0);
    }
    let register = registers.get_mut(register_name).unwrap();
    if op=="inc" {
        *register += amount;
    } else {
        *register -= amount;
    }
}
