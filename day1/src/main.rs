use std::io::Read;
use std::fs::File;

fn main() {
    let mut f = File::open("input").unwrap();
    let mut input = String::new();

    let _ = f.read_to_string(&mut input);

    let captcha = input.trim().as_bytes();


    solve_captcha(&captcha, 1);
    solve_captcha(&captcha, captcha.len()/2);
}

fn solve_captcha(captcha: &[u8], offset: usize){
    let mut count: usize = 0;
    let mut n: usize = 0;
    while n < captcha.len(){
        if captcha[n]==captcha[(n+offset)%captcha.len()]{
            count+=(captcha[n]-48) as usize; // We're dealing with ascii, so 0->9 is 48->57
        }
        n+=1;
    }
    println!("Sum of matching numbers: {}", count);
}
