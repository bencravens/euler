fn is_palindrome(num: i32) -> bool {
    let str_num: String = num.to_string();
    let str_num_rev: String = str_num.chars().rev().collect();

    if str_num == str_num_rev {
        return true;
    } else {
        return false;
    }   
}

fn main() {
    let mut largest = 0;
    let mut product = 0;
    
    /*slow naive solution*/
    /*iterate over all 3 digit numbers*/
    for i in 0..999 {
        for j in 0..999 {
            product = i * j;
            if is_palindrome(product) {
                if product > largest {
                    largest = product;
                }   
            }   
        }   
    }   

    println!("{}",largest);
}
