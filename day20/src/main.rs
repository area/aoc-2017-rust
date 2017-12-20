use std::io::Read;
use std::fs::File;
use std::collections::HashMap;


struct Particle {
    p: Vec<isize>,
    v: Vec<isize>,
    a: Vec<isize>,
    destroyed: bool
}


fn main() {


    let mut f = File::open("input").unwrap();
    let mut input = String::new();
    let _ = f.read_to_string(&mut input);
   // let mut registers1: HashMap<String, isize> = HashMap::new();

    let mut particles: HashMap<usize, Particle> = HashMap::new();
    let particles_start = input.trim().to_string();
    let mut idx = 0;
    for particle_start in particles_start.split("\n"){
        let vectors = particle_start.split(", ").collect::<Vec<&str>>();
        let p: Vec<isize> = vectors[0].split("<").collect::<Vec<&str>>()[1].split(">").collect::<Vec<&str>>()[0].split(',').collect::<Vec<&str>>().iter().map(|x| x.parse().unwrap()).collect();
        let v: Vec<isize> = vectors[1].split("<").collect::<Vec<&str>>()[1].split(">").collect::<Vec<&str>>()[0].split(',').collect::<Vec<&str>>().iter().map(|x| x.parse().unwrap()).collect();
        let a: Vec<isize> = vectors[2].split("<").collect::<Vec<&str>>()[1].split(">").collect::<Vec<&str>>()[0].split(',').collect::<Vec<&str>>().iter().map(|x| x.parse().unwrap()).collect();
        // println!("{:?}", p);
        // println!("{:?}", v);
        // println!("{:?}", a);
        particles.insert(idx, Particle {p , v , a, destroyed: false});
        idx+=1;
    }
    let time = 400;
    for _time in 0..time {
        for idx in 0..1000{
            let particle = particles.get_mut(&idx).unwrap();
            particle.v[0]+= particle.a[0];
            particle.v[1] += particle.a[1];
            particle.v[2] += particle.a[2];

            particle.p[0]+= particle.v[0];
            particle.p[1] += particle.v[1];
            particle.p[2] += particle.v[2];
        }

        //Check for collisions.
        let mut to_mark_as_collided = vec![];
        for idx in 0..1000{
            let particle = particles.get(&idx).unwrap();
            if particle.destroyed { continue; }
            for idy in idx+1..1000{
                let particle2 = particles.get(&idy).unwrap();
                if particle2.destroyed { continue; }
                if particle.p[0]==particle2.p[0] && particle.p[1]==particle2.p[1] && particle.p[2]==particle2.p[2]{
                    //They collided
                    to_mark_as_collided.push(idx);
                    to_mark_as_collided.push(idy);
                }
            }
        }
        to_mark_as_collided.sort();
        to_mark_as_collided.dedup();
        //Mark collided as destroyed
        for idx in to_mark_as_collided{
            // println!("{}", idx);
            let particle = particles.get_mut(&idx).unwrap();
            particle.destroyed = true;
        }
    }
    let mut closest_particle = 0;
    let mut closest_distance = distance(&particles[&0]);
    for idx in 1..1000{
        if distance(&particles[&idx]) < closest_distance {
            closest_distance = distance(&particles[&idx]);
            closest_particle = idx;
        }
    }
    println!("Closest particle to the origin after {} is {} ", time, closest_particle);
    //Count destoryed
    let mut destroyed = 0;
    for idx in 0..1000{
        let particle = particles.get(&idx).unwrap();
        if particle.destroyed==true{
            destroyed+=1;
        }
    }
    println!("Number of particles destroyed is {}, leaving {} ", destroyed, 1000-destroyed);

}

fn distance(particle: &Particle) -> usize{
    return (particle.p[0].abs() + particle.p[1].abs() + particle.p[2].abs()) as usize;
}