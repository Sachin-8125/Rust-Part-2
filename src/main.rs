fn main() {
    let mut vec = vec![1,2,3,4];
    let ans = even_filter(&vec);
    println!("{:?}",ans);
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32>{
    let mut new_vec = Vec::new();
    for val in vec{
        if val%2==0 {
            new_vec.push(*val);
        }
    }
    return new_vec;

}



