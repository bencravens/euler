fn is_prime(n: i32) -> bool {
    let mut upper_limit = (n as f64).sqrt() as i32;
    let mut i=3;

    if (n < 2) {
        return false;
    } else if (n % 2)==0 {
        return false;
    }

    while i <= upper_limit {
        let r = n % i;
        if r==0 {
            return false;
        }
        i = i + 1;
    }

    return true;
}

fn main() {
    let num=10001;
    let mut count=0;
    let mut i=0;

    while count < num {
        if is_prime(i) {
            count = count + 1;
            println!("{}th prime is {}",count+1,i);
        }
        i = i + 1;
    }

}
