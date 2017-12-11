use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::cmp;

static N: usize = 0;
static S: usize = 1;
static SW: usize = 2;
static SE: usize = 3;
static NW: usize = 4;
static NE: usize = 5;

fn main() {


    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let movement_list = input.trim().to_string();
    let mut steps: Vec<usize>= vec![0,0,0,0,0,0];

    let mut max_distance: usize = 0;

    for entry in movement_list.split(","){
        match entry {
            "n" => steps[N] +=1,
            "s" => steps[S] +=1,
            "sw" => steps[SW] +=1,
            "se" => steps[SE] +=1,
            "nw" => steps[NW] +=1,
            "ne" => steps[NE] +=1,
            _ => println!("Don't know")
        }
        let mut steps_copy = steps.to_vec();
        reduce(&mut steps_copy);
        max_distance = cmp::max(max_distance, steps_copy.iter().sum::<usize>());
    }

    reduce(&mut steps);
    println!("Steps away at end of movement: {:?}", steps.iter().sum::<usize>());
    println!("Max steps away during movement: {:?}", max_distance);
}


fn reduce(steps: &mut Vec<usize>){
    let mut shortened = true;
    while shortened {
        let mut before: usize = steps.iter().sum();
        let mut matched;

        // Cancel out n/s
        matched = cmp::min(steps[N],steps[S]);
        steps[N] -= matched;
        steps[S] -= matched;

        // Cancel out ne/sw
        matched = cmp::min(steps[NE],steps[SW]);
        steps[NE] -= matched;
        steps[SW] -= matched;

        // Cancel out nw/se
        matched = cmp::min(steps[NW],steps[SE]);
        steps[NW] -= matched;
        steps[SE] -= matched;

        // N,SE becomes NE
        matched = cmp::min(steps[N],steps[SE]);
        steps[N] -= matched;
        steps[SE] -= matched;
        steps[NE] += matched;

        // N,SW becomes NW
        matched = cmp::min(steps[N],steps[SW]);
        steps[N] -= matched;
        steps[SW] -= matched;
        steps[NW] += matched;

        // S,NE becomes SE
        matched = cmp::min(steps[S],steps[NE]);
        steps[S] -= matched;
        steps[NE] -= matched;
        steps[SE] += matched;

        // S,NW becomes SW
        matched = cmp::min(steps[S],steps[NW]);
        steps[S] -= matched;
        steps[NW] -= matched;
        steps[SW] += matched;

        //SW,SE becomes S
        matched = cmp::min(steps[SW],steps[SE]);
        steps[SW] -= matched;
        steps[SE] -= matched;
        steps[S] += matched;

        //NW,NE becomes N
        matched = cmp::min(steps[NE],steps[NW]);
        steps[NW] -= matched;
        steps[NE] -= matched;
        steps[N] += matched;

        let mut after: usize = steps.iter().sum();
        shortened = (before != after);
    }
}
