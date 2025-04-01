fn map_example() {
    let nums = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = nums.iter().map(|x| x * x).collect();
    println!("map_example: {:?}", squared);
}

fn filter_example() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let even_numbers: Vec<i32> = nums.into_iter().filter(|x| x % 2 == 0).collect();
    println!("filter_example: {:?}", even_numbers);
}

fn fold_example() {
    let nums = vec![1, 2, 3, 4, 5];
    let sum = nums.iter().fold(0, |acc, x| acc + x);
    println!("fold_example: {}", sum);
}

fn skip_example() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let skipped: Vec<i32> = nums.iter().skip(2).cloned().collect();
    println!("skip_example: {:?}", skipped);
}

fn take_example() {
    let nums = vec![1, 2, 3, 4, 5];
    let taken: Vec<i32> = nums.iter().take(3).cloned().collect();
    println!("take_example: {:?}", taken);
}

fn enumerate_example() {
    let nums = vec!["a", "b", "c"];
    for (index, value) in nums.iter().enumerate() {
        println!("enumerate_example: Index {} -> Value {}", index, value);
    }
}

fn chain_example() {
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let combined: Vec<i32> = a.iter().chain(b.iter()).cloned().collect();
    println!("chain_example: {:?}", combined);
}

fn zip_example() {
    let a = vec![1, 2, 3];
    let b = vec!['a', 'b', 'c'];
    let zipped: Vec<(i32, char)> = a.iter().zip(b.iter()).map(|(&x, &y)| (x, y)).collect();
    println!("zip_example: {:?}", zipped);
}

fn main() {
    map_example();
    filter_example();
    fold_example();
    skip_example();
    take_example();
    enumerate_example();
    chain_example();
    zip_example();
}
