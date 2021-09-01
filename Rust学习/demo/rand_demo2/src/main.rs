use rand::Rng;
use std::cmp::Ordering;
use std::io; //引入标准的比较库，Ordering是对两个值进行比较

fn main() {
    println!("猜数游戏开始");
    let rand_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("输入一个数字:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取输入数据");

        //使用模式匹配，当输入的数字产生错误，就跳过本次循环，如果正确就直接将数字给返回
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜的数字是{}", guess);
        //匹配模式，类似于Switch
        match guess.cmp(&rand_number) {
            Ordering::Equal => {
                println!("你猜中啦");
                break;
            } //注意后面是逗号
            Ordering::Greater => println!("猜大了"),
            Ordering::Less => println!("猜小了"),
        }
    }
}
