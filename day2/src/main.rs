use std::io::Read;
use std::fs::File;

fn main() {

    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let db = input.trim().to_string();

    checksum(&db);
    checksum2(&db);
}

fn checksum(db: &str){
    let mut sum = 0;

    for line in db.split("\n"){
        let mut low: usize = usize::max_value();
        let mut high: usize = 0;
        for entry in line.split("\t"){
            let value = entry.parse().unwrap();
            if value > high {
                high = value;
            }
            if value < low {
                low = value;
            }
        }
        sum += high-low;
    }
    println!("Checksum: {}", sum);
}

fn checksum2(db: &str){
    let mut sum = 0;

    for line in db.split("\n"){
        let vec = line.split("\t").collect::<Vec<&str>>();
        let mut idx = 0;
        let mut idy = 0;
        loop {
            if idx==idy { idy+=1;} // Don't compare a number to itself.
            let a: usize = vec[idx].parse().unwrap();
            let b: usize = vec[idy].parse().unwrap();
            if a%b==0{
                sum += a/b;
                break;
            }
            if idy==vec.len()-1{
                idx+=1;
                idy=0;
            }else{
                idy+=1;
            }
        }
    }
    println!("Checksum2: {}", sum);
}
