
pub fn iterators () -> Vec<u32> {
    let v = vec![1,3,4,6];
    for i in &v{
        print!("{}",i);
    }
    let iter = v.iter()
                .zip(&v)
                .map(|(item1, item2)| item1 + item2 )
                .collect();
    println!("{:?}",iter);

    for i in &iter {
        print!("{}", i);
    }
    iter
}