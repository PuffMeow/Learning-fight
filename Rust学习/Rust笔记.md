### Rust官方学习资料

https://www.rust-lang.org/zh-CN/learn。为什么我要学Rust，首先是因为它下到可以写嵌入式编程，上到可以写Web应用，既足够底层，又拥有一定的应用层开发效率，另外作为一个科班出身的，当年学习C++不精，所以现在想通过Rust弥补一下自己的底层应用开发知识，于是便开始了Rust的学习。俗话说的好：学习一个新东西最好的时间就是过去，以及现在。那我们现在就开始！

### Rust特点

优点：无需GC、安全、性能高、无所畏惧的并发

缺点：难

### 适合领域

- 高性能Web服务器
- WebAssembly
- 命令行工具
- 网络编程
- 嵌入式
- 系统编程
- 游戏开发

### 安装

[Rust官网](https://www.rust-lang.org/tools/install)，右上角可以选择语言，**有中文**。根据你的操作系统会自动给你推荐安装，按照提示按照就行了，安装的过程中会出现选项，选择第一个默认default的选项安装就行，安装完成之后输入查看版本号，显示了就说明安装成功了，这里我的搭建环境是**Windows**，其它系统的安装方式可以自行查看哈。

```
rustup -V
```

除了需要安装RUSTUP-INIT以外，还需要安装 C++ build tools，地址：[C++ build tools](https://visualstudio.microsoft.com/zh-hans/visual-cpp-build-tools/)。安装之后**勾选上工作负荷里面的使用C++的桌面开发**，右边的可选项里只勾上第一个MSVC...就可以了。安装完之后重启电脑就可以进行下一步了。

如果没安装Vs 2013及其以上版本的C++工具的话就会出现以下编译报错

```
error: linker `link.exe` not found
  |
  = note: 系统找不到指定的文件。 (os error 2)

note: the msvc targets depend on the msvc linker but `link.exe` was not found

note: please ensure that VS 2013, VS 2015, VS 2017 or VS 2019 was installed with the Visual C++ option

error: aborting due to previous error
```

### 查看文档

可以查看本地安装的离线文档

```
rustup doc
```

### 开发工具

**macOS/Linux用户首选CLion + Rust插件，VSCode不适合去折腾，这里我们用的是Windows环境，所以使用VS Code，下面文章所有都是基于VS Code来做配置**

```
Visual Studio Code
```

我们前端喜欢用的那玩意儿，然后安装Rust插件，直接插件市场搜Rust安装就完事了，另外一个就是rust-analyzer，这个插件是对代码进行分析用的(下载不了的话需要开一下魔法上网)。

### Hello World

新建一个目录，然后里面创建一个文件叫`main.rs`

main函数具有特殊含义，在Rust里面是可执行程序最先运行的代码，也就是代码的主入口。

println!是Rust的宏(宏是啥后面会讲)，用来进行打印文本，如果是函数的话就没有!

```rust
fn main() {
  println!("hello world");
}
```

编译，控制台输入`rustc main.rs` (c表示complie编译的意思)，然后可以看到编译出的文件

```
main.exe //可执行文件，控制台直接输入.\main执行(在当前文件夹路径之下)
main.pdb //调试信息
```

编译和运行是单独的两个步骤，运行之前必须先进行rustc编译，在windows中会生成main.pdb文件，rustc只适合编写简单的程序。Rust是预编译语言，编译之后得到的exe可执行文件可以直接给别人没有Rust环境的电脑上执行。对于更加复杂的程序，那么我们就需要用到cargo了。

### Cargo

Cargo是Rust的包管理系统和构建工具，类似于NPM。安装Rust的适合就自动安装上了Cargo了。

rust的包管理平台->[Lib.rs](https://www.baidu.com/link?url=N2HEh5QmQ6SqjQwa0h1yhwLCKmyYvQOiYjOyDkFHN9u&wd=&eqid=86471ac00006131600000004612cff14)

命令行输入`cargo -V`可以看到cargo的版本号，说明可以正常工作。

#### 创建项目

```
cargo new cargo_demo
```

接下来会生成一个新的文件夹，里面包含src目录(源代码)和Cargo.toml文件

`cargo.toml`文件是啥？

这是`cargo`的配置文件，即`(Tom's Obvious, Minimal Language)`，类似于Node里面的`package.json`

```
[package]
name = "cargo_demo"  //项目名
version = "0.1.0"   //版本号
edition = "2018"   //Rust的使用版本

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]  //项目依赖项
```

而外部第三方库则称为crate

#### 构建项目

```
cargo build
```

会给你生成一个可执行文件，第一次运行的时候会生成cargo.lock文件，表示项目依赖项的精确版本，类似于package-lock.json，不需要手动修改。

#### 构建并且运行项目

编译+执行，编译后会得到一个target目录，里面包含着编译之后的信息。

```
cargo run
```

#### 代码检查

检查代码，确保代码可以编译通过，但是不产生可执行文件，执行速度比cargo build快得多。

```
cargo check
```

#### 发布构建

编译时会进行优化，但是会构建更长时间，构建完成生成的是target/release而不是target/debug。

```
cargo build --release
```

#### 更新依赖项

```
cargo update
```

会忽略`Cargo.lock`里的依赖版本而去使用最新的依赖项去更新

#### 解决cargo下载依赖速度慢的问题

更换镜像源，windows系统下，按win键搜索powershell并打开

```
cd ~/.cargo
```

然后打开vscode编辑器

```
code .
```

在里面创建一个文件叫做config(没任何后缀)，然后在config里写入配置信息，更换中国科学技术大学ustc的镜像源

```
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = "ustc"
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```

使用其它的创建文件的方式也行，找到.cargo文件夹，在这个文件夹下创建config文件，写上配置信息就行。

#### 安装开发环境

Rust Language Server (RLS)为集成开发环境（IDE）提供了强大的代码补全和内联错误信息功能。

命令行依次执行命令，这几个工具可以给VS Code提供IDE般的体验，包括代码提示，代码告警，函数提示等。

```
cargo install racer
rustup component add rls
rustup component add rust-analysis
rustup component add rust-src
```

### Demo练习

#### 数字游戏1

```rust
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
```

```
猜数游戏开始
猜一个数：
1
猜测的数是：1
```

#### 数字游戏2

Rust并没有提供生成随机数的标准库，所以我们得引入第三方的生成随机数的标准库rand。[Lib.rs](https://www.baidu.com/link?url=N2HEh5QmQ6SqjQwa0h1yhwLCKmyYvQOiYjOyDkFHN9u&wd=&eqid=86471ac00006131600000004612cff14)，在这里面搜索rand。

使用方法：在`Cargo.toml` 里面添加依赖

```toml
[dependencies]
rand = "0.8.4"
```

添加完依赖项之后运行cargo build，会自动安装相关依赖项，如果rand里带有其它的依赖项，也同时会把其它的依赖项安装下来。如果只是更新了源代码，cargo build的时候就不需要编译依赖项了，只是编译源代码。生成随机数代码：

```rust
use rand::Rng; //引入Rand库

fn main() {
    println!("生成随机数");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("打印的随机数是:{}", secret_number)
}
```

#### 数字游戏3

我们这次来随机生成一个数，然后去输入值，看输入的值是否和生成的随机数匹配

```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io; //引入标准的比较库，Ordering是对两个值进行比较

fn main() {
    println!("猜数游戏开始");
    let rand_number = rand::thread_rng().gen_range(1..101);
    println!("随机生成的数字是{}", rand_number);

    println!("输入一个数字:");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("无法读取输入数据");

    //将字符串类型转化成数字类型，然后和生成的随机数做比较，记得先trim去除空格或换行符，parse是将其它类型转换为数字类型。
    //u32类型其实就是无符号整型，后面会讲到类型
    //Rust中可以重复声明，这是一个shadow(隐藏，隐藏之前声明过的变量)变量，在当前作用域中，
    //这里的guess以后的代码会读取到这个guess而不是上面的那个，
    //相当于我们不再关心原变量，而往后我们取的是当前的这个目标量
    let guess: u32 = guess.trim().parse().expect("请输入一个数字");

    println!("你猜的数字是{}", guess);

    //匹配模式，类似于Switch
    match guess.cmp(&rand_number) {
        Ordering::Equal => println!("你猜中啦"), //注意后面是逗号
        Ordering::Greater => println!("猜大了"),
        Ordering::Less => println!("猜小了"),
    }
}

```

#### 循环的多次猜测

我们想要不断的可以输入去猜测，直到猜对了才停止，而不是猜完一次就停止，那要怎么做呢，其实就是加入一个loop循环，猜中了就终止循环

```rust
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

```

### 变量、可变性

- Rust中声明变量使用let(不要理解成和别的语言的const一样，const声明时必须要赋值，并且是常量值，而rust中的let不需要初始赋值，声明和赋值可以分开)，比如说：

  ```rust
  fn main() {
      let x: u32;
      x = 32;
      
      //如果在之后重新赋值,这里就会报错·cannot assign twice to immutable variable `x`，不能对不可变变量重复赋值。
      x = 64;
  }
  ```

- 默认情况，变量不可变(immutable)，除非加上mut

- 常量声明：绑定以后也是不可变的，但和不可变变量有区别：不可以使用mut关键字，声明变量使用const，并且必须标注类型，常量可以在任何作用域声明，全局作用域也行。常量只能绑定到常量表达式，而不能绑定到函数的调用结果或需要运行才能计算出的值。常量在程序运行期间，在声明的作用域内一直有效。Rust中，常量值声明所有都要用大写，比如：

  `const RAND_NUMBER:u32 = 100`

- 隐藏变量(shadow)，可以使用相同的名字声明新变量，新变量会隐藏之前声明过的同名变量。

### 数据类型

Rust是静态类型语言，在编译时必须要知道所有变量的类型，而基于程序使用的值，编译器通常是可以自己推导出的，类型于TS的类型推导。但是在类型转换时类型比较多，例如(String转为整数的parse方法)，就需要添加上类型标注，不然就会编译报错。比如说下面的例子不写:u32就会报错。

```rust
let guess:u32 = "555".parse().expect("不是一个数字类型");
```

Rust中主要有4中标量类型，分别是

- 整数类型：u32(unsign)就是无符号(不区分正负号)整数类型，占据32位(2^32-1)的空间。而有符号整数类型是以i开头，比如i32(integer)，这里可以看下面这个表

  有符号范围是-(2^(n-1))到2^(n-1) - 1

  无符号范围是0到(2^n-1)

  | 长度                                               | 整型  | 无符号整型 |
  | -------------------------------------------------- | ----- | ---------- |
  | 8-bit                                              | i8    | u8         |
  | 16-bit                                             | i16   | u16        |
  | 32-bit                                             | i32   | u32        |
  | 64-bit                                             | i64   | u64        |
  | 128-bit                                            | i128  | u128       |
  | 根据电脑的位数适配，比如电脑是32位或64位，自动适配 | isize | usize      |

  整数溢出概念：比如u8的范围是0-255，如果把u8变量值设为256，调试模式下就会panic(错误机制，译为恐慌)，发布模式下Rust不会检查可能导致整数溢出的panic，如果溢出了会把256变成0，257变成1，程序不会产生panic。

- 浮点类型：f32，32位单精度浮点类型，类似于C++里的float；f64，64位双精度浮点类型，类似于C++的double，Rust浮点类型使用了IEEE-754标准描述，跟我们熟悉的JS是一样的。f64是默认的浮点类型，精度高，且速度和f32差不多，所以采用该类型作为默认。

- 布尔类型：true和false，占用1个字节大小，符号是bool，可以显式声明类型，也可以由编译器默认推导。

- 字符类型：Rust中使用char类型来描述单个字符，字符类型字面量使用单引号，占用4个字节，使用的是Unicode标量值，可以表示比ASCⅡ码多得多的字符，比如拼音，韩文日文、emoji表情等。

### 复合类型

#### Tuple(元组)

可以将多个类型的多个值放到同一个类型里，类似于TS的[string, number, boolean]。Tuple长度固定，声明了就无法再修改，比如说：

```rust
let tup:(char, f64, bool) = ('x', 55, true)
//Rust的解构值
let (x, y, z) = tup
//访问值
println!("{},{},{}",tup.0, tup.1, tup.2)
//类似的，对应到TS就是
let tup:[string, number, boolean] = ['x', 55, true]
//TS的解构
let [x, y, z] = tup
//TS访问值
console.log(`${tup[0]},${tup[1]},${tup[2]}`)
```

#### 数组

数组的数据存放在栈中，而不是堆中，而Vector和数组类型但是比数组更灵活，是Rust中的标准库，数据存在堆中。数组长度不可变，但是Vector长度可变。

数组的类型可以以[类型;长度]这种方式来表示，比如

```rust
let a:[i32;5] = [1,2,3,4,5]
```

