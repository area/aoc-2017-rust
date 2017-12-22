use std::io::Read;
use std::fs::File;
use std::collections::HashMap;


fn main() {

    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let starting_input = input.trim().to_string();
    let mut grid: HashMap<String, usize> = HashMap::new();
    // 0 - clean
    // 1 - weakened
    // 2 - infected
    // 3 - flagged
    let starting_rows = starting_input.split("\n").collect::<Vec<_>>();
    // println!("{:?}", starting_rows);
    let starting_size: isize = starting_rows.len() as isize;

    for y in 0..starting_size {
        let row = starting_rows[starting_size as usize - y as usize -1];
        for x in 0..starting_size {
            let character = row.chars().collect::<Vec<char>>()[x as usize];
            if character == '#'{
                let key = format!("{},{}",x-(starting_size-1)/2 , y-(starting_size-1)/2);
                grid.insert(key, 2);
            }
        }
    }
    // println!("{:?}", grid);

    let mut position = (0,0);
    let mut direction = (0,1);
    let mut infect_count = 0;
    for _i in 0..10000000{
        // println!("{}", node_infected(&grid, position));
        if node_state(&grid, position)==0{
            direction = turn_left(&direction);
            //We are about to infect, so increase counter
        }else if node_state(&grid, position)==1{
            infect_count+=1;
        }else if node_state(&grid, position)==2{
            direction = turn_right(&direction);
        }else if node_state(&grid, position)==3{
            direction = turn_right(&direction);
            direction = turn_right(&direction);
        }
        increment_node_state(&mut grid, position);
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

fn node_state(grid: &HashMap<String, usize>, coord: (isize, isize)) -> usize {
    let key = format!("{},{}",coord.0, coord.1);
    // println!("key looking up: {}", key);
    if !grid.contains_key(&key){
        return 0;
    }
    // println!("{}", grid.get(&key).unwrap());
    *grid.get(&key).unwrap()
}

fn increment_node_state(grid: &mut HashMap<String, usize>, coord: (isize, isize)) {
    let key = format!("{},{}",coord.0, coord.1);
    if !grid.contains_key(&key){
        grid.insert(key, 1);
    }else{
        *grid.get_mut(&key).unwrap() = (node_state(grid, coord) +1)%4;
    }
}
