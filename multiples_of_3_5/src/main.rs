fn main() {
    let mut sum = 0;

    for val in 1..1000 {
        if (val % 3 == 0) || (val % 5 == 0) {
            sum += val;
        }
    }
    println!("sum of multiples of 3 and 5 below 10 {}", sum);
}
