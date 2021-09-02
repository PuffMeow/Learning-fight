fn main() {
    let s = String::from("hello world");
    let res = first_word(&s);

    println!("res: {}", res);
}

fn first_word(s: &String) -> &str {
    //as_bytes方法会将s字符串转化为字符串数组。bytes的类型是&[u8]
    let bytes = s.as_bytes();
    // (i, &item)是元组类型。iter方法创建迭代器，依次返回数组中每个元素
    //然后调用enumerate这个方法，把iter的结果进行包装，并把每个结果作为元组的一部分进行返回。
    //元组的第一个元素就是enumerate遍历的索引，第二个元素就是数组里的元素。
    //这里实际上用了模式匹配
    for (i, &item) in bytes.iter().enumerate() {
        //判断遍历到的项是否等于空格 b' '(byte),为空格的写法。
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
