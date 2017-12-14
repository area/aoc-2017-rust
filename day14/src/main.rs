fn main() {

    let key_string: String = "wenycdww-".to_string();
    let mut used_cells = 0;
    let mut grid: [[usize; 128]; 128] = [[0; 128]; 128];

    for ii in 0..128{
        let mut input = format!("{}{}", key_string, ii).into_bytes();
        input.extend(vec![17,31,73,47,23]);
        let mut elements = (0..256).collect::<Vec<usize>>();
        let mut cur_pos = 0;
        let mut skip_size = 0;
        // Execute swaps
        let mut round_number = 0;
        while round_number < 64 {
            for swap in input.iter(){
                    reverse_section(&mut elements, cur_pos, swap);
                    cur_pos += (*swap as usize + skip_size) % 256;
                    skip_size += 1;
            }
            round_number +=1;
        }
        // Find the sparse hash
        let mut hash = String::new();
        let mut i =0;
        while i < 16{
            let mut j =0;
            let mut result = 0;
            while  j < 16 {
                result  = result ^ elements[i*16 + j];
                j+=1;
            }
            hash.push_str(&format!("{:08b}", result));
            i+=1;
        }
        let hash_bytes = hash.as_bytes();
        for jj in 0..128{
            if hash_bytes[jj] == 49 {
                grid[ii][jj] = 1;
            }
        }
        used_cells += hash.matches("1").count();
    }
    println!("{} cells used", used_cells);
    let mut n_groups = 0;
    for ii in 0..128{
        for jj in 0..128{
            if grid[ii][jj]==1{
                n_groups +=1;
                remove_group_containing(&mut grid, ii, jj);
            }
        }
    }
    println!("{} groups in total", n_groups);
}

fn remove_group_containing(mut grid: &mut [[usize; 128]; 128], x: usize, y: usize){
    if grid[x][y]==1 {
        grid[x][y]=0;
        if y > 0 && grid[x][y-1]==1 {
            remove_group_containing(&mut grid, x, y-1);
        }
        if y < 127 && grid[x][y+1]==1 {
            remove_group_containing(&mut grid, x, y+1);
        }
        if x > 0 && grid[x-1][y]==1{
            remove_group_containing(&mut grid, x-1, y);
        }
        if x < 127 && grid[x+1][y]==1{
            remove_group_containing(&mut grid, x+1, y);
        }
    }
}

fn reverse_section(elements: &mut Vec<usize>, from: usize, length: &u8) {
    let mut idx = 0;
    // println!("{:?} {} {}", elements, from, length);
    while idx < *length as usize / 2{
        let target = (from + idx) % 256;
        let opposite = (from + *length as usize - idx - 1) % 256;
        let tmp = elements[opposite];
        elements[opposite] = elements[target];
        elements[target] = tmp;
        idx+=1;
    }
}
