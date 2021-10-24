enum List {
    Node(i32, List),
    Nil,
}

pub fn run() {
    //let a1: [u8; 7000000] = [1; 7000000];

    let mut v1 = vec![1, 2, 3, 4];
    let v2 = vec![5, 6, 7, 8];
    let mut v3 = vec![9, 10];
    println!("Stack address of v1 is: {:p}", &v1);
    println!("Stack address of v2 is: {:p}", &v2);
    v1.insert(1, 10);

    let t1: (i64, String) = (10, String::from("hello"));
    let mut b1 = Box::new(t1);
    (*b1).1 += "world";
    println!("{} {}", b1.0, b1.1);
}
