fn main() {
    let number_list = vec![34, 50, 25, 100, 64];

    let mut largest = &number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("largest = {largest}");

}
