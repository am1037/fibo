fn main() {
    let mut vec: Vec<usize> = vec![0, 1];
    for i in 2..100 {
        vec.push(vec[i - 1] + vec[i - 2]);
        println!("{}-th fibo : {} ", i, vec[i]);
    }
}

// fn fibo(n: i32) -> i32 {
//     if n == 0 {
//         return 0;
//     } else if n == 1 {
//         return 1;
//     } else {
//         return fibo(n - 1) + fibo(n - 2);
//     }
// }