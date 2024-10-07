fn main() {
   let mut nums = vec![1,2,3];
   let iter = nums.iter_mut();

   for val in iter{
    *val = *val+1;
   }
   println!("{:?}",nums);
}





