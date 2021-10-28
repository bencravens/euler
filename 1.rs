fn main() {
    let mut i=0;
    let mut sum=0;
    
    while i < 1000 {
        if (i % 5) == 0 {
            println!("{}", i);
            sum = sum + i;
        } else if (i % 3) == 0 {
            println!("{}", i);
            sum = sum + i;
        }
        i = i + 1;
    }
    println!("{}",sum);
}

