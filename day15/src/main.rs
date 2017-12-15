fn main() {
    let mut generator_a: usize = 883;
    let mut generator_b: usize = 879;

    let mut count = 0;
    for _i in 0..40000000 {
        generator_a *= 16807;
        generator_b *= 48271;
        generator_a %= 2147483647;
        generator_b %= 2147483647;
        if generator_a & 0xffff == generator_b & 0xffff {
            count +=1;
        }
    }
    println!("Total matched L16 bits: {}", count);

    generator_a = 883;
    generator_b = 879;

    count = 0;
    for _j in 0..5000000 {
        while generator_a %4 !=0{
            generator_a *= 16807;
            generator_a %= 2147483647;
        }
        while generator_b %8 !=0{
            generator_b *= 48271;
            generator_b %= 2147483647;
        }
        if generator_a & 0xffff == generator_b & 0xffff {
            count +=1;
        }
        //Move to next values
        generator_a *= 16807;
        generator_b *= 48271;
        generator_a %= 2147483647;
        generator_b %= 2147483647;
    }
    println!("Total matched L16 bits: {}", count);
}
