fn main() {
   let nums = vec![1,2,3,4,5];
   let mut iter = nums.iter();

   while let Some(val) = iter.next(){
    print!("{} ",val);
   }
}





