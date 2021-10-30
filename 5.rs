fn main() {
    let mut found = false;
    let mut smallest = 20; 
    let mut factor = 20; 
    
    while found==false {
        while factor > 0 { 
            if (smallest % factor) == 0 { 
                factor = factor - 1;
            } else {
                /*reset*/
                factor = 20; 
                smallest = smallest + 1;
            }   
        }   
        found = true;
    }   
    println!("{}",smallest);
}
