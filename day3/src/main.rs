use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let slot = args[1].parse::<f64>().unwrap() as usize;
    println!("Number of steps to origin: {}", get_steps(slot));
}

fn get_steps(slot: usize) -> usize{
    if slot==1{
        // Slot 1 is a special case
        return 0;
    }
    // Which ring are we in?
    let mut ring = (slot as f64).sqrt().ceil() as usize;
    if ring %2==0{
        ring+=1;
    }
    ring = ring/2;
    // Which numbers are along the edges of this ring?
    let midpoints = get_midpoints(ring);
    // How many steps to get to one of these midpoints?
    let steps_to_midpoint = midpoints.iter().map(|&x| (x as isize -slot as isize).abs() as usize).collect::<Vec<usize>>();
    // Total movement required is ring number plus minimum number of steps to midpoint
    return steps_to_midpoint.iter().min().unwrap() + ring;
}

fn get_midpoints(ring: usize) -> Vec<usize> {

    let u = vec![0, 1, 2, 3];
    return u.iter().map(|&x| ((ring*2+1).pow(2) - 2*x*ring -ring) as usize).collect::<Vec<_>>();
}
