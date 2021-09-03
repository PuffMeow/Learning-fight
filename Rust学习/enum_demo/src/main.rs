fn main() {
    let v = Some(0u8);

    match v {
        Some(3) => println!("3"),
        _ => (), //什么也不做
    }

    //这两段代码效果完全一致
    if let Some(3) = v {
        println!("3");
    }
}
