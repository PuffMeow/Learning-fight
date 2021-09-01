use rand::Rng; //引入Rand库

fn main() {
    println!("生成随机数");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("打印的随机数是:{}", secret_number)
}
