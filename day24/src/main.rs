use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::cmp;


fn main() {

    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let starting_input = input.trim().to_string();
    let component_list = starting_input.split("\n").collect::<Vec<&str>>();

    let current_best_strength = 0;
    let current_open_socket = 0;


    println!("{:?}", build_bridge(current_open_socket, 0, 0, component_list));

}

fn build_bridge(current_open_socket: usize, current_strength: usize, current_length:usize, component_list: Vec<&str>) -> (usize, usize){
    let mut max_strength = current_strength;
    let mut max_length = current_length;
    for component_idx in 0..component_list.len(){
        let component = component_list[component_idx];
        let sockets: Vec<_>= component.split("/").map(|x| x.parse().unwrap()).collect();
        if (sockets[0]==current_open_socket){
            let mut remaining_components = component_list.to_vec();
            remaining_components.remove(component_idx);
            let final_res = build_bridge(sockets[1],current_strength+sockets[0]+sockets[1], current_length+1, remaining_components);
            if final_res.1 > max_length{
                max_strength = final_res.0;
                max_length = final_res.1;
            } else if final_res.1==max_length && final_res.0 > max_strength{
                max_strength = final_res.0;
            }

        }else if (sockets[1]==current_open_socket){
            let mut remaining_components = component_list.to_vec();
            remaining_components.remove(component_idx);
            let final_res = build_bridge(sockets[0],current_strength+sockets[0]+sockets[1], current_length+1, remaining_components);
            if final_res.1 > max_length{
                max_strength = final_res.0;
                max_length = final_res.1;
            } else if final_res.1==max_length && final_res.0 > max_strength{
                max_strength = final_res.0;
            }
        }
    }
    return (max_strength, max_length);
}