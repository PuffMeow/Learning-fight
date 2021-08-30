use std::io; //prelude预导入模块

fn main() {
    println!("猜数游戏开始");
    println!("猜一个数：");

    //let声明变量(默认let表示变量是不可变的immutable)，mut表示变量是可变(mutable)的
    //双冒号类似于Java里的静态方法，表示创建一个空字符串
    let mut guess = String::new();

    //read_line用来获取用户输入，&是取地址符号，表示参数是引用类型，可以在不同的代码中通过引用访问同一个内存区域
    io::stdin().read_line(&mut guess).expect("无法读取行");

    //{}表示占位符
    println!("猜测的数是：{}", guess);
}
