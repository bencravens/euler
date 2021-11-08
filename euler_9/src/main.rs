use rand::Rng;

fn main() {
    let mut found = false;
    let mut sum: i64 = 1;
    let mut a: i64 = 1;
    let mut b: i64 = 1;
    let mut c_squared: i64 = 1;
    let mut cfloat: f64 = 1.0;
    let mut c: i64 = 1;

    let mut rng = rand::thread_rng();

    //run until we have a solution
    while found!=true {
        a = rng.gen_range(1..1000);
        b = rng.gen_range(1..1000);
        c_squared = a.pow(2) + b.pow(2);
        cfloat = (c_squared as f64).sqrt();
        if ((cfloat as i64) as f64) == cfloat {
            //c is a whole number
            c = cfloat as i64;
            sum=a+b+c;
            if (sum==1000) {
                found=true;
                break;
            }
        }
    }
    println!("a is {}, b is {}, c is {}",a,b,c);
    println!("sum is {}",sum);
    println!("a*b*c is {}",a*b*c);
}
