use std::io::Read;
use std::fs::File;

fn main() {
    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let path = input.to_string();
    let mut rows = path.split("\n").collect::<Vec<&str>>();
    rows.pop();
    let mut grid = vec![];
    for row in rows.iter(){
        grid.push(row.chars().collect::<Vec<char>>());
    }
    //Find startpos
    let mut position: (isize, isize ) = (0,0);
    let mut direction: (isize, isize) = (1,0);

    for i in 0..grid[0].len(){
        if grid[0][i]=='|'{
            position = (0, i as isize);
        }
    }
    let mut letters = String::new();
    let mut n_steps = 1;
    loop {
        position.0 += direction.0;
        position.1 += direction.1;
        let pos_row = position.0 as usize;
        let pos_col = position.1 as usize;

        let current_char = grid[pos_row][pos_col];
        if current_char==' '{
            break;
        }
        n_steps+=1;
        let mut n_char: char = ' ';
        let mut s_char: char = ' ';
        let mut w_char: char = ' ';
        let mut e_char: char = ' ';
        if pos_row > 0{
            n_char = grid[pos_row-1][pos_col];
        }
        if pos_row < grid.len()-1{
            s_char = grid[pos_row+1][pos_col];
        }
        if pos_col > 0{
            w_char = grid[pos_row][pos_col-1];
        }
        if pos_col < grid[pos_row].len()-1{
            e_char = grid[pos_row][pos_col+1];
        }

        if current_char == '+' {

            if n_char!=' ' && direction != (1,0){
                direction = (-1,0);
            }
            else if s_char!=' ' && direction != (-1,0){
                direction = (1,0);
            }
            else if w_char!=' ' && direction != (0,1){
                direction = (0,-1);
            }
            else if e_char!=' ' && direction != (0,-1){
                direction = (0,1);
            }
        } else if current_char.is_alphabetic(){
            letters.push(current_char);
        }
    }

    println!("letters: {:?}", letters);
    println!("nsteps: {:?}", n_steps);
}
