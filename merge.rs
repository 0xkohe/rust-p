type Number = i32;
fn merge(mut left: Vec<Number>, mut right: Vec<Number>) -> Vec<Number> {
   // let mut r:Vec<Number> = Vec::with_capacity(left.len() + right.len());
   let mut r:Vec<Number> = vec![];
   // let mut r = std::collections::VecDeque::<Number>::new();
   while !left.is_empty() && !right.is_empty() {
       if left[0] <= right[0] {
           r.push(left.remove(0));
       } else {
           r.push(right.remove(0));
       }
   }
   if !left.is_empty() {
       r.append(&mut left);
   }

   if !right.is_empty() {
       r.append(&mut right);
   }

   r
}
fn merge_sort(x : Vec<Number>) -> Vec<Number> {
   if x.len() <= 1 {
       return x;
   }
   let mid = x.len() / 2;
   let left = merge_sort(x[..mid].to_vec());
   let right = merge_sort(x[mid..].to_vec());
   merge(left, right)
}
fn main() {
    let x = vec![3, 2, 1, 5, 4];
    let sorted = merge_sort(x);
    println!("{:?}", sorted);
}
