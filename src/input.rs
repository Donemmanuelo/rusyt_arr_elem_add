use std::io;

pub fn input() -> ([i32; 50], usize, usize) {
    println!("please enter the number of element you want in your array");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("invalid input");
    let num: usize = num.trim().parse().expect("invalid input");
    let mut array: [i32; 50] = [0; 50];
    let mut inde: usize = 0;
    for _ in 0..num {
    println!("please enter any number");
    let mut temp = String::new();
    io::stdin().read_line(&mut temp).expect("invalid input");
    let temp = temp.trim().parse().expect("invalid input");
    array[inde] = temp;
    inde +=1;
    }
    (array, inde, num)

}