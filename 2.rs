fn main() {
    /*only store current and previous two to avoid 
    a really massive array*/
    
    /*fibbonaci number 2 iterations ago */
    let mut prev_prev = 0;
    /*fibbonaci number last iteration */
    let mut prev = 1;
    /*current fibonnaci number*/
    let mut cur = 1;
    let mut sum = 0;

    /* while cur < 4e6 */
    while (cur) < 4000000 {
        cur = prev + prev_prev;
        println!("{}", cur);
        /* add even fibbonacis to sum*/
        if (cur % 2) == 0 { 
            sum = sum + cur;
        }   
        /*set prev and prev_prev */
        prev_prev = prev;
        prev = cur;
    }   
    println!("{}", sum);
}
