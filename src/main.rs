mod input;
mod add;

use input::input;
use add::add;
fn main() {
   let  (array, inde, num) = input();
   println!("The original array is: {:?}", &array[0..inde]);
   add(array, num)
}
