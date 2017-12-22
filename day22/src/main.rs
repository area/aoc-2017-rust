use std::io::Read;
use std::fs::File;
use std::collections::HashMap;


fn main() {

    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let starting_input = input.trim().to_string();
    let mut grid: HashMap<String, bool> = HashMap::new();

    let starting_rows = starting_input.split("\n").collect::<Vec<_>>();
    // println!("{:?}", starting_rows);
    let starting_size: isize = starting_rows.len() as isize;

    for y in 0..starting_size {
        let row = starting_rows[starting_size as usize - y as usize -1];
        for x in 0..starting_size {
            let character = row.chars().collect::<Vec<char>>()[x as usize];
            if character == '#'{
                let key = format!("{},{}",x-(starting_size-1)/2 , y-(starting_size-1)/2);
                grid.insert(key, true);
            }
        }
    }
    // println!("{:?}", grid);

    let mut position = (0,0);
    let mut direction = (0,1);
    let mut infect_count = 0;
    for _i in 0..10000{
        // println!("{}", node_infected(&grid, position));
        if !node_infected(&grid, position){
            direction = turn_left(&direction);
            //We are about to infect, so increase counter
            infect_count+=1;
        }else{
            direction = turn_right(&direction);
        }
        toggle_node(&mut grid, position);
        position.0 += direction.0;
        position.1 += direction.1;
        // println!("{:?}", grid);
        // println!("{:?}", direction);
        // println!("{:?}", position);
        // println!("====")
    }
    println!("{:?}", infect_count);

}

fn turn_left(direction: &(isize, isize)) -> (isize, isize){
    if direction.0 == 0 && direction.1 ==1{
        return (-1,0);
    } else if direction.0 == -1 && direction.1 == 0{
        return (0,-1);
    } else if direction.0 == 0 && direction.1==-1 {
        return (1,0);
    } else if direction.0 == 1 && direction.1==0 {
        return (0,1);
    } else {
        panic!("No such direction");
    }
}

fn turn_right(direction: &(isize, isize)) -> (isize, isize){
    if direction.0 == 0 && direction.1 ==1{
        return (1,0);
    } else if direction.0 == 1 && direction.1==0 {
        return (0,-1);
    } else if direction.0 == 0 && direction.1==-1 {
        return (-1,0);
    } else if direction.0 == -1 && direction.1 == 0{
        return (0,1);
    } else {
        panic!("No such direction");
    }
}

fn node_infected(grid: &HashMap<String, bool>, coord: (isize, isize)) -> bool {
    let key = format!("{},{}",coord.0, coord.1);
    // println!("key looking up: {}", key);
    if !grid.contains_key(&key){
        return false;
    }
    // println!("{}", grid.get(&key).unwrap());
    *grid.get(&key).unwrap()
}

fn toggle_node(grid: &mut HashMap<String, bool>, coord: (isize, isize)) {
    let key = format!("{},{}",coord.0, coord.1);
    if !grid.contains_key(&key){
        grid.insert(key, true);
    }else{
        *grid.get_mut(&key).unwrap() = !node_infected(grid, coord);
    }
}
