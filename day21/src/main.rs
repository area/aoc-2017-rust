use std::io::Read;
use std::fs::File;
use std::collections::HashMap;


fn main() {

    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let mut image = Vec::new();
    image.push(vec!['.','#','.']);
    image.push(vec!['.','.','#']);
    image.push(vec!['#','#','#']);

    let rules_input = input.trim().to_string();
    let mut rules: HashMap<String, String> = HashMap::new();

    for rule in rules_input.split("\n"){
        let parts = rule.split("=>").collect::<Vec<&str>>();
        rules.insert(parts[0].trim().to_string(), parts[1].trim().to_string());
        rules.insert(flip_x(&parts[0].trim().to_string()), parts[1].trim().to_string());
        rules.insert(flip_y(&parts[0].trim().to_string()), parts[1].trim().to_string());
    }
    for k in 0..18 {
        let size = image.len();
        // print_image(&image);
        let mut new_image = Vec::new();

        if size %2 == 0{
            for _i in 0..size+size/2{
                new_image.push(vec!['.'; size + size/2])
            }
            for x in 0..size/2{
                for y in 0..size/2{
                    let rep1 = format!("{}{}/{}{}", image[x*2][y*2], image[x*2][y*2+1], image[x*2+1][y*2], image[x*2+1][y*2+1]);
                    let rep2 = format!("{}{}/{}{}", image[x*2+1][y*2], image[x*2][y*2], image[x*2+1][y*2+1], image[x*2][y*2+1]);
                    let rep3 = rep1.chars().rev().collect::<String>();
                    let rep4 = rep2.chars().rev().collect::<String>();
                    let mut output = "";
                    if rules.contains_key(&rep1){
                        output = rules.get(&rep1).unwrap();
                    } else if rules.contains_key(&rep2){
                        output = rules.get(&rep2).unwrap();
                    } else if rules.contains_key(&rep3){
                        output = rules.get(&rep3).unwrap();
                    } else if rules.contains_key(&rep4){
                        output = rules.get(&rep4).unwrap();
                    } else {
                        panic!("No rule found for {} {} {} {}", rep1, rep2, rep3, rep4)
                    }
                    let mut x_offset=0;
                    let mut y_offset=0;
                    for character in output.chars(){
                        if character == '/' {
                            y_offset+=1;
                            x_offset=0;
                            continue;
                        }
                        new_image[x*3+x_offset][y*3+y_offset] = character;
                        x_offset+=1;
                    }
                }
            }
        }
        else if size %3 == 0{
            for _i in 0..size+size/3{
                new_image.push(vec!['.'; size + size/3])
            }

            for x in 0..size/3{
                for y in 0..size/3{
                    // 123     741
                    // 456     852
                    // 789     963
                    let rep1 = format!("{}{}{}/{}{}{}/{}{}{}", image[x*3][y*3], image[x*3][y*3+1], image[x*3][y*3+2], image[x*3+1][y*3], image[x*3+1][y*3+1], image[x*3+1][y*3+2], image[x*3+2][y*3], image[x*3+2][y*3+1], image[x*3+2][y*3+2]);
                    let rep2 = format!("{}{}{}/{}{}{}/{}{}{}", image[x*3+2][y*3], image[x*3+1][y*3],image[x*3][y*3], image[x*3+2][y*3+1], image[x*3+1][y*3+1], image[x*3][y*3+1], image[x*3+2][y*3+2], image[x*3+1][y*3+2], image[x*3][y*3+2]);
                    let rep3 = rep1.chars().rev().collect::<String>();
                    let rep4 = rep2.chars().rev().collect::<String>();
                    let mut output = "";
                    if rules.contains_key(&rep1){
                        output = rules.get(&rep1).unwrap();
                    } else if rules.contains_key(&rep2){
                        output = rules.get(&rep2).unwrap();
                    } else if rules.contains_key(&rep3){
                        output = rules.get(&rep3).unwrap();
                    } else if rules.contains_key(&rep4){
                        output = rules.get(&rep4).unwrap();
                    } else {
                        panic!("No rule found for {} {} {} {}", rep1, rep2, rep3, rep4)
                    }
                    let mut x_offset=0;
                    let mut y_offset=0;
                    for character in output.chars(){
                        if character == '/' {
                            y_offset+=1;
                            x_offset=0;
                            continue;
                        }
                        new_image[x*4+x_offset][y*4+y_offset] = character;
                        x_offset+=1;
                    }
                }
            }
        }

        else {
            panic!("Unexpected size");
        }

        image = new_image;
        println!("After {} iterations, {} pixels on", k+1, count_on(&image));
    }
    // print_image(&image);

}

fn flip_x(representation: &String) -> String {
    return representation.split("/").map(|x| x.chars().rev().collect()).collect::<Vec<String>>().join("/");
}

fn flip_y(representation: &String) -> String{
    let mut ret = String::new();
    for row in representation.split('/').rev(){
        ret.push_str(row);
        ret.push_str("/");
    }
    ret.pop();
    ret.to_string()
}

fn count_on(image: &Vec<Vec<char>>) -> usize{
    let mut count = 0;
    for row in image{
        for c in row{
            if *c == '#'{
                count+=1;
            }
        }
    }
    count
}

fn print_image(image: &Vec<Vec<char>>){
    let mut output: String = String::new();
    for row in image.clone(){
        output.push_str(&format!("{}", row.iter().collect::<String>()));
        output.push_str("\n");
    }
    println!("{}", output);
}

// fn distance(particle: &Particle) -> usize{
//     return (particle.p[0].abs() + particle.p[1].abs() + particle.p[2].abs()) as usize;
// }
