use std::io;
pub fn add(a: [i32; 50], t: usize) {
    let tmp2 = t - 1;

    println!("please enter the index of the element you want to add to your array / index start from 0 to {tmp2}");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("invalid input");
    let num: usize = num.trim().parse().expect("invalid input");

    println!("please enter the element you want to input in the array");
    let mut nu = String::new();
    io::stdin().read_line(&mut nu).expect("invalid input");
    let nu: usize = nu.trim().parse().expect("invalid input");
    let mut tmp1 = 0;
    // let mut tmp2  = 0;
    println!("The new array is:");
    print!("[");
    for i in 0..t + 1 {
        if i == num {
            tmp1 = a[i];

            print!("{}, ", nu);
            continue;
        } else if num == 0 && tmp1 == a[i - 1] {
            //tmp2 = a[i + 1];
            //a[i] = tmp2;
            print!("{}, ", tmp1);
            tmp1 = a[i];
            continue;
        } else if i >= num && a[i] != a[i - 1]  {
            //tmp1 = a[i - 1];
            print!("{}, ", a[i - 1]);
            continue;
        } else {
            print!("{}, ", a[i]);
            continue;
        }
    }
    print!("]");
}
