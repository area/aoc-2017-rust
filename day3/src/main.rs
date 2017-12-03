use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let slot = args[1].parse::<f64>().unwrap() as usize;
    println!("Number of steps to origin from slot {}: {}", slot, get_steps(slot));
    let mut idx = 1;
    loop {
        if slot > get_stored_value_in_slot(idx){
            idx+=1;
            continue
        } else {
            break;
        }
    }
    println!("First slot with value greater than {} is {} with value {}", slot, idx, get_stored_value_in_slot(idx));
}

fn get_steps(slot: usize) -> usize{
    if slot==1{
        // Slot 1 is a special case
        return 0;
    }
    // Which ring are we in?
    let ring = get_ring(slot);
    // Which numbers are along the edges of this ring?
    let midpoints = get_midpoints(ring);
    // How many steps to get to one of these midpoints?
    let steps_to_midpoint = midpoints.iter().map(|&x| (x as isize -slot as isize).abs() as usize).collect::<Vec<usize>>();
    // Total movement required is ring number plus minimum number of steps to midpoint
    return steps_to_midpoint.iter().min().unwrap() + ring;
}

fn get_ring(slot: usize) -> usize{
    let mut ring = (slot as f64).sqrt().ceil() as usize;
    if ring %2==0{
        ring+=1;
    }
    return ring/2;
}

fn get_midpoints(ring: usize) -> Vec<usize> {

    let u = vec![0, 1, 2, 3];
    return u.iter().map(|&x| ((ring*2+1).pow(2) - 2*x*ring -ring) as usize).collect::<Vec<_>>();
}


fn get_same_ring_adjacent_indicies(index: usize) -> Vec<usize> {
    let mut adjacent = Vec::new();
    if index==1{
        return adjacent
    }
    //First, which ring is this in?
    let ring = get_ring(index);
    // Now, which slots on that ring is it adjacent to?
    // We're always adjacent to the slot one less than us
    // But if it's on the previous ring, don't return it here
    if ring==get_ring(index-1){
        adjacent.push(index-1);
    }
    // We're adjacent to the slot two less than us if we just turned a corner
    if get_steps(index-1) > get_steps(index) && get_steps(index)==get_steps(index-2) && get_ring(index)==get_ring(index-2){
        adjacent.push(index-2);
    }

    // If we're the penultimate or last slot on our ring, we're adjacent to the first slot
    if (ring*2+1).pow(2)-index <=1{
        adjacent.push(((ring-1)*2+1).pow(2)+1);
    }

    return adjacent;
}

fn get_lower_ring_adjacent_indicies(index:usize) -> Vec<usize> {
    let mut adjacent = Vec::new();
    if index==1{
        return adjacent
    }
    if index<=9{
        adjacent.push(1);
        return adjacent;
    }
    let ring = get_ring(index);
    let ringmax = get_ring_max(ring);
    let ringmin = get_ring_min(ring);
    // If we're the first on the ring, we're adjacent to the first and last of the previous ring
    if index==((ring-1)*2+1).pow(2)+1 {
        adjacent.push(get_ring_max(ring-1));
        adjacent.push(get_ring_max(ring-2)+1);
    }
    else if index==((ring-1)*2+1).pow(2)+2 {
    // If we're the second index on the ring, we're adjacent to the first, second and last of the previous ring
        adjacent.push(get_ring_max(ring-1));
        adjacent.push(get_ring_max(ring-2)+1);
        adjacent.push(get_ring_max(ring-2)+2);
    } else if is_corner(index){
        //We're next to the same corner on the previous ring
        let n = (ringmax-index) / (ring*2);
        adjacent.push(get_ring_max(ring-1) - n*((ring-1)*2));
    } else if is_corner(index+1){
        //We're next to the same corner on the previous ring, and the index next to in the same direction
        let n = (ringmax-index-1) / (ring*2);
        adjacent.push(get_ring_max(ring-1) - n*((ring-1)*2));
        adjacent.push(get_ring_max(ring-1) - n*((ring-1)*2)-1);
    } else if is_corner(index-1){
        //We're next to the same corner on the previous ring, and the index next to in the same direction
        let n = (ringmax-index+1) / (ring*2);
        adjacent.push(get_ring_max(ring-1) - n*((ring-1)*2));
        adjacent.push(get_ring_max(ring-1) - n*((ring-1)*2)+1);
    } else {
        // We are next to three slots that are in a straight line on the previous row.
        // Which was the last corner we did?
        let sidelength = (ringmax-ringmin+1) / 4;
        let side = (index-ringmin) / sidelength;
        let slots_since_corner = index-ringmin - (side*sidelength) +1;
        let adjacent_index_below = get_ring_min(ring-1) + ((get_ring_max(ring-1)-get_ring_min(ring-1) + 1)/4)*side + slots_since_corner -2;
        adjacent.push(adjacent_index_below-1);
        adjacent.push(adjacent_index_below);
        adjacent.push(adjacent_index_below+1);
    }

    return adjacent;
}

fn get_ring_max(ring:usize) -> usize{
    return (ring*2+1).pow(2)
}

fn get_ring_min(ring:usize) -> usize{
    return get_ring_max(ring-1)+1
}


fn is_corner(index:usize) -> bool {
    if (get_steps(index-1)==get_steps(index+1) && get_steps(index)>get_steps(index+1)) || (get_ring(index)*2+1).pow(2)==index {
        return true;
    }
    false
}


fn get_stored_value_in_slot(index: usize) -> usize{
    if index==1{
        return 1;
    }
    let slots = get_same_ring_adjacent_indicies(index);
    let values = slots.iter().map(|x| get_stored_value_in_slot(*x)).collect::<Vec<usize>>();
    let slots2 = get_lower_ring_adjacent_indicies(index);
    let values2 = slots2.iter().map(|x| get_stored_value_in_slot(*x)).collect::<Vec<usize>>();
    let sum = values.iter().sum::<usize>() + values2.iter().sum::<usize>();
    return sum;

}
