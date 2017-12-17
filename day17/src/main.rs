fn main() {
    let mut spinlock: Vec<usize> = vec![0];
    let mut spinlock_pos = 0;
    // let mut step_size = 377;
    let mut step_size = 377;
    for i in 1..2018{
        spinlock_pos+=1;
        spinlock.insert(spinlock_pos, i);
        spinlock_pos += step_size;
        spinlock_pos %= spinlock.len();
    }
    for j in 0..spinlock.len(){
        if spinlock[j]==2017{
            println!("value after 2017 is {}", spinlock[(j+1)%spinlock.len()]);
            break;
        }
    }

    let mut spinlock_length = 1;
    let mut spinlock_slot1=0;
    let mut spinlock_pos = 0;
    for j in 1..50000001{
        spinlock_pos+=1;
        if (spinlock_pos==1){
            spinlock_slot1=j;
        }
        spinlock_length +=1;
        spinlock_pos+=step_size;
        spinlock_pos %=spinlock_length;
    }
    println!("value after 0 after 50 million insertions is {}", spinlock_slot1);
}
