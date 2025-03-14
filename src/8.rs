fn main() {
    let mut num_list = Vec::new();
    for i in 1..101 {
        num_list.push(i);
    }

    let mut sum = 0;
    for i in &num_list {
        sum += i;
    }

    println!("Sum of first 100 numbers: {}", sum);
}
