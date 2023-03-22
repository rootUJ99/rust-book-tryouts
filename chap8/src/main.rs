fn main() {
    let v1 = vec![1,3,4];
    let mut v2: Vec<u32> = Vec::new();
    v2.push(22);
    // let ele = v1.get(0);
    match v2.get(6) {
        Some(el) => println!("{}",el),
        None => println!("none")
    };
    let v3: Vec<u32> = Vec::from([1,2,4]);
    println!("{:?} {:?} {:?}", v1, v2, v3);
   
    for i in v3 {
        println!("{}", i);
    }

    let s1 = String::from("toto");
    let s2 = String::from("momo");
    let s3 = format!("{} {}" ,s1, s2);
    println!("{} {} {}", s1, s2, s3);
}
