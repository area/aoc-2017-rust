use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::cmp;


fn main() {


    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let layers_input = input.trim().to_string();

    let mut layers_map: HashMap<usize, usize> = HashMap::new();
    let mut max_layer_depth = 0;

    let layers = layers_input.split("\n").collect::<Vec<&str>>();
    for layer_desc in layers {
        let layer: Vec<&str> = layer_desc.split(": ").collect();
        // println!("{:?}", layer);
        let layer_depth = layer[0].parse::<usize>().unwrap();
        let layer_range = layer[1].parse::<usize>().unwrap();
        max_layer_depth =  cmp::max(max_layer_depth, layer_depth);

        layers_map.insert(layer_depth, layer_range);
    }

    let mut time = 0;
    let mut delay = 0;
    let mut severity;
    let mut caught = true;
    while caught {
        caught = false;
        severity = 0;
        // Time also indicates my position
        while time <= max_layer_depth {
            if layers_map.contains_key(&time){
                let layer_range = layers_map.get(&time).unwrap();
                if ((delay + time) % ((layer_range-1)*2)) == 0{
                    severity += time * layer_range;
                    caught = true;
                    if delay!=0{
                        break;
                    }
                }
            }
            time +=1;
        }
        if delay == 0 {
            println!("Total severity going through with 0 delay: {} ", severity);
        }
        time =0;
        delay +=1;
    }
    println!("To avoid being caught, use a delay of {} ", delay-1);

}
