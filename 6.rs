fn sum_n(n: i32) -> i32 {
    /*use a formula to sum the first n integers*/
    let result = (n*(n+1))/2;
    /*square it*/
    return (result*result);
}

fn sum_n_squared(n: i32) -> i32 {
    /*use a formula to sum the first n squared integers*/
    let result = (n*(n+1)*(2*n+1))/6;
    return result;
}

fn main() {
    let sum: i32;
    let square_sum: i32;
    let diff: i32;

    sum = sum_n(100);
    square_sum = sum_n_squared(100);
    println!("{}",sum);
    println!("{}",square_sum);
    diff = sum - square_sum;

    println!("{}",diff);
}
