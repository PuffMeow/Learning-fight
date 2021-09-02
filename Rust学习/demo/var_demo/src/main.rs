fn main() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    //函数执行的结果返回到s3上了
    let s3 = take_and_give_back(s2);

    //这里的s2会报错。因为s2已经被移动到函数里，最后函数执行完，s2就会被drop掉。
    println!("s1:{}, s2:{}, s3:{}", s1, s2, s3)
}

fn gives_ownership() -> String {
    let some_string = String::from("hello1");
    some_string
}

fn take_and_give_back(test_string: String) -> String {
    //这里取得s2的String的所有权，并将它作为结果进行返回
    test_string
}
