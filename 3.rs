fn main() {
    /*use this algorithm
    https://stackoverflow.com/questions/23287/algorithm-to-find-largest-prime-factor-of-a-number
    */

    let mut max_prime_factor = 1;
    let mut d = 2;
    let mut n: i64 = 600851475143; /* big number we are trying to find factor of*/

    while (n > 1) {
        while (n % d) == 0 { 
            if (d > max_prime_factor) {
                max_prime_factor = d;
            }   
            n = n / d;
        }   
        d = d + 1;
    }   
    println!("{}", max_prime_factor);
}
