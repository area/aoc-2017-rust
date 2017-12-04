use std::io::Read;
use std::fs::File;

fn main() {

    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
    let db = input.trim().to_string();

    check_validity(&db);
    check_validity_2(&db);
}

fn check_validity(db: &str){
    let mut valid = 0;

    for line in db.split("\n"){
        let passphrase = line.split(" ").collect::<Vec<&str>>();
        let mut sorted_passphrase = passphrase.to_vec();
        sorted_passphrase.sort();
        sorted_passphrase.dedup();
        if passphrase.len() == sorted_passphrase.len() {
            valid +=1;
        }
    }
    println!("N valid passphrases: {}", valid);
}

fn check_validity_2(db: &str){
    let mut valid = 0;

    for line in db.split("\n"){
        let passphrase = line.split(" ").collect::<Vec<&str>>();
        let mut sorted_passphrase = passphrase.to_vec();
        let mut anagrammed_passphrase: Vec<String> = sorted_passphrase.iter().map(|x| -> String {
            let mut letters: Vec<_> = x.chars().collect();
            letters.sort();
            letters.iter().collect()
        }).collect();

        anagrammed_passphrase.sort();
        anagrammed_passphrase.dedup();
        if passphrase.len() == anagrammed_passphrase.len() {
            valid +=1;
        }
    }
    println!("N valid passphrases: {}", valid);
}
