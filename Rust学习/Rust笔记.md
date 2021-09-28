### 前言

为什么我要学Rust，首先是因为它下到可以写嵌入式编程，上到可以写Web应用，既足够底层，又拥有一定的应用层开发效率，另外作为一个科班出身的，当年学习C++不精，所以现在想通过Rust弥补一下自己的底层应用开发知识，于是便开始了Rust的学习。俗话说的好：学习一个新东西最好的时间就是过去，以及现在。那我们现在就开始！

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

### 谁在用？



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

数组的数据存放在栈中，而不是堆中，而Vector和数组类型但是比数组更灵活，是Rust中的标准库，数据存在堆中。数组长度不可变，但是Vector长度可变。如果不确定使用数组还是Vec，那么就直接使用Vec

数组的类型可以以[类型;长度]这种方式来表示，比如

```rust
let a: [i32;5] = [1,2,3,4,5]
如果是下面这种形式,就表示数组有5个数，每个数都是3
let b: [3; 5] = [3,3,3,3,3]
```

数组在Stack上分配的单个块内存，使用索引来访问数组元素，比如a[0] = 1。

如果是复杂的数组访问形式，编译时代码能通过，但是运行时会报错，比如

```rust
let index = [10,11,12,13];
let weeks = ["周一", "周二", "周三", "周四", "周五", "周六", "周天"];
//这里编译时不报错，但是运行时就会报错，超出了索引范围
let week = weeks[index[1]];
```

#### 函数

- main函数是程序的入口
- 声明函数使用fn
- Rust不像JS使用驼峰式命名规范，Rust中推荐使用snack case命名规范，即小写字母之间用下划线分割开

函数返回值，默认使用函数体里的最后一个表达式作为返回值，比如说

```rust
//在->后面声明函数返回值的类型
//如果函数想提前返回，可以使用return 并指定一个值。否则默认使用函数体里最后一个表达式当作返回值
fn five() -> i32 {
    let x = 5 + 5; //这是一个语句，把语句和表达式区分开。
    5   //这是一个表达式，记住没有分号;这里相当于 return 5;
}
```

### if else 控制流

和JS的基本一样，还有else if，主要是判断条件那里不需要写括号了。

```rust
fn main() {
    let num = 3;
    
    if num % 4 == 0 {
        println!("能被4除尽");
    } else if num % 3 == 0  {
        println!("能被3除尽");
    } else {
        println!("不是3也不是4的公倍数");
    }
}
```

**如果else if使用过多的话，可以使用match 模式匹配来进行代替。**

三元表达式在Rust中可以这样写

```rust
let condition = true
let num = if condition { 5 } else { 6 }  //这里返回5
```

### 循环

- loop循环：反复(无限循环，类似于while(true)执行一段代码，直到你喊停为止，比如break;

- while循环：类似于其它语言

- for循环：类似于JS的for of，拿到数组里每一个元素。Rust中使用最多的遍历方式。

  ```rust
  fn main() {
      let a = [1,2,3,4,5]
      for item in a.iter() {
          println!("Value is {}", item); //1,2,3,4,5
      }
      
      //另外一种遍历写法,从最后一个元素遍历到第一个元素，反转遍历。[1..6]是左闭右开区间
      for item2 in [1..6].rev() {
          println!("{}", item2);  //5,4,3,2,1
      }
  }
  ```

### 所有权(重点知识)

它可以让Rust不需要GC(Garbage Collector垃圾回收器)，就能保证内存安全，Rust中最独特的特点(核心特性)。其它语言像JS、Java都有自动垃圾回收器，C和C++则需要手动去回收内存。

#### Stack栈和Heap堆

存储在Stack的数据必须拥有已知的固定大小，编译时大小未知或者大小可能发生变化的数据放在Heap堆里，上面提到过的普通数组和Vec就是其中的一个例子。

堆内存组织性要差一点，数据放到堆中时，操作系统会找到一个足够大的空间，标记为在用，并返回一个指针，指针指向这个地址的空间，称为内存分配，其中的指针放在栈中，想要访问实际数据，必须要用指针来进行定位。

访问堆的数据要比访问栈慢得多，因为需要通过指针才能找到堆中的数据，多了一个指针跳转的环节，传达命令需要时间。数据存放地址越近，寻址速度就会快一些(栈就是这样的，栈是连续的地址空间)，地址之间距离越远，访问速度就越慢，指针寻址要时间(堆中存放的数据的地址不一定连续)。

#### 所有权解决的问题

所有权本质上就是用来管理堆中数据的。

- 跟踪代码哪些部分正在使用堆中的数据
- 最小化堆上的重复数据量
- 清理堆中未使用的数据

#### 所有权的三条规则

- 每个值都有一个变量，这个变量就是这个值的所有者，比如说`let x = 0;`，x就是0的所有者。
- 每个值同时只能有一个所有者
- 当所有者超出作用域的时候，值就会被删除

#### 作用域

类似于JS的let声明变量的作用域，当前作用域里的范围，不使用闭包的话其它作用域没法访问。

```rust
fn main() {
    //s不可用
    let s = "hello"; //s可用
    //此后可以对s进行相关操作
} //超出作用域，s不可用
```

#### String类型

String类型比其它的基础类型更复杂，其它的基础数据类型的数据都保存在栈中，而String的数据存放在堆中。

- 字符串字面值，比如`let x = "hello";`，这个字符串字面值是不可变的。
- String在堆中分配内存，可以存储动态长度的字符串数量，是可变的。
- 对于String类型的内存回收：当拥有它的变量走出作用域范围时，内存就会自动回收给操作系统。即调用drop函数去回收内存。

```rust
//创建一个String类型的字符串，::表示from是String命名空间下的函数,这类字符串可以修改。
let mut s = String::from("hello"); 

s.push_str(", world!");

println!("{}", s); //hello, world!
```

#### 变量和数据使用（移动Move）进行交互

```rust
fn main() {
    let x = 5;
	let y = x;
}
```

这里的x和y都是简单数据类型，都被压到了栈中。

而对于String类型

```rust
fn main() {
    let s1 = String::from("hello");
	let s2 = s1;
}
```

存在问题(这里不是Rust所使用的方式，只是说常规的方式)：

- 此时s1赋值给s2，String的数据就被复制了一份，实际上是在栈中复制了一份s1的指针(指向存放s1数据的堆地址)、长度、容量，并没有复制指针所指向的堆中的数据。

- 当变量离开作用域时，Rust会自动调用drop函数，并将变量使用的堆内存释放
- s1和s2离开作用域的时候，都会尝试释放相同的内存：会引起二次释放的bug。

Rust的解决方式，为了保证内存安全：

- Rust不尝试复制被分配的内存
- s1赋值给s2后，s1就失效了，s1离开作用域时，Rust不释放任何东西，如果在s2之后再访问s1，就会报错。

#### 克隆

想对堆中的String数据进行深度拷贝而不仅仅只是浅拷贝栈中的(指针、长度、容量)，可以使用clone方法。

```rust
fn main() {
	let s1 = String::from("hello");
	let s2 = s1.clone();
	println!("{}, {}",s1, s2);
}
```

克隆之后，s1和s2指针指向的堆内存地址就不是一样的了。

#### 复制

而对于栈中的数据，我们一般称为复制而不叫克隆。下面的例子就是复制：

```rust
fn main() {
	let x = 5;
	let y = x;
}
```

- **Copy trait(复制特性)**，可以像整数这样完全存放在stack上面的类型
- 如果一个类型实现了Copy这个trait，那么旧变量可以在赋值给其它变量后继续使用，而不会被回收。
- 如果一个类型或者该类型的一部分使用了Drop trait，那么Rust就不允许它继续实现Copy trait了。

拥有Copy trait的类型：

- 任何简单的组合数据类型都可以实现Copy trait
- 需要分配内存或资源的类型都不可以实现Copy trait，而是Drop trait
- 拥有Copy trait的类型：所有整数类型，比如u32、bool类型、char类型、浮点类型、Tuple(前提是里面的所有数据都是可以实现Copy trait的，并且元组数量不能超过12)

#### 所有权和函数

下面看两个例子，为引用的概念做铺垫。

```rust
fn main() {
    let s = String::from("hello world");

    take_string(s);

    //这里s会报错，因为s的所有权被转移到了take_string函数里，然后函数执行完，就会调用drop函数，把s的内存释放掉。
    println!("s:{}", s);

    let x = 5;

    //这里不会发生什么特殊的事情，因为传递进去的x是通过复制进去的。
    take_var(x);
}

fn take_string(test_string: String) {
    println!("{}", test_string);
}

fn take_var(test_var: i32) {
    println!("{}", test_var)
}

```

```rust
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

```

#### 返回值和作用域

- 函数在返回的时候也会发生所有权的转移
- 把一个值赋值给其它变量的时候所有权会发生移动
- 当一个包含堆数据的变量离开作用域时，它的值会被drop函数清理掉，除非所有权移动到了另一个变量上。

#### 如何让函数使用某个值，但是又不获得所有权？ ->使用引用

这里就提出了引用(reference)的概念。引用允许你使用某个值而不获得它的所有权。

```rust
fn main() {
    let s1 = String::from("hello");

    //这里calc_length函数传递进去的s1，只是s1的引用，所以所有权不会进行移动。外面的s1的所有权依旧存在。
    let len = calc_length(&s1);

    println!("{}的长度是{}", s1, len); //hello的长度是5
}

fn calc_length(test_string: &String) -> usize {
    test_string.len()
}
```

#### 借用

把引用作为函数参数的这种行为就称为借用，可以看上面的例子，把s1借用到了calc_length函数里，但是没获得它的所有权。**借用到函数里面的变量是不可以修改的，和变量一样，引用默认也是不可修改的(imutable)，但是如果声明可变引用的话则引用是可以修改的**

```rust
fn main() {
    let mut s1 = String::from("hello");

    let len = calc_length(&mut s1);

    println!("{}的长度是{}", s1, len); //hello,world的长度是11
}

fn calc_length(test_string: &mut String) -> usize {
    //因为传的是可变引用，所以这里可以修改
    test_string.push_str(",world");
    test_string.len()
}
```

可变引用的限制 ： 在特定作用域内，对某一块数据，只能有一个可变的引用。**这样做的好处是可以在编译时防止数据产生竞争。**

```rust
fn main() {
    let mut s = String::from("hello");

    let s1 = &mut s;
    //这里就会报错cannot borrow `s` as mutable more than once at a time
    //意思就是不能同时对s进行超过一次的可变引用。
    let s2 = &mut s;

    println!("s1:{}, s2:{}", s1, s2);
}
```

数据竞争会发生的情况，这些行为在运行时是很难发现的，所以就在编译时就防止这种现象出现。

- 两个或多个指针同时访问同一个数据
- 至少有一个指针用于写入数据
- 没有使用任何机制来同步对数据的访问

可以通过创建新的作用域来允许非同时创建多个可变引用：

```rust
fn main() {
    let mut s = String::from("hello");

    let s1 = &mut s;

    {
        let s2 = &mut s;
    }
}
```

另外的限制：

- 不可以同时拥有一个可变引用和一个不可变的引用，多个不变的引用则是可以的

```rust
fn main() {
    let mut s = String::from("hello");

    let s1 = &s;
    let s2 = &s;
    // 这里报错cannot borrow `s` as mutable because it is also borrowed as immutable。
    //不能借用已经被不可变引用的s再进行可变引用。
    let s3 = &mut s;

    println!("s1:{}, s2:{}, s3:{}", s1, s2, s3)
}
```

#### 悬空引用

概念：一个指针引用了内存中的某个地址，而这块内存可能已经被释放并分配给别人使用了。

而Rust编译器会保证引用永远都不会产生悬空引用

- 如果引用了某些数据，编译器会保证引用离开作用域之前数据不会离开作用域。

```rust
fn main() {
    let s = test();
}

//这里报错：expected named lifetime parameter，期待声明一个生命周期的参数
//生命周期后面具体再讲
fn test() -> &String {
    let s = String::from("hello");
    &s
}
```

#### 引用的规则

任何给定时刻，只能满足以下两个条件之一：

- 一个可变的引用
- 任意数量不可变的引用

引用必须要一直有效。

#### 切片(一种不持有所有权的数据类型，slice)

字符串切片是指向字符串中**一部分内容的引用**。使用方式[开始索引..结束索引) ，左闭右开区间。

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5]; //语法糖&[..5]
    let world = &s[6..11]; //语法糖&[6..]

    //获取所有字符串,相当于[0..s.len()]
    let whole = &s[..];

    println!("{}, {}, whole: {}", hello, world, whole); //hello, world, whole: hello world
}
```

测试一道题：

- 接收字符串作为参数
- 返回它在这个字符串中找到的第一个单词
- 如果函数没找到任何空格，那么将整个字符串返回

```rust
fn main() {
    let s = String::from("hello world");
    let res = first_word(&s);

    println!("res: {}", res);
}

//返回类型&str是字符串切片的类型。
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
```

### Struct 结构体

#### 基本使用

自定义的数据类型，类似于TS的interface。

```rust
struct User {
    username: String,
    email: String,
    age: usize,
    active: bool,
} //这里不需要分号

fn main() {
    //如果需要user1里的字段可变，可以声明 let mut user1。那么里面的内容都是可以修改的。
    let user1 = User {
        username: String::from("王大锤"),
        email: String::from("xxxxxx@qq.com"),
        age: 13,
        active: true,
    };

    println!(
        "username: {}, email: {}, age: {}, active: {}",
        user1.username, user1.email, user1.age, user1.active
    );
    //username: 王大锤, email: xxxxxx@qq.com, age: 13, active: true
}
```

Struct也可以作为函数的返回值，字段名和字段值对应变量名相同时，可以简写，类似于JS

```rust
fn build_user(email:String, username: String) -> User {
    User {
        email,
        username,
        age:13,
        active:true
    }
}
```

#### struct更新语法

```rust
fn main() {
    //如果需要user1里的字段可变，可以声明 let mut user1。那么里面的内容都是可以修改的。
    let user1 = User {
        username: String::from("王大锤"),
        email: String::from("xxxxxx@qq.com"),
        age: 13,
        active: true,
    };

    let user2 = User {
        username: String::from("大锤giegie"),
        email: String::from("xxxx@163.com"),
        ..user1
    };
}
```

#### Tuple Struct(结构体元组)

可以定义类似Tuple元组的Struct。

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
let white = Color(255,255,255);
let origin = Point(0,0,0);
```

#### Struct数据的所有权

```rust
struct User {
    username: String,
    email: String,
    age: usize,
    active: bool,
}
```

- 这里的字段使用了String而不是&str
- 只要struct实例是有效的，那么里面的字段数据也是有效的
- Struct里也可以存放引用，但是需要使用生命周期(后面补充)

- 如果struct里存放引用但不使用生命周期就会报错。

#### Demo

计算长方形面积

```rust
//对这个结构体使用调试模式，这个注解实际上使用的是std::fmt::Debug
#[derive(Debug)] 
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 10,
    };

    println!("area is {}", area(&rect));

    //这里是为了能将rect里的信息更好的打印出来，对应的结构体上面记得加#[derive(Debug)]，加#号会换行
    println!("{:#?}", rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

#### stuct的方法

- 方法和函数类似
- 不同之处：方法在struct中(或enum、trait对象)的上下文定义里

- 方法第一个参数总是self，表示方法被调用的struct实例，后续跟着的是其它参数。

方法定义

- 在impl块里定义方法
- 方法第一个参数是&self时是引用，不获得所有权，如果写self会获得当前结构体所有权，也可以使用mut关键数字实现可变借用

使用方法来重写上面的例子

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    //引用Rectangle结构体
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 10,
        height: 10,
    };

    println!("area is {}", rect.area());

    println!("{:#?}", rect);
}
```

#### 方法调用运算符

Rust会在调用方法时自动引用或解引用。

调用方法时，Rusthui根据情况自动添加&、&mut或*(解引用符号)，以便object可以匹配到方法的签名，比如说下面两行代码效果相同。

```
p1.distance(&p2);
(&p1).distance(&p2);
```

#### 关联函数

可以在impl块里定义不把self作为第一个参数的函数，叫关联函数(不叫方法了)，类似于别的语言的静态方法static。比如我们之前用得很多的`String::from()`就是一个关联函数。

- 关联函数常用于构造器

看个例子，下面就是关联函数的例子。

```rust
#[derive(Debug)] //对这个结构体使用调试模式，这个注解实际上使用的是std::fmt::Debug
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect = Rectangle::square(10);
    println!("{:?}", rect); //Rectangle { width: 10, height: 10 }
}
```

到了现在，我们再来说说我们之前一直在使用到的两个冒号，::，这是干嘛用的呢？

- 可以使用于关联函数
- 模块创建的命名空间(后面讲)

#### 多个impl块

每个struct结构体可以拥有多个impl块。比如同名的impl块，里面放不同的方法(同一种类型的抽象可以这样但没必要)。



### 枚举

允许我们列举所有可能的值来定义一个类型，比如性别：要么是男、要么是女。(人妖？？？)，这就是一种枚举类型

```rust
enum Gender {
	Boy,
	Girl,
}
```

#### 枚举值

```rust
enum Gender {
	Boy,
	Girl,
}
//枚举的变体都位于标识符的命名空间下，使用两个冒号分隔开
let boy = Gender::Boy;
let girl = Gender::Girl;
```

#### 将数据附加到枚举的变体中

```rust
enum Gender {
    Boy(String),
    Girl(String),
}

//或者
enum Gender {
    Boy(String, String, String),
    Girl(String),
}

fn main() {
    let boy = Gender::Boy( "大锤", "中风", "小明");
    let girl = Gender::Girl("小红");
}
```

- 优点：不需要使用额外的结构体struct
- 每个变体都可以拥有不同(任意)的类型以及关联的数据量

```rust
#[derive(Debug)]
enum Gender {
    Boy(String, String, String),
    Girl(String),
}

fn main() {
    let boy1 = String::from("大锤");
    let boy2 = String::from("中风");
    let boy3 = String::from("小明");
    let girl1 = String::from("小红");

    let boy = Gender::Boy(boy1, boy2, boy3);
    let girl = Gender::Girl(girl1);

    //boys are: Boy("大锤", "中风", "小明"), girl is: Girl("小红")
    println!("boys are: {:?}, girl is: {:?}", boy, girl);
}
```

#### 为枚举定义方法

类似于给结构体定义方法，使用impl关键字。

```rust
enum Gender {
    Boy(String, String, String),
    Girl(String),
}

impl Gender {
    fn run(&self, gender: String) {
        println!("{} 跑得很快", gender);
    }
}

fn main() {
    let boy1 = String::from("大锤");
    let boy2 = String::from("中风");
    let boy3 = String::from("小明");
    let girl1 = String::from("小红");

    let boys = Gender::Boy(boy1, boy2, boy3);
    let girl = Gender::Girl(girl1);

    boys.run(String::from("男孩们")); //男孩们 跑得很快
    girl.run(String::from("小女孩")); //小女孩 跑得很快
}
```

#### Option枚举

- 定义在标准库中
- 在prelude(预导入模块)中，不需要用户手动导入
- 描述了某个值可能存在或不存在的情况

Rust中没有Null(表示没有值)这个概念，Null的概念实际上是因为某种原因变为无效或缺失的值，那么Rust使用什么代替呢？

Rust提供了类似于Null概念的枚举 Option<T>，T是泛型，泛型的用法类似于其它语言，后面具体讲。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

看个例子

```rust
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let z: Option<i8> = None;

    //报错cannot add `std::option::Option<i8>` to `i8`
    let sum = x + y;
}
```

在这里，Option<i8>和i8是两个不同的类型，不可以直接相加，比Null来说更安全。

#### 枚举和模式匹配的控制流运算符match

这是现代化语言的精髓部分

- 允许一个值和一系列模式进行匹配，并执行匹配的模式对应的代码
- 模式可以是字面值、变量名、通配符

```rust
#[derive(Debug)]
enum UsState {
    Sanfrancisco,
    Alaska,
}

enum Coin {
    Penny,            //一便士
    Nickel,           //五美分
    Dime,             //十美分
    Quarter(UsState), //二十五美分
}

fn value_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("美国的{:?}州用的25美分硬币", state); //美国的Sanfrancisco州用的25美分硬币
            25
        }
    }
}

fn main() {
    let q = Coin::Quarter(UsState::Sanfrancisco);
    println!("{}", value_in_coin(q)); //25
}
```

match 必须穷举所有可能的值

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        //如果不加None的模式匹配就会报错，必须穷举所有可能的值
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let one = Some(1);
    let two = plus_one(one);
    let none = plus_one(None);
}
```

如果不想列出所有可能的值时可以使用_(下划线)通配符，替代其它没列出的值

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    //不需要穷举所有的可能性了，全部默认返回Some(0)
    match x {
        _ => Some(0),
    }
}

fn main() {
    let one = Some(1);
    let two = plus_one(one);
    let none = plus_one(None);
}
```

#### if let

处理只关心一种匹配而忽略其它匹配的情况

```rust
fn main() {
    let v = Some(0u8);

    match v {
        Some(3) => println!("3"),
        _ => (), //什么也不做
    }

    //这两段代码效果完全一致,if let 只关心一种模式匹配，但是放弃了穷举的可能，可以看作是match的语法糖
    if let Some(3) = v {
        println!("3");
    }
}
```

也可以使用else来进行其它可能性的返回

```rust
fn main() {
    let v = Some(0u8);

    match v {
        Some(3) => println!("3"),
        _ => println!("others"),
    }

    if let Some(3) = v {
        println!("3");
    } else {
        println!("others");
    }
}
```

### 模块系统

**Package（包）、Crate（单元包）、Module（模块）**

- Package(最顶层)：Cargo特性，让你构建、测试、共享Crate
- Crate(处于Package层级下)：一个模块树，可以产生一个library或可执行文件
- Module：使用use关键字，让你控制代码的组织、作用域、私有路径。
- Path：路径为struct、function或module等项命名的方式

**Crate的类型**

- binary(二进制Crate)
- libary(库Crate)

**Crate Root(单元包的根)**

- 是源代码文件
- Rust编译器从这里开始，组成Crate的根Module

**一个Package(使用cargo new project-name命令创建出来的那个文件夹就叫Package)**

- 包含1个Cargo.toml，描述如何构建这些Crates
- 只能包含0-1个libary crate
- 可以包含任意数量的binary crate
- 必须至少有一个crate

**如果Package里的src文件夹里有一个main.rs文件，这就是binary crate(二进制单元包)的根入口**。

**如果Package里的src文件夹有lib.rs文件，这就是library crate(库单元包)的根入口。**

- crate名和package名相同

Cargo把crate root文件交给rustc(Rust编译器)来构建library(库)或者binary(二进制文件)。

#### Cargo惯例

- 一个Package可以同时含有src/main.rs和src/lib.rs，名称都和Package名相同
- 一个Package可以有多个binary crate，源代码文件放在src/bin目录，里面的每个文件都是单独的binary crate

#### Crate的作用

- 将相关功能组合到一个作用域内，便于在项目间进行共享
- 在Crate中可以定义Module将代码进行分组，提高模块复用性，并且能控制项目的私有性

#### Module

可以理解为文件夹里的文件，也类似于其它语言的命名空间的概念

- 在Crate内创建，使用关键字mod
- 可以进行嵌套
- Module里可以进行其它项(struct、enum、常量、trait、函数等)的定义

```rust
mod top_module {
    mod inner_module {
        fn test_fn() {}   
    }
}
```

当前的目录组织结构是这样的

```rust
└─crate
    ├─ Cargo.toml
    ├─ src
    ├─── main.rs
    ├─── lib.rs
    ├───── top_module
    ├─────── inner_module
    ├───────── test_fn
```

src/main.rs和src/lib.rs叫做crate单元包的根，这两个文件(任意一个)的内容形成了crate模块，位于整个模块树根部

#### 路径（Path）

两种形式：

- 绝对路径：从crate root开始找，使用crate名或字面值crate
- 相对路径：从当前模块开始找，使用self，super或当前模块的标识符

路径至少由一个标识符组成，标识符之间使用::

#### 访问权限

Rust中所有的（函数、方法、stuct、enum、模块mod、变量）默认都是私有的，如需暴露属性出去，可以使用**pub关键字**，

- 父级模块不能访问子模块的私有属性

- 子模块可以使用所有祖先模块中的属性

```rust
mod top_module {
  pub mod inner_module {
    pub fn test_fn() {
      println!("hello, world");
    }
  }
}

pub fn test() {
  //使用绝对路径
  crate::top_module::inner_module::test_fn();

  //使用相对路径
  top_module::inner_module::test_fn();
}
```

#### use关键字

将路径导入到当前的作用域内，遵循私有制原则

```rust
mod top_module {
  pub mod inner_module {
    pub fn test_fn() {
      println!("hello, world");
    }
    fn private_fn() {}
  }
}

//如果use前面也加上pub叫做重导出，那么外部的代码也可以引入模块到他们自己的作用域
use crate::top_module::inner_module; 

pub fn test() {
  inner_module::test_fn();
  //不能引用私有的函数
  inner_module::private_fn();
}
```

#### 标准库

```rust
//use std::这里有一些标准库。
//比如下面的写法可以拿到PI的常量值。
use std::f64::consts::PI;
```

#### as关键字

存在两个相同的名称，且同样需要导入，我们可以使用 as 关键字为标识符添加别名

```rust
mod top_module {
  pub mod inner_module {
    pub fn test_fn() {
      println!("hello, world");
    }
    fn private_fn() {}
  }
}

use crate::top_module::inner_module::test_fn;
use crate::top_module::inner_module::test_fn as nation_test_fn;

pub fn test() {
  test_fn();
  nation_test_fn();
}
```

#### 使用外部包

在前面几章我们引入那个rand包其实已经有过演示了。

- 在Cargo.toml添加依赖的包版本号
- 使用use将包内的属性引入到作用域中

标准库std也是属于外部包，但是我们不需要修改Cargo.toml的依赖项来包含std

**嵌套包的引用**

```rust
use std::io;
use std::io::Write;

//可以替代为
use std::io::{self, Write}
----------

use std::cmp::Ordering;
use std::io;
//代替为
use std::{cmp::Ordering, io};
```

#### 通配符*

使用*可以把路径内所有可以公共访问的属性都引入到该作用域内

比如`use std::*`

但是要**谨慎使用**

应用场景：

- 测试，将所有测试代码同时引入
- 有时被用于预导入(prelude)模块

#### 将模块拆分到其它文件中

```rust
// main.rs
mod top_module;

fn main() {
    println!("main_module.");
    println!("{}", top_module::test());
}
```

```rust
// top_module.rs
pub fn test() -> String {
    String::from("top_module")
}
```

### 常见的集合

#### Vector容器

##### 概念

存储在堆内存的数组，可以动态扩容，写法Vec<T>

- 标准库提供
- 可存储多个值
- 只能存储相同类型
- 在内存中连续存放

```rust
fn main() {
    //使用泛型显式声明Vec里存储的数据类型
    let arr: Vec<i32> = Vec::new();
    
    //使用初始值创建，使用vec!宏, 编译器可以自动推导出Vec里存储的数据类型
    let arr2 = vec![1, 2, 3];
}
```

##### 插入数据使用push方法

```rust
fn main() {
  let mut arr = Vec::new();
  //往Vec添加数据
  arr.push(1);
}
```

##### 读取值，使用索引或者get方法

```rust
fn main() {
  let arr = vec![1, 2, 3, 4, 5];
  let second = &arr[1];
  println!("第二个元素是：{}", second); //第二个元素是：2

  //get方法传索引值，使用get时，如果索引值超出范围会比较安全，会走下面的None逻辑
  match arr.get(1) {
    Some(second) => println!("能拿到第二个元素:{}", second), //能拿到第二个元素:2
    None => println!("拿不到第二个元素"),
  }
}
```

##### Vec的所有权借用

```rust
fn main() {
  let mut arr = vec![1, 2, 3, 4, 5];
  let first = &arr[0]; //不可变的引用

  //这里会报错cannot borrow `arr` as mutable because it is also borrowed as immutable
  arr.push(6);

  println!("第一个元素的值是: {}", first);
}
```

为什么这里会报错呢，因为我们first借用的是第一个元素，下面arr.push新添加一个元素，又因为Vec在内存中是连续存放的，可能新添加进来的一个6会遇到地址不够的情况，那就得进行重新分配，得去找另一块连续地址的内存去存放，但是我们的first还是指向的原来的第1个元素的地址，就会出现问题了。

遍历Vec的值并修改里面的每一个元素

```rust
fn main() {
  let mut arr = vec![1, 2, 3, 4, 5];
  for i in &mut arr {
    //解引用才能去修改值
    *i += 1;
    println!("遍历修改后的值是:{}", i);
  }
}

// 遍历修改后的值是:2
// 遍历修改后的值是:3
// 遍历修改后的值是:4
// 遍历修改后的值是:5
// 遍历修改后的值是:6
```

如果想要在Vec中存储不同的数据类型，可以使用enum

- enum的变体可以附加不同的数据类型
- enum的变体定义在同一个enum类型下

```rust
enum ArrType {
  Int(i32),
  Float(f32),
  Text(String),
}

fn main() {
  let arr = vec![
    ArrType::Float(1.2),
    ArrType::Int(10),
    ArrType::Text(String::from("hello world"))
  ]
}
```

#### String

##### 概念

Rust中的字符串可能会比较让人困扰，在Rust中它并不简单

- Rust倾向于暴露可能的错误
- 字符串数据结构复杂
- 使用了UTF-8编码

字符串是基于byte的集合，里面提供了一些方法能将byte解析为文本。

Rust在核心语言层面只有一种字符串类型：字符串切片str( &str)。它是对存储在其它地方、UTF-8编码的字符串的引用。字符串字面值存储在二进制文件中。

标准库中的String类型：内容可增长、可修改、可获得所有权

通常所说的字符串是String或者&str

##### 创建字符串

其实在这里之前我们就已经有过很多字符串的实践了，比如创建一个字符串`String::new()`。

使用初始值来创建字符串的话可以使用`to_string()`，可用于实现了Display trait的类型，包括字符串字面值。

```rust
fn main() {
  //如果直接创建字面量字符串，它的类型是&str。
  let str = "hello world";
   
  //下面两种创建字符串的方法效果是一样的，可根据个人喜好选择写法。
  //而使用to_string方法，可以将它转变为String类型
  let str2 = "hello world".to_string();
  //使用String::from来创建String类型的字符串。
  let str3 = String::from("hello world");
}
```

##### 更新String

- push_str方法：把一个字符串切片附加到String

- push方法：把单个字符附加到String中

- +运算符：这个方法实际上使用了类似于fn add(self, &str) -> String的方法。标准库中的add方法是使用了泛型的，所以这里说是类型于，这个方法只能把&str类型添加到String中，这里我们传入的&s2是String的引用，这个方法使用解引用强制转换将String转换成了字符串切片&str，所以可以调用成功。s1的所有权转移到add函数里，所以add函数执行完s1就失效了。

  ```rust
  fn main() {
      let s1 = String::from("hello");
      let s2 = String::from(" world");
    
      let s3 = s1 + &s2;
      
      println!("{}", s3); //hello world
      println!("{}", s1); //s1不能再使用了
      println!("{}", s2); //s2可以继续使用
  }
  ```

- format!宏：连接多个字符串

  ```rust
  fn main() {
    let s1 = String::from("打");
    let s2 = String::from("飞");
    let s3 = String::from("鸡");
  
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s); //打-飞-鸡
  }
  ```

##### String类型的索引访问

String类型的内部实现其实就是对Vec<u8>的包装。

String类型不支持索引方式访问，比如`str[0]或者&str[0]`都不能对str字符串进行索引访问。因为索引操作的时间复杂度是O(1)，而String不能保证这个O(1)的时间复杂度，因为它需要去遍历所有内容来确认有多少合法的字符。

Rust中有三种看待字符串的方式：

- 字节
- 标量值(Unicode标量值)
- 字形簇(最接近所谓的"字母")：Rust标准库中没有实现，可以使用第三方库

```rust
fn main() {
  let s = "中文";

  //这里使用的是字节的概念
  for i in s.bytes() {
    println!("{}", i);
  }
  // 一个中文用三个字节表示
  // 228
  // 184
  // 173
  // 230
  // 150
  // 135

  //这里使用的是标量值的概念
  for i in s.chars() {
    println!("{}", i);
  }

  // 中
  // 文
}
```

##### 切割String

这里之前其实也讲了，就是字符串切片。在多语言环境中会容易出现问题

```rust
fn main() {
  let str = "asdfghjkl";
  println!("{}", &str[..2]); //as
}
```

```rust
fn main() {
  let str = "中文";
  //中
  println!("{}", &str[..3]);
  //因为一个中文表示三个字节类似于(a,b,c)(a,b,c)，切割不按照边界切的话就会编译出错
  //'byte index 4 is not a char boundary; it is inside '文' (bytes 3..6) of `中文`'
  println!("{}", &str[0..4]); //这里编译报错panicked
}
```

#### HashMap

##### 概念

HashMap<K, V>，K是key，V是value，类似于JS的Map。以键值对的形式存储数据，**数据存储在堆中**，HashMap是同构的，就是所有K必须是同种类型，所有V也必须是同种类型。

适用于使用key来寻找值

##### 创建

- 因为HashMap用的比较少，所以不在prelude预导入模块中，所以需要我们手动引入

- 标准库对其支持较少，没有内置的宏来创建HashMap

- 初始化没有数据时需要显式声明类型，不显式声明时可以使用insert方法来添加一条数据，会自动推导出hashMap的类型

```rust
use std::collections::HashMap;

fn main() {
  let mut map: HashMap<String, i32> = HashMap::new();
    
  //或者
  let mut map2 = HashMap::new();
  map2.insert(String::from("0"), 100);
}
```

##### Collect方法创建HashMap

在元素类型为Tuple的Vector上使用collect方法，可以组建一个HhashMap

- 要求Tuple有两个值，一个作为Key，一个作为Value
- collect方法可以把数据整合成很多种数据集合类型，包括HashMap，返回值需要显式声明

```rust
use std::collections::HashMap;

fn main() {
  let colors = vec![String::from("red"), String::from("pink")];
  let items = vec![10, 20];

  // 遍历items和colors，调用zip方法创建一个元组，就是拉链的意思
  //这两个Vector像拉链一样拉形成了一个元组的数组，最后使用collect生成hashMap
  //collect可以生成很多种数据结构，所以必须要在前面显式声明数据类型
  //两个下划线占位符，Rust能推导出Key和Value的数据类型
  let map: HashMap<_, _> = items.iter().zip(colors.iter()).collect();
}
```

##### HashMap和所有权

- 对于实现了Copy trait的类型(i32等存放在栈中的基础数据类型)，值会被复制到HashMap中

- 对于拥有所有权的值（如String等存在堆中的数据类型），值会被移动，所有权会转移给HashMap
- 如果将值的引用插入到HashMap中，值本身不移动

```rust
use std::collections::HashMap;

fn main() {
  let s1 = String::from("hello");
  let s2 = String::from("world");

  let mut map = HashMap::new();
  map.insert(s1, s2);

  //因为s1、s2的所有权已经转移到HashMap里了，所以这里的s1和s2就已经失效了
  //borrow of moved value: `s1`，`s2`
  //如果上面改成将引用传入，那么s1和s2就不会失效
  println!("{}, {}", s1, s2);
}
```

##### 访问HashMap中的值

get(key: K) -> Option<&V>

```rust
use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert(String::from("red"), 10);
  map.insert(String::from("pink"), 20);

  let red = String::from("red");
  let val = map.get(&red);

  match val {
    Some(s) => println!("{}", s),
    None => println!("HashMap中没有该值"),
  }
}
```

##### 遍历

```rust
use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert(String::from("red"), 10);
  map.insert(String::from("pink"), 20);

  for (k, v) in &map {
    println!("k: {} , v: {}", k, v);
  }
}
// k: pink , v: 20
// k: red , v: 10
```

##### 更新

- HashMap大小可变
- 每个K只能对应一个V

更新策略：

- 替换原有的V：就是两次插入的键相同时，第二次插入的值会覆盖掉第一次的值

- 添加：HashMap中没有K，那么我们才去插入V，我们需要使用entry方法(检查指定的K是否对应一个V，参数为K，返回值enum Entry代表值是否存在)

  ```rust
  use std::collections::HashMap;
  
  fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("red"), 10);
    map.insert(String::from("pink"), 20);
  
    //blue的键不存在时就插入100
    map.entry(String::from("blue")).or_insert(100);
  
    //{"red": 10, "pink": 20, "blue": 100}
    println!("{:?}", map);
  }
  ```

- 基于现有的K更新V：

  ```rust
  use std::collections::HashMap;
  
  fn main() {
    let text = "hello world today is a good day";
    let mut map = HashMap::new();
  
    //按空格分割的遍历器
    for word in text.split_whitespace() {
      //判断遍历到的单词在Map里是否存在，不存在就插入0
      //这里的count得到的是插入值的引用
      let count = map.entry(word).or_insert(0);
      //解引用然后将插入的值+1
      *count += 1;
    }
  
    // {"day": 1, "is": 1, "world": 1, "hello": 1, "today": 1, "good": 1, "a": 1}
    println!("{:#?}", map);
  }
  ```

##### Hash函数

默认情况下，HashMap使用加密功能强大的Hash函数，可以抵挡拒绝服务(Dos)攻击。

- 它不是可用的最快的Hash算法
- 但具有更好的安全性

- 可以指定不同的Hasher来切换到另一个函数，Hasher是实现BuildHasher trait的类型

### 错误处理

#### panic!宏 - 不可恢复的错误

Rust的可靠性依赖错误处理，Rust没有类似异常的机制

错误的分类：

- 可恢复：例如文件没找到，可以再次尝试。使用 Result<T, E>

- 不可恢复的错误：例如索引超出范围，使用panic!宏

  当panic!宏执行，程序会打印一个错误信息，并展开、清理调用栈，最后退出程序

为应对panic，展开或中止(abort)调用栈。

- 默认情况下，panic发生时Rust沿着调用栈往回走、清理每个遇到的函数中的数据。或者立即中止调用栈、不进行清理，直接停止程序，内存需要操作系统进行回收。

- 想让二进制文件更小，可以把默认的展开改成中止，在Cargo.toml文件

  ```rust
  [dependencies]
  
  [profile.release]
  panic = 'abort'
  ```

手动编写panic，下面这段代码运行cargo run 时会引起panic。数组访问越界的时候也会产生panic

```rust
fn main() {
  panic!("crash");
}
//thread 'main' panicked at 'crash', src\main.rs:2:3
//note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

- 将环境变量设置为RUST_BACKTRACE=1展示回溯信息
- Mac中直接执行RUST_BACKTRACE=1
- windows在cmd中执行 set RUST_BACKTRACE=1
- windows在powershell中执行 $env:RUST_BACKTRACE=1

#### Result枚举

```rust
enum Result<T,E> {
    //里面有两个变体
    Ok(T),
    Err(E),
}
```

- T：操作成功的情况下，Ok变体里返回的数据的类型
- E：操作失败的情况下，Err变体里返回的数据的类型

比如我们打开系统的一个文件，有成功和失败两种情况

```rust
use std::fs::File;

fn main() {
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => {
      panic!("文件没找到,错误信息: {:?}", error);
    }
  };
}

//thread 'main' panicked at '文件没找到,错误信息: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }'
```

#### unwrap方法

match表达式的一种快捷写法，缺点就是不能自定义调试信息

- 如果Result结果是Ok，返回Ok里面的值
- 如果Result结果是Err，调用panic!宏

上面的那段代码可以像这样写

```rust
use std::fs::File;

fn main() {
  let f = File::open("hello.txt").unwrap();
}
//thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }'
```

#### expect

可以自定义调试信息

```rust
use std::fs::File;

fn main() {
  let f = File::open("hello.txt").expect("找不到文件 hello.txt");
}
//thread 'main' panicked at '找不到文件 hello.txt: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }'
```

#### 传播错误

将错误返回给调用者，像标准库的File::open和File::create等方法的错误都属于传播错误，需要我们自己手动处理

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_str_from_file() -> Result<String, io::Error> {
  let f = File::open("hello.txt");

  let mut f = match f {
    Ok(file) => file,
    Err(err) => return Err(err),
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}

fn main() {
  //这里就拿到函数里传播出来的错误
  //thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound,     //message: "系统找不到指定的文件。" }'
  let res = read_str_from_file().unwrap();
}
```

#### ?运算符

传播错误处理的一种快捷写法（语法糖，效果跟上面完全一样）

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_str_from_file() -> Result<String, io::Error> {
  let mut f = File::open("hello.txt")?;

  // let mut f = match f {
  //   Ok(file) => file,
  //   Err(err) => return Err(err),
  // };

  let mut s = String::new();
  f.read_to_string(&mut s)?;

  // match f.read_to_string(&mut s) {
  //   Ok(_) => Ok(s),
  //   Err(e) => Err(e),
  // }
    
  Ok(s)
}

fn main() {
  let res = read_str_from_file().unwrap();
}
```

继续优化简洁，使用链式调用的方式简化代码

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_str_from_file() -> Result<String, io::Error> {
  let mut s = String::new();
  File::open("hello.txt")?.read_to_string(&mut s)?;
  Ok(s)
}

fn main() {
  let res = read_str_from_file().unwrap();
}
```

#### ?运算符和main函数

?运算符只能用于返回的值类型是Result的函数

- main函数返回类型是()
- main函数的返回类型也可以是：Result<T, E>
- Box<dyn Error>是trait对象，理解为任何可能的错误类型

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
  let f = File::open("hello.txt")?;

  Ok(())
}
```

#### 什么时候应该用panic

总体原则

- 在定义一个可能失败的函数时，优先考虑返回Result
- 某种情况肯定是不可恢复的，那我们就使用panic!

比如说我们编写一个代码库，一些逻辑是肯定会引起不可恢复的，那我们可以使用panic!自己去处理代码错误，其它时候大部分都是要将错误传播出去给调用者自己去处理。

有时候你比编译器知道更多信息的时候，可以确定Result就是Ok:unwrap

```rust
use std::net::IpAddr;

fn main() {
  //这里肯定不会出现错误
  let ip: IpAddr = "127.0.0.1".parse().unwrap();
}
```

有时候用户调用我们的函数，传入了无意义的参数，我们可以手动调用panic!去给用户警告

调用外部不可控的代码时，返回非法的状态无法修复，可以调用panic!

### 泛型

#### 概念

提高代码的复用能力。泛型是具体类型或其它属性的抽象代替，可以理解为是类型的一个变量。

#### 泛型函数

我们传入一个泛型，这个泛型的类型其实就是一个变量，比如我们传入一个i32类型的值进去，返回的也是i32类型的值，这里的T相当于一个类型的变量，我们可以传任意数据类型进去

```rust
fn test<T>(item: T) -> T {
    item
}
```

#### 结构体泛型

```rust
struct Point<T> {
  x: T,
  y: T,
}

//泛型可以传多个不同的类型，这里的字母一般是由自己定义的
struct Area<T, U, P> {
  x: T,
  y: U,
  z: P,
}

fn main() {
  let point = Point { x: 1, y: 1 };
  let area = Area {
    x: 1,
    y: 1.00,
    z: "123",
  };
}
```

#### enum中的泛型

可以让枚举的变体拥有泛型数据类型，下面的两个例子就是我们之前提到过的

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E), 
}
```

#### 方法中使用泛型

```rust
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn getX(&self) -> &T {
    &self.x
  }
}

fn main() {
  let p = Point { x: 1, y: 2 };
  // x is: 1
  println!("x is: {}", p.getX());
}
```

- 把T放在impl关键字之后，表示在类型T上实现方法
- 只针对具体类型实现方法：`impl Point<i32>`

- struct里的泛型参数类型可以和定义的方法的泛型类型参数不同

### Trait(特性)

这个也是Rust中最重要的特性之一。

- Trait可以告诉编译器某种类型具有哪些并且可以和其它类型共享的功能
- 抽象的定义共享行为
- Trait bounds(约束):泛型类型参数指定为实现了特定行为的类型
- **Trait其实和其它语言的接口interface很像，但又有区别**

#### 定义trait

把方法签名放一起，来定义实现某种目的所必需的一组行为

- 只有方法签名，没有具体实现
- 可以有多个方法，每个方法占一行，以分号;结尾
- 实现该Trait的类型必须提供具体的方法实现

```rust
pub trait Test {
  fn test1(&self) -> String;
  fn test2(&self) -> String;
}

fn main() {}
```

#### 在类型上实现trait

与为类型实现方法类似，不同的地方就是`impl XXX for XXX {}`, {}里要对方法的定义进行具体的实现

```rust
//src/lib.rs

//人或动物的描述信息
pub trait Description {
  fn description(&self) -> String;
}

pub struct People {
  pub name: String,
  pub gender: String,
  pub age: i32,
  pub height: i32,
}

impl Description for People {
  fn description(&self) -> String {
    format!(
      "Student description: name:{}-age:{}-gender:{}-height:{}",
      self.name, self.age, self.gender, self.height
    )
  }
}

pub struct Animal {
  pub name: String,
  pub action: String,
}

impl Description for Animal {
  fn description(&self) -> String {
    format!(
      "Animal description: name:{}-action: {}",
      self.name, self.action
    )
  }
}
```

```rust
// src/main.rs
//这里的demo是Cargo.toml文件里的[package]的name值
//把trait和struct引入
use demo::Description;
use demo::People;

fn main() {
  let student = People {
    name: String::from("大锤"),
    age: 18,
    height: 200,
    gender: String::from("男"),
  };

  //People: Student description: name:大锤-age:18-gender:男-height:200
  println!("People: {}", student.description())
}

```

#### 实现trait的约束

可以在某个类型上实现Trait的前提是，这个类型或者这个Trait是在本地crate里定义的 。

不能为外部类型来实现外部的Trait：

- 这个限制是程序属性的一部分
- 更具体说是**孤儿规则**，之所以这样命名是因为父类型不存在
- 这个规则能确保别人不能破坏你的代码，反之亦然，你也不能破坏别人的代码
- 如果没有这个规则，两个crate可以为同一个类型实现同一个Trait，Rust就不知道用哪个实现了
- 也就是说不能为第三方库的类型来实现第三方库的Trait

#### 默认行为

- 就是在Trait中的方法可以给默认的实现方式，然后impl trait for struct的时候不需要手动去实现Trait的方法

  ```rust
  pub trait Description {
    fn description(&self) -> String {
      String::from("默认实现")
    }
  }
  
  pub struct People {
    pub name: String,
    pub gender: String,
    pub age: i32,
    pub height: i32,
  }
  
  impl Description for People {}
  ```

- 默认实现的方法也可以调用Trait里的其它方法，即使调用的方法没有默认的实现

  ```rust
  pub trait Description {
    fn description(&self) -> String;
    fn default_fn(&self) -> String {
      format!(
        "description and default function test: {}",
        self.description()
      )
    }
  }
  
  pub struct People {
    pub name: String,
    pub gender: String,
    pub age: i32,
    pub height: i32,
  }
  
  impl Description for People {
    fn description(&self) -> String {
      //People: 没有进行默认实现的方法: 大锤, 18, 男, 200
      format!(
        "没有进行默认实现的方法: {}, {}, {}, {}",
        self.name, self.age, self.gender, self.height
      )
    }
  }
  ```

#### 把trait作为函数参数

- 使用impl Trait语法：适用于简单的情况

  ```rust
  pub trait Description {
    fn description(&self) -> String;
    fn default_fn(&self) -> String {
      format!(
        "description and default function test: {}",
        self.description()
      )
    }
  }
  
  pub struct People {
    pub name: String,
    pub gender: String,
    pub age: i32,
    pub height: i32,
  }
  
  impl Description for People {
    fn description(&self) -> String {
      //People: 没有进行默认实现的方法: 大锤, 18, 男, 200
      format!(
        "没有进行默认实现的方法: {}, {}, {}, {}",
        self.name, self.age, self.gender, self.height
      )
    }
  }
  
  pub fn test_trait_fn(item: impl Description) {
    println!("{}", item.description())
  }
  ```

- 使用Trait bound语法：适用于复杂的情况，实际就是泛型约束，上面那种方式其实就是这种方式的语法糖

  ```rust
  //上面的那个方法改写成这样
  pub fn test_trait_fn<T: Description>(item: T) {
    println!("{}", item.description())
  }
  ```

- 如果实现了多个Trait，可以使用+号连接

  ```rust
  use std::fmt::Display;
  
  ......
  
  pub fn test_trait_fn1(item: impl Description + Display) {
    println!("{}", item.description())
  }
  
  pub fn test_trait_fn2<T: Description + Display>(item: T) {
    println!("{}", item.description())
  }
  ```

- 使用where子句来进行泛型约束，上下两种实现方式是等价的

  ```rust
  use std::clone::Clone;
  use std::fmt::{Debug, Display};
  
  //test_trait_fn1方法的T类型实现了Description和Display两个Trait，U类型实现了Debug和Clone两个Trait
  pub fn test_trait_fn1<T: Description + Display, U: Debug + Clone>(item1: T, item2: U) {
    println!("{}", item1.description())
  }
  
  pub fn test_trait_fn2<T, U>(item1: T, item2: U)
  where
    T: Description + Display,
    U: Debug + Clone,
  {
    println!("{}", item1.description())
  }
  ```

#### 实现了的Trait作为返回类型

impl Trait只能返回确定的同一种类型，返回可能出现的不同类型代码会报错，比如函数中判断正确时返回People，错误时返回Animal就会报错。

```rust
//只能返回实现了Description这个Trait
pub fn test() -> impl Description {
  People {
    name: String::from("王大锤"),
    gender: String::from("男"),
    age: 18,
    height: 200,
  }
}
```

#### 看个例子，获取数组里的最大的一项

T是实现了PartialOrd和Clone两个Trait的类型

```rust
fn largest<T: PartialOrd + Clone>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list.iter() {
    if item > largest {
      largest = item
    }
  }

  largest
}

fn main() {
  let str = vec![String::from("hello"), String::from("world")];
  let res = largest(&str);

  println!("Largest is {}", res)
}
```

#### 使用Trait Bound有条件的实现方法

- 在使用泛型类型参数的impl块上使用Trait bound，我们可以有条件的为实现了特定Trait的类型来实现方法。下面这段代码的意思就是对于Pair来说，不论T是什么类型的时候都会有new这个方法，而只有T的类型是PartialOrd和Display的时候，它才会有cmp_display这个方法。

  ```rust
  use std::fmt::Display;
  
  struct Pair<T> {
    x: T,
    y: T,
  }
  
  impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
      Self { x, y }
    }
  }
  
  impl<T: PartialOrd + Display> Pair<T> {
    fn cmp_display(&self) {
      if (self.x > self.y) {
        println!("x > y")
      } else {
        println!("x < y")
      }
    }
  }
  
  fn main() {}
  ```

- 也可以为实现了其它Trait的任意类型有条件的实现某个Trait

- 为满足Trait Bound的所有类型上实现Trait叫做覆盖实现。

### 生命周期

#### 概念

Rust的又一个重要特性之一。Rust里每个引用都有自己的生命周期，生命周期就是让引用保持有效的作用域，大多数时候生命周期是隐式的、可以被推断出来的。当引用的生命周期可能以不同的方式互相关联时，就需要手动标注生命周期。**生命周期的主要目标就是避免悬垂引用**。

下面的代码报错借用的值没有足够长的生命周期，这是因为代码执行完里面的花括号作用域后，x已经被释放了，x的引用不能赋值给r。这里面其实用到了借用检查器：用来比较作用域来判断所有的借用是否合法

```rust
fn main() {
  let r;
  {
    let x = 32;
    //报错：borrowed value does not live long enough
    r = &x;
  }

  println!("{}", r);
}
```

下面再来看个生命周期的例子

```rust
fn main() {
  {
    // 'a生命周期，'a生命周期覆盖了'b生命周期。'b的生命周期比较短，所以不能把值借用给'a生命周期上的变量
    let r;
    {
      // `b生命周期
      let x = 32;
      //报错：borrowed value does not live long enough
      r = &x;
    }

    println!("{}", r);
  }
}
```

```rust
fn main() {
  //像这样是没问题的，因为x的生命周期包含了r的生命周期
  let x = 32;
  let r = &x;

  println!("{}", r);
}
```

#### 生命周期泛型例子

看个生命周期的相关例子，产生错误的原因是编译器不知道传入参数str1或者str2的生命周期，借用检查器也不知道返回类型的生命周期是跟哪个有关系。这里看不懂就先暂时看着，后面会解释

```rust
fn main() {
  let s1 = String::from("hello world");
  let s2 = "I am Wang Dachui";

  let longest = get_longest(&s1.as_str(), &s2);

  println!("{}", longest)
}

// 报错：返回类型包括一个借用的值，但是签名不知道它是从str1借用还是str2借用的，可以加上生命周期参数
// help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `str1` or `str2`
// help: consider introducing a named lifetime parameter
//类似于↓这样
// fn get_longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
fn get_longest(str1: &str, str2: &str) -> &str {
  if (str1.len() > str2.len()) {
    str1
  } else {
    str2
  }
}
```

```rust
fn main() {
  let s1 = String::from("hello world");
  let s2 = "I am Wang Dachui";

  let longest = get_longest(&s1.as_str(), &s2);

  println!("{}", longest)
}

//标注生命周期泛型，然后像这样写程序就能正常运行了。
//这里的意思是：get_longest函数有一个'a的生命周期标注，它有两个参数str1和str2都是字符串切片类型引用，
//这两个参数的存活时间必须不能短于'a，而对于返回值，也不能短于'a这个生命周期
//其实就是为了告诉编译器，生命周期不一样时不能给你编译通过
//这里的'a其实能获得的生命周期就是str1和str2中比较短的那一个，取交集
fn get_longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
  if (str1.len() > str2.len()) {
    str1
  } else {
    str2
  }
}
```

#### 生命周期标注语法

- 生命周期的标注不会改变引用的生命周期长度
- 当指定了泛型生命周期参数，函数可以接收带有任何生命周期的引用
- 生命周期的标注：描述了多个引用的生命周期之间的关系，但不影响生命周期

##### 语法

生命周期参数名：以'开头，通常以全小写开头且很短，很多人喜欢用'a做生命周期标注的名称

生命周期标注位置：在引用的&符号后面，使用空格将标注和引用类型分开

例子：

```rust
&i32 //一个引用
&'a i32 //带有显式生命周期的引用
&'a mut i32 //带有显式生命周期的可变引用	
```

单个生命周期的标注没有什么意义

##### 函数签名中的生命周期标注

泛型生命周期参数声明在：函数名和参数列表之间的<>里

生命周期'a的实际生命周期就是str1和str2中比较小的那个。

```rust
fn main() {
  let s1 = String::from("hello world");
  let longest;
  {
    let s2 = String::from("I am Wang Dachui");
    //这里s2发生报错： borrowed value does not live long enough
    //借用的值没有总够长的生命周期
    longest = get_longest(&s1.as_str(), &s2.as_str());
  }

  //s2在离开作用域后已经被drop函数回收，所以不能够被借用
  println!("{}", longest)
}

fn get_longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
  if (str1.len() > str2.len()) {
    str1
  } else {
    str2
  }
}
```

#### 深入理解生命周期

- 指定生命周期参数的方式依赖于函数所做的事情

  ```rust
  //这个函数返回了，所以生命周期只和str1有关，跟str2无关，就可以能把str2的生命周期标注给去了
  fn get_longest<'a>(str1: &'a str, str2: &str) -> &'a str {
    str1
  }
  ```

- 从函数返回引用时，返回类型的生命周期参数需要与其中一个参数的生命周期匹配

- 如果返回的引用没有指向任何参数，那么它只能引用函数内创建的值：这就是垂悬引用，这个值在函数结束的时候就离开了作用域 

  ```rust
  fn test<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    let res = String::from("hello");
    //报错：returns a value referencing data owned by the current function
      //返回了一个当前函数拥有的相关数据
    res.as_str()
  }
  
  //如果想返回内部的值给外部使用，那么就不返回引用
  fn test<'a>(str1: &'a str, str2: &'a str) -> String {
    let res = String::from("hello");
    res
  }
  ```

#### Struct定义中的生命周期标注

Struct内可以包括：

- 基本数据类型
- 引用：需要在每个引用上添加生命周期标注

```rust
struct ImportantExcerpt<'a> {
  //part引用存活的时间必须要比实例的存活时间要长
  //只要实例存在就会一直对part有引用
  part: &'a str,
}

fn main() {
  let novel = String::from("很久很久以前，有一个...");

  let first_sentence = novel.split("，").next().expect("找不到，");

  let i = ImportantExcerpt {
    //这里part生命周期其实就是从first_sentence创建到离开main作用域
    //它的生命周期比i的生命周期要长
    part: first_sentence,
  };
}
```

##### 生命周期的省略概念

在Rust引用分析中所编入的模式称为生命周期的省略规则，也就是一些可以预测的生命周期已经被写到编译器中，我们就不需要手动去写生命周期标注了。

##### 输入、输出生命周期的概念

生命周期在函数/方法的参数中，那么就叫做输入生命周期。

输入出现在函数/方法的返回值中，就叫做输出生命周期。

##### 生命周期省略的三个规则

编译器使用3个规则在没有显式标注生命周期的情况下，来确定引用的生命周期，适用于fn定义和impl块。如果编译器应用完这三个规则之后，仍然无法确定生命周期的引用就会报错。规则1应用于输入生命周期，规则2、3应用于输出生命周期。

- 规则1：每个引用类型的参数都有自己的生命周期
- 规则2：如果只有1个输入生命周期参数，那么该生命周期被赋给所有的输出生命周期参数
- 规则3：如果有多个输入生命周期参数，但其中一个是&self或&mut self(方法才有self)，那么self的生命周期会被赋给所有的输出生命周期参数

例子1：

假如我们是编译器，那么应用这三条规则后会发生什么

```rust
//我们写的代码↓
fn example(s: &str) -> &str {}
//应用第一条规则后
fn example<'a>(s: &'a str) -> &str {}
//应用第二条规则后
fn example<'a>(s: &'a str) -> &'a str {}
```

例子2：

```rust
//我们写的代码↓
fn example2(x: &str, y: &str) -> &str {}
//应用第一条规则后
fn example2<'a, 'b>(x: &'a str, y: &'b str) -> &str {}
//第二条规则不适用，因为函数有两个参数
//第三条规则也不适用，因为这不是一个方法
//应用完三条规则后，编译器也不能确定生命周期，然后就会报错
```

##### 方法定义中的生命周期标注

第三条规则只适用于方法

在哪声明和使用生命周期，依赖于：生命周期参数是否和字段、方法的参数或返回值有关

struct字段的生命周期名：

- 在impl后声明
- 在struct名后使用
- 这些生命周期是struct类型的一部分

impl块内的方法签名中：

- 引用必须绑定于struct字段引用的生命周期，或者引用是独立的也可以
- **生命周期省略规则经常使得方法中的生命周期标注不是必须的**

下面的代码编译没问题，根据第三条规则，self的生命周期会被赋给所有的输出生命周期参数

```rust
struct ImportantExcerpt<'a> {
  part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
  fn example(&self) -> i32 {
    3
  }

  fn example2(&self, str1: &str, str2: &str) -> &str {
    self.part
  }
}

fn main() {}
```

##### 静态生命周期'static

’static是一个特殊的生命周期：表示整个程序的持续时间，比如所有的字符串字面值都拥有'static生命周期。

比如：

```rust
//它存储在二进制文件中，所以它总是可用的
let s: &'static str = "I have a dream";
```

为引用指定'static生命周期前要思考：是否需要引用在程序整个生命周期中存活？

#### 泛型参数类型、Trait Bound、生命周期综合例子

```rust
use std::fmt::Display;

fn longest_mark<'a, T>(x: &'a str, y: &'a str, mark: T) -> &'a str
where
  T: Display,
{
  println!("mark is {}", mark);
  if (x.len() > y.len()) {
    x
  } else {
    y
  }
}

fn main() {}
```

### 编写测试

#### 测试(就是一个函数)

验证非测试代码的功能是否和预期一致，测试体通常执行3个操作：

- 准备数据/状态
- 运行被测试代码
- 断言(assert)结果

#### 测试函数

测试函数需要使用test属性(Attribute)进行标注，Attribute就是一段Rust代码元数据

- 在函数上加`#[test]`就可以把函数变成测试函数
- Rust也允许测试私有函数，外部的私有函数可以在测试模块里进行调用测试。

#### 运行测试

使用cargo test命令运行所有测试函数，Rust会构建一个Test runner可执行文件，它会运行标注了test的函数，并报告其是否成功。

当使用cargo创建library项目的时候，会生成一个test module，里面有一个test函数，你可以添加任意数量的test module或函数

创建库项目使用`cargo new demo --lib`，然后我们可以看到lib.rs文件中

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

#### 测试失败时

- 测试函数panic就表示失败

- 每个测试运行在一个新的线程
- 当主线程看见某个测试线程挂了，那么测试标记为失败

#### 断言

assert!宏，来自标准库，用来确定某个状态是否为true

- true：测试通过
- panic：测试失败

assert_eq!(equal)断言相等和assert_ne!(not equal)断言不相等这两个宏用来测试相等性，断言失败时会使用debug格式自动打印出两个参数的值，所以就要求参数实现了PartialEq和Debug Traits（所有基本类型和标准库里大部分的类型都实现了这两个Trait）

#### 添加自定义错误信息

可以向assert!、assert_eq!、assert_ne!添加可选的自定义消息，这些自定义消息和失败消息都会打印出来。

- assert!：第一个参数必填，自定义信息作为第二个参数
- assert_eq!、assert_ne!：第一二个参数必填，第三个参数作为自定义信息
- 自定义信息参数会传给format!宏，可以使用{}占位符

#### 检查Panic

使用should_panic标注检查是否发生了恐慌，可以验证代码在特定情况是否发生了panic，函数发生panic测试会通过，下面的例子就会测试通过。

```rust
#[cfg(test)]
mod tests {
    #[test]
    #[should_panic]
    fn it_works() {
        panic!("Panic")
    }
}
```

让should_panic更加精确，使用expected参数，用来指定失败的消息中是否包含指定的文字。下面的例子就会检查通过，如果把num改成50以上，就会检查失败

```rust
#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "发生panic时值小于50")]
    fn it_works() {
        let num = 10;
        if num < 50 {
            panic!("发生panic时值小于50");
        } else {
            panic!("发生panic时值大于50");
        }
    }
}
```

#### 使用Result<T,E>测试

不需要使用should_panic标注

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 1 + 1 == 2 {
            Ok(())
        } else {
            Err(String::from("失败"))
        }
    }
}
```

#### 控制测试怎么运行

- 改变cargo test的行为，可以添加命令行参数
- **默认行为是并行使用多线程执行所有测试**，不显示所有输出，使得读取和测试结果相关的输出更容易
- 命令行参数：比如cargo test --help可以查看帮助，使用cargo test -- --help可以查看所有和--命令有关的帮助信息

并行运行测试：默认使用多线程，优点是运行快，但是要确保测试函数之间不会相互依赖，并且不依赖于某个共享状态(环境、环境变量、工作目录等)。

--test-threads参数：传递给二进制文件，不想以并行方式运行测试，或者想对线程进行更细粒的控制，后边紧跟着线程的数量

`cargo test --test-threads=1`

默认情况下，测试代码中使用到了println!，只会在测试失败时打印出来，测试成功的不会打印。如果想要测试成功的时候也打印，就使用 --show-output参数

#### 按名称运行测试子集

选择要运行的测试，将测试的名称（一个或者多个）作为cargo test的参数

运行测试单个：指定测试名`cargo test other`

运行多个测试：指定测试名的一部分（模块名也可以）`cargo test test_`会匹配带有test_的测试函数名

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_eq() {
        assert_eq!(4, 4)
    }

    #[test]
    fn test_nq() {
        assert_ne!(3, 4)
    }

    #[test]
    fn other() {
        assert!(true)
    }
}
```

#### 忽略某些测试用例

一般是对于耗时的测试用例，我们可以忽略掉，运行其它的测试。

可以使用**ignore(attribute)**属性标注`#[ignore]`，而想要运行被忽略的测试，可以使用`cargo test -- --ignored`

#### 测试分类

测试分类为两类：一类是单元测试、一类是集成测试

- 单元测试：小、专注，一次只对一个模块进行隔离的测试，可以测试private接口
- 集成测试：在库的外部，和其它代码外部代码一样使用你的代码，只能使用public接口，可能在每个测试中使用到多个模块

##### 单元测试

使用`#[cfg(test)]`标注，在test模块上标注的`#[cfg(test)]`，只有运行cargo test才会编译和运行代码，运行cargo build的时候不会。cfg指的是configuration(配置)

##### 集成测试

它不需要#[cfg(test)]标注，集成测试完全位于被测试库的外部。目的是测试被测试库的多个部分是否能正确的一起工作，集成测试覆盖率是很重要的指标。

创建和src目录并列的tests目录，tests目录下每个测试文件都是一个单独的crate，**cargo在编译时会自动寻找tests目录**，将里面每一个文件都单独处理为一个单独的包。集成测试需要将被测试的库导入

```rust
// src/lib.rs
pub fn add(num1: i32, num2: i32) -> i32 {
    num1 + num2
}
```

```rust
// tests/add_test.rs
use demo;

#[test]
fn it_add_test() {
  assert_eq!(4, demo::add(2, 2));
}

//然后可以执行cargo test进行测试
```

运行一个指定的集成测试：`cargo test 函数名 `

运行某个测试文件内的所有测试：`cargo test --test 文件名`

集成测试的子模块：

tests目录下每个文件都被单独编译成crate，这些文件不共享行为。如果想要在集成测试文件内使用一些工具类辅助函数，那么我们就可以把文件抽离tests的根目录，比如在tests目录下新建一个common目录，里面编写工具函数，然后在对应的测试文件中引入这个模块使用。cargo编译测试的时候就不会去编译common目录里的文件。

##### 针对binary crate的集成测试

如果项目是二进制包，只有src/main.rs而没有src/lib.rs，这时候就不可以在tests目录下创建集成测试

- 不能在tests目录下创建集成测试
- 无法把main.rs的函数导入作用域
- 只有library crate才能暴露函数给其它的crate使用
- binary crate意味着需要独立运行



### 编写命令行工具

#### 接收命令行参数

```rust
use std::env; //环境变量相关的模块

fn main() {
  //args方法返回一个迭代器，collect方法会产生一个集合。
  //比如我们输入cargo run 123 abcd，这里就打印["target\\debug\\demo.exe", "123", "abcd"]
  let args: Vec<String> = env::args().collect();

  println!("{:?}", args)
}
```

我们想要实现的效果就是`cargo run abcd text.txt`，在text.txt文件中查找abcd字符串

```rust
use std::env; //环境变量相关的模块

fn main() {
  //args方法返回一个迭代器，collect方法会产生一个集合。
  //比如我们输入cargo run 123 abcd，这里就打印["target\\debug\\demo.exe", "123", "abcd"]
  let args: Vec<String> = env::args().collect();

  let query = &args[1];
  let file_name = &args[2];

  //在text.txt文件中寻找abcd字符串
  println!("在{}文件中寻找{}字符串", file_name, query);
}
```

#### 读取文件

我们在和src同级的根目录下创建`text.txt文件`，里面随便输入一些内容

```json
我是一只小小小小鸟
想要飞却怎么也飞不高
~~~~~~
```

```rust
use std::env; //环境变量相关的模块
use std::fs; //文件系统相关模块

fn main() {
  let args: Vec<String> = env::args().collect();

  let query = &args[1];
  let file_name = &args[2];

  println!("在{}文件中寻找{}字符串", file_name, query);

  let contents = fs::read_to_string(file_name).expect("读取文件出现了一些错误");

  // 在text.txt文件中寻找小小小小鸟字符串
  // 读取到的内容是：
  // 我是一只小小小小鸟
  // 想要飞却怎么也飞不高
  // ~~~~~~
  println!("读取到的内容是：\n{}", contents);
}
```

#### 模块化抽取

二进制程序关注点分离的原则：

- 将程序拆分为main.rs和lib.rs，将业务逻辑放入到lib.rs
- 当命令行解析逻辑较少时，将它放到main.rs也行
- 当命令行解析逻辑变复杂时，需要将它从main.rs提取到lib.rs

经过拆分抽取后，我们希望main.rs保留的功能：

- 使用参数值调用命令行解析逻辑
- 进行其它配置
- 调用lib.rs中的run函数
- 处理run函数中可能会出现的错误

```rust
use std::env; //环境变量相关的模块
use std::fs; //文件系统相关模块

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args);

  let contents = fs::read_to_string(config.file_name).expect("读取文件出现了一些错误");

  println!("读取到的内容是：\n{}", contents);
}

struct Config {
  query: String,
  file_name: String,
}

impl Config {
  fn new(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_name = args[2].clone();
    Config { query, file_name }
  }
}
```

#### 错误处理优化

我们想要在程序出错的时候，将错误信息抛出去给用户，但又不想要出现太多关于Rust本身给我们抛出的错误信息，所以就需要去自定义最终的错误信息输入

```rust
use std::env; //环境变量相关的模块
use std::fs; //文件系统相关模块
use std::process; //进程相关模块

fn main() {
  let args: Vec<String> = env::args().collect();

  //unwrap_or_else方法，如果在前面的方法调用成功就会返回结果
  //如果前面的方法调用失败就会返回一个闭包(一个匿名函数)，使用|err|表示
  let config = Config::new(&args).unwrap_or_else(|err| {
    //程序出错时只会打印这一句话作为输出
    println!("解析参数时发生了错误");
    //调用process::exit退出进程方法，程序的执行会立即终止，参数1作为程序退出的状态码
    process::exit(1);
  });

  let contents = fs::read_to_string(config.file_name).expect("读取文件出现了一些错误");

  println!("读取到的内容是：\n{}", contents);
}

struct Config {
  query: String,
  file_name: String,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("传入的参数值数量不够");
    }
    let query = args[1].clone();
    let file_name = args[2].clone();
    Ok(Config { query, file_name })
  }
}
```

#### 抽取业务逻辑

在src目录下新建lib.rs文件，然后将一些逻辑代码抽取过去。另外我们需要将业务逻辑先单独抽取到一个run函数中。

```rust
use std::env; //环境变量相关的模块
use std::error::Error;
use std::fs; //文件系统相关模块
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();

  //unwrap_or_else方法，如果在前面的方法调用成功就会返回结果
  //如果前面的方法调用失败就会返回一个闭包(一个匿名函数)，使用|err|表示
  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("解析参数时发生了错误");
    //调用process::exit方法，程序的执行会立即终止，参数1作为程序退出的状态码
    process::exit(1);
  });

  //这里的Err是run的返回类型Result的一个变体
  if let Err(e) = run(config) {
    println!("程序发生错误：{}", e);
    process::exit(1);
  };
}

// Box<dyn Error>,表示的是一个返回了实现Error这个trait的类型。后面细讲。
fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_name)?;

  println!("读取到的内容是：\n{}", contents);

  Ok(())
}

struct Config {
  query: String,
  file_name: String,
}

impl Config {
  fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("传入的参数值数量不够");
    }
    let query = args[1].clone();
    let file_name = args[2].clone();
    Ok(Config { query, file_name })
  }
}
```

将run函数的业务逻辑抽取之后，我们就要对代码进行拆分到不同文件中了。

```rust
// src/lib.rs

use std::error::Error;
use std::fs; //文件系统相关模块

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_name)?;

  println!("读取到的内容是：\n{}", contents);

  Ok(())
}

pub struct Config {
  pub query: String,
  pub file_name: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("传入的参数值数量不够");
    }
    let query = args[1].clone();
    let file_name = args[2].clone();
    Ok(Config { query, file_name })
  }
}
```

```rust
// src/main.rs

use demo::Config; //引入Config块
use std::env; //环境变量相关的模块
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    println!("解析参数时发生了错误");
    process::exit(1);
  });

  //demo是项目名称
  if let Err(e) = demo::run(config) {
    println!("程序发生错误：{}", e);
    process::exit(1);
  };
}
```

#### 测试驱动开发(TDD)

这种方法就是我们先编写测试用例，然后再去编辑实际逻辑，也就是用测试去驱动我们的开发。

- 编写一个会失败的测试，运行该测试，确保它是按照预期的原因失败
- 编写或修改刚好足够的代码，让新测试通过
- 重构刚刚添加或修改的代码，确保测试会始终通过

- 返回步骤1，继续

我们先编写一个测试用例，然后再去进行相关的代码开发

```rust
// src/lib.rs

#[cfg(test)]
mod tests {
  //引入模块外部的内容
  use super::*;

  #[test]
  fn one_result() {
    let query = "小小鸟";
    //注意前面不要留空格
    let content = "\
我是一只小小小小鸟
想要飞却怎么也飞不高
~~~~~~";

    //这里可以测试通过
    assert_eq!(vec!["我是一只小小小小鸟"], search(query, content));
  }
}
```

看下我们最终编写的代码：

```rust
// src/lib.rs
use std::error::Error;
use std::fs; //文件系统相关模块

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_name)?;
  //修改
  for line in search(&config.query, &contents) {
    println!("{}", line);
  }

  Ok(())
}

pub struct Config {
  pub query: String,
  pub file_name: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("传入的参数值数量不够");
    }
    let query = args[1].clone();
    let file_name = args[2].clone();
    Ok(Config { query, file_name })
  }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "小小鸟";
    //注意前面不要留空格
    let content = "\
我是一只小小小小鸟
想要飞却怎么也飞不高
~~~~~~";

    //这里可以测试通过
    assert_eq!(vec!["我是一只小小小小鸟"], search(query, content));
  }
}
```

这里编写的测试用例通过后，我们就可以运行我们的测试命令。

比如我们在根目录下的text.txt文件中放入了这些字符串

```json
我是一只小小小小鸟
想要飞却怎么也飞不高
~~~~~~
```

然后我们想要搜索某个字符串就去运行

```rust
cargo run 小小鸟 text.txt
```

最后我们的代码给我们打印出来的就是`我是一只小小小小鸟`，找到了包含了某个字符串的某一行内容。

#### 使用环境变量

比如我们想要对英文进行搜索的时候，希望可以区分大小写或者不区分，就需要使用上环境变量了

```rust
use std::env;
use std::error::Error;
use std::fs; //文件系统相关模块

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.file_name)?;
  println!("{}", !config.case_sensitive);
  let results = if !config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };
  for line in results {
    println!("{}", line);
  }

  Ok(())
}

pub struct Config {
  pub query: String,
  pub file_name: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("传入的参数值数量不够");
    }
    let query = args[1].clone();
    let file_name = args[2].clone();
    //使用CASE_INSENSITIVE环境变量，意思就是不区分大小写
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Ok(Config {
      query,
      file_name,
      case_sensitive,
    })
  }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      result.push(line);
    }
  }

  result
}

//为了不区分大小写，我们把搜索和要搜索的内容全部转换成小写就可以了
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut result = Vec::new();
  let query = query.to_lowercase();
  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      result.push(line);
    }
  }

  result
}

#[cfg(test)]
mod tests {
  use super::*;

  //区分大小写
  #[test]
  fn case_sensitive() {
    let query = "fly";
    //注意前面不要留空格
    let content = "\
I am a little bird.
Thought I want to fly.
I can't fly hight.
Fly into the sky.
~~~~~~";

    //这里可以测试通过
    assert_eq!(
      vec!["Thought I want to fly.", "I can't fly hight."],
      search(query, content)
    );
  }

  //不区分大小写
  #[test]
  fn case_insensitive() {
    let query = "FLY";
    //注意前面不要留空格
    let content = "\
I am a little bird.
Thought I want to fly.
I can't fly hight.
Fly into the sky.
~~~~~~";

    //这里可以测试通过
    assert_eq!(
      vec![
        "Thought I want to fly.",
        "I can't fly hight.",
        "Fly into the sky."
      ],
      search_case_insensitive(query, content)
    );
  }
}
```

然后在命令行中输入，就可以看到是大小写不敏感的

```
windows：
set CASE_INSENSITIVE=1
cargo run T text.txt

Mac
CASE_INSENSITIVE=1
cargo run T text.txt
```

#### 将错误信息写入到标准错误而不是标准输出

- 标准输出：stdout- println!
- 标准错误：stderr- eprintln!

我们把main.rs里的打印换成eprintln!

```rust
// src/main.rs
use demo::Config; //引入Config块
use std::env; //环境变量相关的模块
use std::process;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    eprintln!("解析参数时发生了错误");
    process::exit(1);
  });

  //demo是项目名称
  if let Err(e) = demo::run(config) {
    eprintln!("程序发生错误：{}", e);
    process::exit(1);
  };
}
```

然后我们执行命令，这个命令的意思就是让我们把最终的标准输出写入到output.txt文件中。而我们上面替换的epintln!的内容不会写入到文件中。

```rust
cargo run > output.txt
```

### 闭包(closure)

#### 概念

函数式语言的特性：迭代器和闭包。

闭包是啥？就是可以捕获其所在环境的匿名函数

特点：

- 是匿名函数
- 可以保存赋值给变量、作为参数
- 可以在一个地方创建闭包、然后在另一个上下文中调用闭包来完成运算
- 可以从定义的作用域中捕获值

```rust
fn main() {
  let test_closure = |num| {
    println!("这里是在闭包匿名函数内部，传入的值是:{}", num);
    num + 1
  };

  //在外面调用闭包匿名函数，得到的结果是:2
  println!("在外面调用闭包匿名函数，得到的结果是:{}", test_closure(1))
}
```

#### 闭包的类型推断

- 闭包不强制要求标注参数和返回值的类型   

- 闭包通常很短小，只在狭小的上下文空间内工作，编译器通常能判断出类型
- 也可以手动的添加类型

函数和闭包的定义语法

```rust
fn add_one(x: u32) -> u32 { x + 1 }

//闭包
let add_one_closure = |x: u32| -> u32 { x + 1 }; 
//或者省去参数
let add_one_closure2 = |x|  x + 1;
```

闭包的定义最终只会为参数/返回值推断出唯一具体的类型，后面使用不能再修改类型。

```rust
fn main() {
  let test_closure = |x| x;

  let s = test_closure(String::from("闭包测试"));
  //这里会报错，因为闭包已经绑定了String类型，不能再修改了
  let n = test_closure(1);
}
```

#### 使用闭包进行记忆化/延迟计算

创建一个struct，它持有闭包及其调用的结果。

- 只会在需要结果时才会去执行该闭包
- 可以缓存结果

#### 如何让结构体持有闭包

struct的定义需要知道所有字段的类型，就需要指明里面的闭包的类型。每个闭包实例都拥有自己唯一的匿名类型，即使两个闭包签名完全一样，这两个实例也是两个类型。所以要在结构体使用闭包，**就需要用到泛型和Trait Bound**。

#### Fn trait

Fn traits由标准库提供，所有的闭包都实现了以下trait之一：

- Fn
- FnMut
- FnOnce

```rust
//这就是一个闭包结构体。
//一开始value的值是空的，一旦执行过一次calculation这个闭包
//value就会把得到的值给缓存起来，以后再次执行时直接返回这个值
struct Cacher<T>
where
  T: Fn(u32) -> u32,
{
  calculation: T,
  value: Option<u32>,
}

impl<T> Cacher<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    //判断self.value是否有值，如果有就直接返回这个值，
    //如果没有值，就说明闭包没执行过，那么就执行这个闭包，并传参数进去，
    //执行的结果封装到Some里赋值给self.value，并把执行结果返回
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

fn main() {
  let mut add_one = Cacher::new(|num| num + 1);

  //第一次执行闭包后将值缓存了起来，以后都会返回缓存的值
  // 调用add_one的值是：2
  // 调用add_one的值是：2
  // 调用add_one的值是：2
  println!("调用add_one的值是：{}", add_one.value(1));
  println!("调用add_one的值是：{}", add_one.value(2));
  println!("调用add_one的值是：{}", add_one.value(3));
}
```

而针对闭包每次执行都返回一样的值，可以使用hashMap来将结果给保存，然后arg作为key，闭包执行的结果作为value。

针对这个闭包每次只能接收一个u32类型和返回一个u32类型，我们可以把它改造成泛型。

#### 使用闭包捕获环境

闭包可以访问定义它的作用域内的变量，而普通函数则不能。

```rust
//这就是一个闭包结构体。
//一开始value的值是空的，一旦执行过一次calculation这个闭包
//value就会把得到的值给缓存起来，以后再次执行时直接返回这个值
struct Cacher<T>
where
  T: Fn(u32) -> u32,
{
  calculation: T,
  value: Option<u32>,
}

impl<T> Cacher<T>
where
  T: Fn(u32) -> u32,
{
  fn new(calculation: T) -> Cacher<T> {
    Cacher {
      calculation,
      value: None,
    }
  }

  fn value(&mut self, arg: u32) -> u32 {
    //判断self.value是否有值，如果有就直接返回这个值，
    //如果没有值，就说明闭包没执行过，那么就执行这个闭包，并传参数进去，
    //执行的结果封装到Some里赋值给self.value，并把执行结果返回
    match self.value {
      Some(v) => v,
      None => {
        let v = (self.calculation)(arg);
        self.value = Some(v);
        v
      }
    }
  }
}

fn main() {
  let x = 1;

  fn is_equal(y: i32) -> bool {
    //报错：can't capture dynamic environment in a fn item
    //不能在函数内捕获动态环境
    y == x
  }

  //执行正常，里面可以获取到外面的变量，但是会产生额外的内存开销
  let is_equal_closure = |y: i32| -> bool { y == x };
}
```

#### 闭包从所在环境捕获值的方式

和函数获得参数的三种方式一样：

- 取得所有权：FnOnce
- 可变借用：FnMut
- 不可变借用：Fn

创建闭包时，通过闭包对环境值的使用，Rust推断出具体使用哪个Trait：

- 所有闭包都实现了FnOnce
- 没有移动捕获变量的实现了FnMut
- 无需可变访问捕获变量的闭包实现了Fn
- 所有实现了Fn的闭包都实现了FnMut，所有实现了FnMut的闭包也都实现了FnOnce

#### move关键字

在参数列表前使用move关键字，可以强制闭包取得它所使用环境值的所有权，当将闭包传递给新线程以移动数据使其归新线程所有时，这个技术最为有用。

```rust
fn main() {
  let x = vec![1, 2, 3, 4];

  let test_closure = move |y| x == y;

  let y = vec![1, 2, 3, 4];
  println!("闭包调用的值：{}", test_closure(y));

  //报错：value moved into closure here，值的所有权已经被闭包夺去了
  println!("x:{:?}", x);
} 
```

#### 最佳实践

当指定Fn trait bound之一时，首先使用Fn，基于闭包体里的情况，如果需要使用FnMut或者 FnOnce时编译器会告诉你。

### 迭代器

#### 概念

- 迭代器模式：对一系列项执行某些任务
- 迭代器负责：遍历每个项、确定遍历何时完成

Rust的迭代器是惰性的，除非调用消费迭代器的方法，否则迭代器本身没任何效果。

#### Iterator trait

所有迭代器都实现了这个Trait，Iterator trait定义于标准库。定义大概如下

```rust
pub trait Iterator {
    type Item;
    fn next(&mut slef) -> Option<Self::Item>;
    ...
}
```

type Item 和 Self::Item定义了与该Trait关联的类型。实现Iterator trait需要定义一个Item类型，它用于next方法的返回类型(迭代器的返回类型)。这个trait仅要求实现next方法：

- 该方法每次返回迭代器里的一个项
- 返回结果包裹在Som中
- 迭代结束返回None

```rust
//测试通过
#[cfg(test)]
#[test]
fn test() {
  let arr = vec![1, 2, 3, 4];
  //声明为可变的，因为next会消耗一个值
  let mut iter_test = arr.iter();

  assert_eq!(iter_test.next(), Some(&arr[0]));
  assert_eq!(iter_test.next(), Some(&arr[1]));
  assert_eq!(iter_test.next(), Some(&arr[2]));
  assert_eq!(iter_test.next(), Some(&arr[3]));
}
```

#### 几个迭代的方法

- iter方法：在不可变引用上创建迭代器
- into_iter方法：创建的迭代器会获得所有权
- iter_mut方法：迭代可变的引用

#### 消耗迭代器的方法

在标准库中，Iterator trait	有一些带默认实现的方法。

其中有一些方法会调用next方法，实现Iterator trait时必须实现next方法的原因之一。

调用next的方法叫做“消耗型适配器”，因为调用它们会消耗迭代器里的值，相当于一个个吃掉它们。

比如sum方法，就会消耗完迭代器，它会取得迭代器的所有权，通过反复调用next方法，遍历所有元素，每次迭代，把当前的元素添加到一个总和里，迭代结束后返回总和。

```rust
#[cfg(test)]
#[test]
fn test() {
  let arr = vec![1, 2, 3, 4];
  let iter_test = arr.iter();

  let sum: i32 = iter_test.sum();

  //加起来总和为10，测试通过
  assert_eq!(sum, 10);
}
```

#### 产生其它迭代器的方法

定义在Iterator trait上的另一些方法叫做“迭代器适配器”，它们可以把迭代器转化为不同种类的迭代器。

可以通过链式调用使用多个迭代器适配器来执行复杂的操作，这种调用可读性比较高。

例如map方法，可以接收一个闭包，闭包作用于每个元素，最后产生一个新的迭代器(有点像JS里面的map方法)。

```rust
#[cfg(test)]
#[test]
fn test() {
  let arr = vec![1, 2, 3, 4];

  //collect方法是消耗型适配器，能将迭代器给收集起来放到某个类型的集合中
  let arr2: Vec<_> = arr.iter().map(|x| x + 1).collect();

  //测试通过，将arr里每一项都加了1
  assert_eq!(vec![2, 3, 4, 5], arr2)
}
```

#### 闭包捕获环境

filter方法：一个适配器迭代器(也是有点像JS里的filter方法)

- 它接收一个闭包作为参数 
- 这个闭包在遍历迭代器每个元素时，返回bool类型
- 如果闭包返回true，当前元素会包含在filter产生的迭代器中
- 如果闭包返回false，当前元素就不会包含在filter产生的迭代器中

```rust
// src/lib.rs
#[derive(PartialEq, Debug)]
struct Clothes {
  size: u32,
}

//找到适合我的尺寸的衣服
fn clothes_fit_me(clothes: Vec<Clothes>, my_size: u32) -> Vec<Clothes> {
  clothes.into_iter().filter(|x| x.size == my_size).collect()
}

#[cfg(test)]
#[test]
fn test() {
  let arr: Vec<Clothes> = vec![Clothes { size: 10 }, Clothes { size: 15 }];
  assert_eq!(clothes_fit_me(arr, 10), vec![Clothes { size: 10 }]);
}
```

#### 创建自定义迭代器

使用Iterator trait来创建自定义迭代器，主要就是要去实现next方法

```rust
struct Counter {
  count: u32,
}

impl Counter {
  fn new() -> Counter {
    Counter { count: 0 }
  }
}

//自定义迭代器
impl Iterator for Counter {
  //type方法后面会讲到
  type Item = u32;

  fn next(&mut self) -> Option<Self::Item> {
    if self.count < 3 {
      //小于3时每次迭代将值加1并返回这个值
      self.count += 1;
      Some(self.count)
    } else {
      None
    }
  }
}

#[cfg(test)]
#[test]
fn test() {
  let mut counter = Counter::new();

  //测试通过
  assert_eq!(counter.next(), Some(1));
  assert_eq!(counter.next(), Some(2));
  assert_eq!(counter.next(), Some(3));
  assert_eq!(counter.next(), None);
}


#[test]
fn test_other_iterator() {
  //这里相当于[1,2,3]和[2,3]通过zip拉起来形成一个元组，然后map后得到的结果是[2, 6]
  //再通过filter后得到[6]，再通过sum就是返回6了
  let sum: u32 = Counter::new()
    .zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();

  assert_eq!(sum, 6)
}
```

#### 零成本抽象

使用迭代器写循环会比手写循环的效率高一些。

### Cargo发布

#### 通过release profile(发布配置)来自定义构建

- cargo本身预定义了
- 也可以自定义，使用不同配置，对代码编译拥有更多的控制权
- 每个Profile的配置都独立于其它的profile

Cargo具有两种主要的Profile：

- dev profile:适用于开发，cargo build
- release profile:适用于发布，cargo build --release

自定义Profile的配置：

在Cargo.toml里面添加[profile.xxx]区域，比如[profile.release]，可以在里面覆盖默认配置的子集，通常只需要写需要覆盖的默认配置就行。下面是例子，对开发和生产环境的编译代码进行不同程度的优化，等级越高越花费时间，但是优化的代码也越好。

```rust
[package]
name = "demo"
version = "0.1.0"
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]

# opt-level决定了编译的时候对代码进行什么程度的优化，值是从0到3
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

关于Cargo的更多相关信息可以[查看这里的官方文档](https://doc.rust-lang.org/cargo/index.html)

#### 如何在`https://crates.io/`上发布自己的库？

crates.io网站可以下载别人的库或者发布自己的库给别人用，跟npm官网差不多，它会分发已经注册了的包，托管开源代码。

文档注释：用于生成项目的HTML文档，显式公共API的文档注释，给别人解释如何使用API。文档注释使用///，三个斜线。语法支持Markdown的语法。

```rust
/// Add One to the given number
/// # Example：
/// ```
/// let num = demo::add_one(5);
/// assert_eq!(num, 6);
/// ```
pub fn add_one(x: i32) -> i32 {
  x + 1
}
```

#### 生成HTML文档的命令

```rust
cargo doc

# 生成文档并自动在浏览器打开
cargo doc --open 
```

它会运行rustdoc工具(Rust安装包自带)

常用的Doc章节：

- \# Examples

- 其它常用的章节：- Panics(函数可能发生Panic的场景)、- Errors(如果函数返回Result枚举，描述可能的错误种类，以及可导致出错的条件)、- Safety(如果函数处于unsafe调用，就应该解释函数unsafe的原因，以及调用者确保的使用前提)

#### 文档注释作为测试

文档注释中的示例代码块，运行cargo test时，会将文档注释中的示例代码作为测试来运行

#### 为包含注释的项添加文档注释

使用符号//!，这类注释通常描述crate和模块：

- crate root(按惯例src/lib.rs)
- 一个模块内，将crate或模块作为一个整体进行记录

比如说我们有一个库，那么这个库的作用是干嘛的，就可以用//!来描述，一般这段描述就是放在src/lib.rs的顶部

```rust
//! # 测试专用库 
//! 这个库是用来测试的
//! 测试用的标题
```

#### pub use

使用pub use导出方便使用的公共API。

问题所在：crate的程序结构在开发时对开发者很合理，但对于它的使用者来说不够方便。开发者通常会把程序结构分为很多层，使用者想找某个层级深的结构的类型很费劲：

麻烦：my_crate::some_module::another_module::UsefulType;

方便：my_crate::UsefulType;

解决方法：

- 不需要重新组织内部代码结构
- 使用pub use关键字：可以重新导出，创建一个和内部私有结构不同的对外公共结构

```rust
// src/lib.rs

pub mod utils {
  pub fn test_add_one(x: i32) -> i32 {
    x + 1
  }
}
```

```rust
// src/main.rs

//可以看到这里使用了三层的导入结构，如果想要缩短就可以在lib.rs里使用pub use导出
use demo::utils::test_add_one;

fn main() {
  let x = test_add_one(1);
  println!("{}", x);
}
```

在lib.rs中将模块导出使用pub use关键字进行重导出

```rust
// src/lib.rs
pub use utils::test_add_one;

pub mod utils {
  pub fn test_add_one(x: i32) -> i32 {
    x + 1
  }
}
```

```rust
// src/main.rs

//缩短了导入的代码
use demo::test_add_one;

fn main() {
  let x = test_add_one(1);
  println!("{}", x);
}
```

#### 创建账号

发布自己的crate之前，需要在[crates.io](https://crates.io/)网站中创建账号并且取得API token，可以使用github账号登陆。

- 进入网站后，点击右上角的Log in Github并授权。
- 点击头像，选中Account Settings
- 拉到底部，看到API Access旁边的New Token，输入Token name
- 得到一个token。在控制台输入cargo login 你得到的token。然后API token会被保存到你的本地~/.cargo/credentials文件下
- 如果不小心泄露了token，可以登录网站去进行撤销，并重新生成token
- 发布包之前也得去Account Settings里去填写你的邮箱地址

#### 添加包信息

发布crate之前，需要在Cargo.toml的[package]区域添加一些关于包的相关信息。

- name:包名(唯一的)
- description:会出现在crate搜索的结果中
- license:许可证标识值提供（到http://spdx.org/licenses/查找)，多个许可之间用OR隔开
- version:版本号
- author:作者名

```rust
cargo publish
```

运行该命令发布你的包。crate一旦发布，就是永久性的：该版本不能覆盖，代码不能删除。因为有些项目会依赖到你的包的，如果你的包删了那不就出大事情了？

如果发布已经存在crate的新版本，需要修改Cargo.toml里的version值，再进行重新发布，再执行`cargo publish`发布。

#### 撤回版本

不可以删除crate之前的版本，但是可以撤回。

使用该命令去撤回：

```rust
cargo yank --vers 1.0(版本号)
```

取消撤回的命令：

```rust
cargo yank --vers 1.0 --undo
```

撤回命令可以防止其它项目把它作为新的依赖，防止新项目依赖于这个版本，而已经依赖了这个版本的项目则可以继续下载它作为依赖。

- 所有已经产生Cargo.lock的项目都不会被中止
- 任何将来生成的Cargo.lock文件都不会使用被yank的版本

#### Workspaces(工作空间)

##### 概念

- 帮助管理多个互相关联且需要协同开发的crate
- 一套共享同一个Cargo.lock和输出文件夹的包

##### 创建工作空间

- 新建一个空的add文件夹，在vscode中打开它，然后新建一个Cargo.toml文件，里面写上内容，表示我们要创建一个叫做adder的工作区域

  ```rust
  [workspace]
  members = [
      "adder"
  ]
  ```

- 然后在add文件夹之下运行`cargo new adder`，然后就可以开始workspace的开发了，此时运行`cargo build`命令，会在add根目录下生成target和Cargo.lock文件，而不是在adder文件夹里生成，target存放所有成员的产出物，即使你进入到adder文件夹里去执行`cargo build`命令，它也是会把编译产出物打包到根目录的target中。

- 接下来，我们添加第二个成员，它是一个库crate。修改wokspace里面的members

  ```rust
  [workspace]
  members = [
      "adder",
      "add-one"
  ]
  ```

  然后在add目录下执行命令`cargo new add-one --lib`，生成一个库crate。然后在里面的src/lib.rs中编写调用代码。

  ```rust
  pub fn add_one(x: i32) -> i32 {
      x + 1
  }
  ```

- 接下来我们的adder工作区需要依赖到add-one这个库里的add_one这个函数，那我们就去adder的Cargo.toml里修改dependences

  ```rust
  # adder/Cargo.toml
  [package]
  name = "adder"
  version = "0.1.0"
  edition = "2018"
  
  [dependencies]
  add-one = { path = "../add-one" }
  ```

  然后我们就可以在adder里的main.rs中调用这个库函数了

  ```rust
  use add_one::add_one;
  
  fn main() {
      let two = add_one(1);
      //打印结果：2
      println!("打印结果：{}", two);
  }
  ```

##### 工作空间内依赖外部的crate

工作空间内只有一个Cargo.lock文件，在工作空间的顶层目录。这是为了保证：

- 工作空间内所有crate使用的依赖的版本号相同
- 工作空间内所有的crate相互兼容

##### 测试

在工作空间内运行cargo test会一次性运行工作空间内所有的test测试。如果需要指定测试某一个工作区，可以使用下面的命令：

```rust
cargo test -p add-one
```

##### 发布

如果需要发布包，需要单独进入到对应的文件目录下去执行cargo publish，只能对每个包单独发布。

#### 从Crate.io安装二进制crate

命令：

```rust
cargo install crate名称
```

来源：https://crates.io

限制：只能安装具有二进制目标的crate。

二进制crate指的是一个可以运行的程序，由拥有src/main.rs或其它被指定为二进制文件的crate生成

##### cargo install

这个命令安装的二进制存放在根目录的bin文件夹中，如果用rustup安装的Rust，没有任何自定义配置，那么二进制存放的目录就是$HOME/.cargo/bin，但是要确保该目录在环境变量$PATH中。

Windows系统查看环境变量，在控制台中输入：

```rust
echo %PATH%
```

其它操作系统如Mac的查看环境变量是在控制台输入：

```
echo $PATH
```

##### 使用自定义命令扩展cargo

如果环境变量中存在某个二进制是cargo something，那么就可以像子命令一样去运行

```
cargo something
```

另外，我们可以通过下面的命令去查找出相关的cargo的子命令，这样设计的好处是我们可以使用cargo install来扩展我们的cargo，像内置工具一样来运行

```
cargo --list
```

### 智能指针

#### 指针概念

指针：一个变量在内存中包含的是一个地址（指向其它数据）

Rust中最常见的指针是“引用”，使用&表示，借用它指向的值，没有其余的开销，它也是最常见的指针类型。

#### 智能指针概念

行为和指针类型、拥有额外的元数据和功能

引用计数智能指针类型：通过记录所有者的数量，使一份数据被多个所有者同时持有，并在没有任何所有者时自动清理数据

引用和智能指针的不同：

- 引用：借用数据
- 智能指针：很多时候拥有它所指向的数据

#### 例子

我们之前使用到过的String和Vec\<T>都是智能指针，它们都拥有一片内存区域，且允许用户对其操作，还拥有元数据（例如容量）、可以提供额外的功能或保障（比如String保障其数据是合法的UTF-8编码）

#### 实现

通常使用struct来实现，并且实现了Deref和Drop这两个trait

- Deref trait：允许智能指针struct的实例像引用一样使用
- Drop trait：允许你自定义当智能指针实例走出作用域时的代码

#### 使用Box\<T>指向堆上的数据

Box\<T>是最简单的智能指针：允许在堆上存储数据而不是栈上存储，栈上是指向堆数据的指针，它没有性能开销，也没有其它额外的功能。 它也实现了Deref trait和Drop trait（这两个东西后面具体讲）

##### 使用场景

- 编译时，某类型的大小无法确定。但使用该类型时，上下文却要知道它的确切大小。
- 当你有大量数据，想移交所有权时，但需要确保在操作时数据不会被复制。
- 使用某个值时，你只关心它是否实现了特定的trait，而不关心它的具体类型。

##### Box<T>如何在堆上存储数据？

```rust
fn main() {
    //这个数据会被存在堆中
    let b = Box::new(5);
    // b = 5
    println!("b = {}", b);
} //走出作用域时，会同时释放栈中的指针和堆中的数据
```

##### Box赋能递归类型

在编译时，Rust需要知道一个类型所占的空间大小，而递归类型的大小不能在编译时确定。

但是Box类型的大小是确定的，递归的类型就可以使用Box来解决问题，这里对应的也是上面说到的第一种使用场景。

而Rust如何确定非递归类型的空间大小呢？

Rust会对枚举类型判断，遍历所有变体找到需要存储最大变体的空间大小，那这个变体的大小就是枚举类型的空间大小

```rust
enum Message {
    //Quit变体不需要占用任何空间
    Quit,
    //需要存储两个i32类型的空间
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

##### 使用Box来获得确定大小的递归类型

Box\<T>是一个指针，Rust知道它需要多少空间大小，因为指针的大小不会基于它所指向的数据的大小变化而变化。

比如我们要在Rust中实现一个链表数据结构，该怎么表示：

```rust
use crate::List::{Cons, Nil};

fn main() {
    //形成类似于这样的一个链表结构： 1->2->3->null
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

#### Deref trait

Deref(defeference解引用的意思)，实现Deref trait可以使我们可以自定义解引用运算符*的行为。通过Defer trait，智能指针可以像常规引用一样来处理。

##### 解引用运算符

解引用运算符也可以理解成取地址运算符，就是去拿到对应的内存地址里的值。

```rust
// src/lib.rs

#[test]
fn test() {
  let x = 5;
  // y指向x的引用
  let y = &x;

  //将y解引用获取它的值
  assert_eq!(x, *y);
}
```

##### 把Box\<T>当作引用

Box\<T>可以代替上述引用，上面的例子可以改写成这样

```rust
// src/lib.rs
#[test]
fn test() {
  let x = 5;
  let y = Box::new(5);

  //将y解引用获取它的值
  assert_eq!(x, *y);
}
```

##### 自定义智能指针

自定义智能指针需要实现标准库中Deref trait中要求我们实现的deref方法，该方法借用self，返回一个指向内部数据的引用。

可以代码编写完成之后运行`cargo run`看看是否有问题，这里我们自己实现了一个智能指针

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    //关联类型，后面说
    type Target = T;
    fn deref(&self) -> &T {
        //返回一个引用的值，所以外面可以通过解引用拿到这个值
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(5);

    //解引用获取y的值
    assert_eq!(x, *y);
}
```

##### 函数和方法的隐式解引用转换(Defef Coercion)

隐式解引用转换是为函数和方法提供的一种便捷特性

假设T实现了Defef trait：

Deref Coercion可以把T的引用转换成T经过Defef操作后生成的引用

当把某类型的引用传递给函数或方法时，但它的类型和定义的参数类型不匹配，此时：

- 隐式解引用转换就会自动发生
- 编译器会对deref进行一系列调用，来把它转换成所需的参数类型
- 编译完成时，没有额外的性能开销

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(str: &str) {
    println!("Hello, {}", str);
}

fn main() {
    let y = MyBox::new(String::from("World"));

    //这里的&y，编译器会发生一系列隐式解引用过程
    // 1.&MyBox<String>, MyBox实现了Dref trait，可以解引用
    // 2.String类型实现了Deref trait，可以解引用&String，返回一个&str
    // 3.最后 deref &str,匹配到了
    hello(&y);
}

```

##### 解引用和可变性

这一部分目前可以先进行了解，以后可以深入研究：

- 可使用DerefMut trait重载可变引用的*运算符

- 在类型和trait在下列三种情况发生时，Rust会执行deref coercion：

  1.当T:Deref<Target = U>，允许&T转换为&U。(这句话的意思是：当类型T，它实现了Deref trait，它里面实现的deref方法，返回的类型是U，那么T的引用就可以转换为U的引用)

  2.当T:Deref<Target = U>，允许&mut T转换为&mut U

  3.当T:Deref<Target = U>，允许&mut T转换为&U。(Rust能将一个可变引用转换为不可变引用，反过来不行)



### Drop trait

##### 概念

实现Drop trait，可以让我们自定义当值要离开作用域时发生的动作。

例如：文件、网络资源释放

任何类型都可以实现Drop trait，只要求实现一个drop方法就行(参数是对self的可变引用)。Drop trait在预导入模块prelude中。

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("释放数据:{}", self.data);
    }
}

fn main() {
    let a = CustomSmartPointer {
        data: String::from("测试1"),
    };
    let b = CustomSmartPointer {
        data: String::from("测试2"),
    };

    //可以看到释放数据的顺序和创建时是反过来的
    // 自定义智能指针创建
    // 释放数据:测试2
    // 释放数据:测试1
    println!("自定义智能指针创建");
}
```

##### 提前drop值

我们很难去禁用自动的Drop功能，也没必要去禁用，Drop trait存在的目的就是进行自动的释放处理逻辑。

另外，Rust也不允许我们自己手动调用drop方法，比如上面的main函数里，手动调用drop方法就会报错：

```rust
let a = CustomSmartPointer {
    data: String::from("测试1"),
};
//报错：explicit destructor calls not allowed
//显式调用构造器是不允许的
a.drop();
```

我们不能手动调用Drop trait里的drop函数，但是我们可以调用标准库中的`std::mem::drop`函数，来达到提前drop值的目的。可以看看下面的例子

```rust
use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("释放数据:{}", self.data);
    }
}

fn main() {
    let a = CustomSmartPointer {
        data: String::from("测试1"),
    };
    //提前drop掉a的值
    drop(a);
    let b = CustomSmartPointer {
        data: String::from("测试2"),
    };

    // 释放数据:测试1
    // 自定义智能指针创建
    // 释放数据:测试2
    println!("自定义智能指针创建");
}
```



### Rc\<T>引用计数智能指针

#### 概念

有时候，一个值会有多个所有者，为了支持多重引用，Rust引入了引用计数Rc(Reference counting)。

它可以追踪到所有值的引用，如果引用为0，这个值就可以被清理掉了

#### 使用场景

需要在堆上分配数据，这些数据被程序的多个部分读取（只读），但在编译时无法确定哪个部分最后使用完这些数据。

Rc<T>只能用于单线程的场景。

Rc<T>不在预导入模块中，我们需要手动导入

- Rc::clone(&a)函数：增加引用计数
- Rc::strong_count(&a)：强引用计数。获得引用计数
- Rc::weak_count(&a)：弱引用计数。也是获得引用计数

如果我们要实现这样的一个数据结构，要怎么做呢？

```rust
b(3)
   ↘
     a -> 5 -> 10 -> Nil
   ↗
c(4)
```

```rust
use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // 创建一个 5->10->null的链表
    // 此处a的引用计数为1
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));

    //b指向a
    //此处a的引用计数为2
    let b = Cons(3, Rc::clone(&a));
    //c也指向a
    ///此处a的引用计数为2
    let c = Cons(4, Rc::clone(&a));
}
```

接下来我们再看一个关于引用计数的例子

```rust
use crate::List::{Cons, Nil};
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // 创建一个 5->10->null的链表
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("a的引用计数为：{}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("a的引用计数为：{}", Rc::strong_count(&a));

    {
        //离开这个作用域之后引用计数-1
        let c = Cons(4, Rc::clone(&a));
        println!("a的引用计数为：{}", Rc::strong_count(&a));
    }

    println!("a的引用计数为：{}", Rc::strong_count(&a));
}

//最后的打印结果：
// a的引用计数为：1
// a的引用计数为：2
// a的引用计数为：3
// a的引用计数为：2
```

#### Rc::clone方法和类型的clone方法

比如说上面的代码中的a是可以执行自身类型上的clone方法的，那这个方法和Rc::clone方法有什么区别呢？

```rust
let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
a.clone();
```

- Rc::clone()会增加引用，不会执行数据的深拷贝操作。
- 类型的clone()：很多会执行数据的深拷贝操作，会比较耗费时间和性能。

#### Rc\<T>是不可变引用

Rc\<T>通过不可变引用，使得可以在程序不同部分之间共享只读数据。如果是可变引用的话，就会违反借用规则，多个指向同一个区域的可变引用会产生数据的竞争和数据的不一致问题。但是我们如果想要让数据变成可变引用的话该咋办呢？继续往下看

### 内部可变性

#### 概念

内部可变性是Rust的设计模式之一，它允许你仅持有不可变引用的前提下对数据进行修改。

这里面的数据结构中使用了unsafe代码来绕过Rust正常的可变性和借用规则

#### RefCell\<T>

和Rc\<T>不一样，RefCell\<T>类型代表了其持有数据的唯一所有权。

和Rc\<T>一样的，只能用于单线程的场景。

这里再回顾一下借用规则：

- 在任何给定的时间里，你要么只能拥有一个可变引用，要么只能拥有任意数量的不可变引用
- 引用总是有效的

#### Box\<T>和RefCell\<T>的区别

| Box\<T>                      | RefCell\<T>              |
| ---------------------------- | ------------------------ |
| 编译阶段强制代码遵守借用规则 | 只会在运行时检查借用规则 |
| 否则就会出现错误             | 否则触发panic            |

#### 借用规则在不同阶段进行检查的比较

| 编译阶段               | 运行时                                                 |
| ---------------------- | ------------------------------------------------------ |
| 尽早暴露问题           | 问题暴露延后、甚至会到生产环境中                       |
| 没有任何运行时开销     | 因借用计数产生些许性能损失                             |
| 对大多数场景是最佳选择 | 实现某些特定的内存安全场景（不可变环境中修改自身数据） |
| 是Rust的默认行为       |                                                        |

#### 如何选择Box\<T>、Rc\<T>、RefCell\<T>

|                    | Box\<T>                        | Ref\<T>                  | RefCell\<T>                    |
| ------------------ | ------------------------------ | ------------------------ | ------------------------------ |
| 同一数据的所有者:  | 一个                           | 多个                     | 一个                           |
| 可变性、借用检查： | 可变、不可变借用（编译时检查） | 不可变借用（编译时检查） | 可变、不可变借用（运行时检查） |

#### 可变地去借用一个不可变的值

我们可以使用RefCell\<T>在运行时记录借用信息。在此之前，我们要先了解两个方法(安全接口)

- borrow方法：返回智能指针Ref\<T>，它实现了Deref trait
- borrow_mut方法：返回智能指针RefMut\<T>，它实现了Deref trait

RefCell\<T>会记录当前存在多少个活跃的Ref\<T>和RefMut\<T>智能指针：

- 每次调用borrow方法，不可变借用计数加1
- 任何一个Ref\<T>的值离开作用域被释放时：不可变借用计数加减1
- 每次调用borrow_mut：可变借用计数加1
- 任何一个RefMut\<T>的值离开作用域被释放时：可变借用计数减1

Rust是用这个技术来维护借用检查规则的：在任何一个给定时间里，只允许拥有多个不可变借用或一个可变借用

下面的代码就是做一个总结，然后写一个例子去举例说明RefCell的内部可变性，用其来可变借用一个本身是不可变的值。

```rust
pub trait Messenger {
  fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
  messenger: &'a T,
  value: usize,
  max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
  T: Messenger,
{
  pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
    LimitTracker {
      messenger,
      value: 0,
      max,
    }
  }

  pub fn set_value(&mut self, value: usize) {
    self.value = value;

    let percentage_of_max = self.value as f64 / self.max as f64;

    if percentage_of_max >= 1.0 {
      self.messenger.send("错误：超出了最大的设置值");
    } else if percentage_of_max >= 0.9 {
      self.messenger.send("紧急警告：你已经使用了超过设置值的90%");
    } else if percentage_of_max >= 0.75 {
      self.messenger.send("警告：你已经使用了超过设置值的75%");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;

  struct MockMessenger {
    //注意RefCell
    sent_messages: RefCell<Vec<String>>,
  }

  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger {
        //注意RefCell
        sent_messages: RefCell::new(vec![]),
      }
    }
  }

  impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      //注意borrow_mut，可变的去借用一个本身不可变的值
      self.sent_messages.borrow_mut().push(String::from(message));
    }
  }

  #[test]
  fn it_send_an_over_75_percent_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);
    //断言测试的messenger里的所存储的长度为1
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
  }
}
```



####  将Rc\<T>和RefCell\<T>结合使用实现拥有多重所有权的可变数据

```rust
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    // a -> Cons(RefCell { value: 15 }, Nil)
    // b -> Cons(RefCell { value: 6 }, Cons(RefCell { value: 15 }, Nil))
    // c -> Cons(RefCell { value: 10 }, Cons(RefCell { value: 15 }, Nil))
    println!("a -> {:?}", a);
    println!("b -> {:?}", b);
    println!("c -> {:?}", c);
} 
```

#### 其它的可以实现内部可变性的类型

- **Cell\<T>：**通过复制来访问数据，区别于RefCell的借用规则来访问数据
- **Mutex\<T>：**用于实现跨线程情景下的内部可变性

### 循环引用导致内存泄露

#### 概念

Rust的内存安全机制可以保证很难发生内存泄漏，但不是绝不可能。使用Rc\<T>和RefCell\<T>就可能创造出循环引用，从而引发内存泄漏：每个项的引用数量不会变成0，值也就不会被处理掉

#### 防止内存泄漏发生的方法

- 依靠开发者来保证，不能只依靠Rust编译器。 

- 重新组织数据结构：一些引用来表达所有权，一些引用不表达所有权：循环引用中一部分具有所有权关系，另一部分不涉及所有权关系，而只有所有权关系才影响值的清理

**为了防止循环引用，可以把Rc\<T>换成Weak\<T>**

- Rc::clone为Rc\<T>实例的strong_count加1，Rc\<T>的实例只有在strong_count为0的时候才会被清理

- Rc\<T>实例通过调用Rc::downgrade方法可以创建值的Weak Reference(弱引用)，返回类型是Weak\<T>(智能指针)，调用Rc::downgrade会为weak_count加1
- Rc\<T>使用weak_count来追踪存在多少Weak\<T>
- weak_count不为0并不影响Rc\<T>实例的清理

#### Strong VS Weak

- 强引用是关于如何分享Rc\<T>实例的所有权，而弱引用不是
- 使用弱引用并不会创建循环引用：当强引用数量为0时，弱引用会自动断开
- 在使用Weak\<T>之前，需要保证它所指向的值仍然存在：在Weak\<T>实例上调用upgrade方法，返回Option\<Rc\<T>>

### 线程和进程

#### 概念

并发：程序的不同部分之间独立的执行

并行：程序的不同部分同时运行

大部分的操作系统中，代码运行在进程中，操作系统管理多个进程。程序中，各个部分可以同时运行，运行的这些独立部分就是线程。

多线程：提高性能表现，但是复杂度也会增加（不能保证各个线程的执行顺序）

#### 多线程会导致的问题

- 竞争：线程以不一致的顺序访问数据或资源
- 死锁：两个线程彼此等待对方使用完所持有的资源，线程不能继续
- 难以复现：只在某些情况下发生的Bug，很难可靠地复制现象和修复

#### 线程实现

- 通过操作系统的API来创建线程- 1：1模型，需要较小的运行时 

- 通过语言自己实现的线程（绿色线程）- M：N模型，需要较大的运行时
- Rust需要权衡运行时的支持，Rust标准库仅提供1：1模型的线程，但是在社区里也有实现了M：N模型的第三方包

#### 创建线程

- 通过thread::spawn函数创建新线程：参数是一个闭包（需要在新线程中运行的代码）  
- thread::sleep会导致当前线程暂停执行

#### 等待子线程执行完成

- thread::spawn函数的返回值类型是JoinHandle
- JoinHandle持有值的所有权，调用其join方法，可以等待对应的其它线程的完成
- join方法：调用handle的join方法会阻止当前运行线程的执行，直到handle所表示的这些线程终结

```rust
use std::thread;
use std::time::Duration;

fn main() {
  let handle = thread::spawn(|| {
    for i in 1..10 {
      println!("子线程中打印：{}", i);
      thread::sleep(Duration::from_millis(1));
    }
  });

  for i in 1..5 {
    println!("主线程中打印：{}", i);
    thread::sleep(Duration::from_millis(1));
  }

  //join方法阻塞主线程的执行，直到handle所对应的线程结束
  //如果不加这一段的话主线程一执行完就会结束程序，但分线程还没执行完
  handle.join().unwrap();
}
```

#### move闭包

move闭包通常和`thread::spawn`函数一起使用，它允许你在当前线程使用其它线程的数据。创建一个线程时，可以把值的所有权从一个线程转移到另一个线程里。

```rust
use std::thread;

fn main() {
  let arr = vec![1, 2, 3, 4];

  let handle = thread::spawn(move || {
    println!("{:?}", arr);
  });

  handle.join().unwrap();
}
```

#### 使用消息传递来跨线程传递数据

消息传递是一种能保证安全并发的技术。线程通过彼此发送消息(数据)来进行通信。

Channel：一个管道（包含发送端、接收端），调用发送端的方法来发送数据，接收端会检查和接收到达的数据。如果发送端、接收端任意一方被丢弃了，那么Channel就关闭了。

##### 创建Channel

使用`mpsc::channel`函数(multiple producer singgle consumer多个生产者、一个消费者)来创建Channel。它返回一个元组，里面的元素分别是发送端、接收端

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
  let (s, r) = mpsc::channel();

  thread::spawn(move || {
    let str = String::from("子线程发送的数据");
    s.send(str).unwrap();
  });

  let receive = r.recv().unwrap();
  //主线程收到子线程发来的数据：子线程发送的数据
  println!("主线程收到子线程发来的数据：{}", receive);
}
```

##### 发送端的send方法

- 参数：想要发送的数据 

- 返回：Result\<T,E>，如果有问题（例如接收端已经被丢弃），就会返回一个错误

##### 接收端方法

- recv方法：阻止当前线程执行，直到Channel中有值送过来，一旦有值收到，就会返回Result\<T,E>，当发送端关闭，就会收到一个错误 

- try_recv方法：不会阻塞线程，立即返回Result\<T,E>，有数据到达返回OK，里面包含数据，否则返回Err。通常使用循环调用来检查try_recv的结果。 

#### 使用共享内存来实现并发

Channel类似单所有权，一旦所有权移动到Channel，就不能使用它了。共享内存的并发方式类似于多所有权，多个线程可以同时访问同一块内存。

Rust中可以使用`Mutex`(互斥)来实现只允许一个线程来访问数据，想要访问数据线程必须要先获得互斥锁(lock)，lock数据结构是Mutex的一部分，它能跟踪谁对数据拥有独占访问权。mutex通常被描述为：通过锁定系统来保护它所持有的数据。

##### Mutex所使用的两条规则

- 在使用数据之前，必须尝试获得锁(lock)
- 使用完mutex所保护的数据，必须对数据进行解锁，以便其它线程可以获得锁

##### Mutex\<T>的API

通过`Mutex::new`(数据)来创建`Mutex<T>`，`Mutex<T>`是一个智能指针，访问数据前使用lock方法来获取锁，这个方法会阻塞线程的执行，lock方法也可能失败，返回值是`MutexGuard`(智能指针，实现了Deref和Drop trait)

##### 使用Arc\<T>来进行原子引用计数

`Arc<T>`和`Rc<T>`类似，它俩API都是相同的，`Arc<T>`可以用于并发情景，A是atomic，原子的意思。为什么所有的基础类型都不是原子的原因就在于原子类型需要付出性能作为代价。

下面的例子是可以将counter的值加10

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
  let counter = Arc::new(Mutex::new(0));
  let mut handles = vec![];

  for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
      let mut num = counter.lock().unwrap();
      *num += 1;
    });
    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  //结果是:10
  println!("结果是:{}", *counter.lock().unwrap());
}
```

##### RefCell\<T>、Rc\<T>和Mutex\<T>、Arc\<T>

- `Mutex<T>`提供了内部可变性，和Cell家族一样
- 可以使用`RefCell<T>`来改变`Rc<T>`里的内容
- 可以使用`Mutex<T>`来改变`Arc<T>`里的内容
- `Mutex<T>`有产生死锁的风险

##### Send和Sync trait

Rust语言实现的并发特性较少，语言本身并没有实现并发，目前我们遇到的并发都是标准库中的。但是我们也可以自己去实现并发的相关库。Rust中有两个并发的概念，`std::marker::Sync`和`std::marker::Send`这两个标签trait，它们没有实现任何方法。

- Send trait：允许线程间转移所有权，Rust中几乎所有类型都实现了这个trait，但是`Rc<T>`是例外，它只适用于单线程的情景。任何完全由Send类型组成的类型也被标记为Send，除了原始指针之外，几乎所有的基础类型都是Send。
- Sync trait：允许从多线程访问，实现了`Sync trait`的类型可以安全的被多个线程引用。也就是说：如果T是Sync，那么&T就是Send，引用可以被安全的送往另一个线程，基础类型都是Sync，完全由Sync类型组成的类型也是Sync，但是`Rc<T>`不是Sync类型，RefCell<T>和`Cell<T>`家族也不是Sync类型，而`Mutex<T>`是Sync类型

**手动实现Send和Sync这两个trait是不安全的，涉及到unsafe的代码，需要设计的很精细去保证线程之间的安全性。**

### 面向对象编程

Rust受到了多种编程范式的影响，也包括面向对象。

面向对象包括以下特性：命名对象、封装、继承。。。

#### 概念

对象包括数据和行为，面向对象的定义是：面向对象的程序由对象组成，对象包括了数据和操作这些数据的过程，这些过程通常被称为方法或者操作。基于这个定义，我们可以知道Rust是面向对象的，同时也可以满足这些定义。

#### 封装

调用对象外部的代码无法直接访问对象内部的实现细节，唯一可以和对象进行交互的方法就是访问它公开的API。Rust中的pub关键字可以将方法暴露。

#### 继承

使得对象可以沿用另外一个对象的数据和行为，且不需要重复定义相关的代码。但是在Rust中是没有继承的，在Rust中要实现代码复用的话可以使用默认trait方法来进行代码共享。

多态：Rust中使用泛型和trait约束可以实现多态。

#### 使用trait对象来存储不同的值

一个需求是这样的，希望可以创建一个图形工具，它会遍历某个元素列表，依次调用元素的draw方法来进行绘制，比如Button、Text等元素。在面向对象的语言中实现方式一般是定义一个Component父类，里面定义一个draw方法，然后定义Button、Text子类，去继承Component父类。

而在Rust中怎么去实现这样的需求呢？

Rust中可以为共有行为定义一个trait。Rust避免将struct和enum称为对象，因为它们和impl块是分开的。

##### trait对象跟别的语言的对象的区别是什么？

trait对象其实有点类似于别的语言中的对象，它们某种程度组合了数据和行为，但是不同的地方是trait对象不能添加数据。trait对象被专门用于抽象某一些共有的行为，没别的语言中的那么通用。

接下来我们看个例子去实现这个基本的需求。

```rust
// src/lib.rs

pub trait Draw {
  fn draw(&self);
}

pub struct Screen {
  //Box用来定义trait对象，这个结构用来表示Box里的元素都要实现了Draw这个trait
  //而这里如果改成泛型来存储就只能存某一种元素类型，不够灵活
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw()
    }
  }
}

// -----泛型的实现方式-----
// pub struct Screen<T: Draw> {
//   pub components: Vec<T>,
// }

// impl<T> Screen<T>
// where
//   T: Draw,
// {
//   pub fn run(&self) {
//     for component in self.components.iter() {
//       component.draw()
//     }
//   }
// }

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub text: String,
}

impl Draw for Button {
  fn draw(&self) {
    //绘制按钮
  }
}

pub struct Text {
  pub text: String,
}

impl Draw for Text {
  fn draw(&self) {
    //绘制一个文本框
  }
}
```



```rust
// src/main.rs
use demo::Draw;
use demo::{Button, Screen, Text};

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
    // 绘制选择框
  }
}

fn main() {
  let screen = Screen {
    //components里的元素都必须要实现了Draw这个trait才能存进去，否则就会报错
    components: vec![
      Box::new(SelectBox {
        width: 25,
        height: 25,
        options: vec![
          String::from("确定"),
          String::from("取消"),
          String::from("默认"),
        ],
      }),
      Box::new(Button {
        width: 15,
        height: 15,
        text: String::from("确定"),
      }),
      Box::new(Text {
        text: String::from("文本框"),
      }),
    ],
  };

  screen.run();
}
```

#### trait对象执行的是动态派发

##### 静态派发(static dispatch)

将trait对象作用到泛型上时，Rust会执行单态化，也就是说编译器会替换掉泛型参数为具体的非泛型实现。通过单态化生成的代码会执行静态派发，编译的时候会确定具体方法。类似于刚刚上面写到过的代码，这里就是会执行静态派发，一旦编译器确定了泛型参数是什么，在编译阶段中，components里就只能存这个指定了的类型的数据，而不能再存其它的。

```rust
pub struct Screen<T: Draw> {
  pub components: Vec<T>,
}

impl<T> Screen<T>
where
  T: Draw,
{
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw()
    }
  }
}
```

##### 动态派发(dynamic dispacth)

编译器不能在编译的时候确定你具体调用的是哪类方法，编译器会产生额外的代码以便可以在运行时找出希望调用的方法。使用到trait对象就会执行动态派发，但是会产生运行时的开销，它会阻止编译器内联方法代码，使得部分优化操作不能进行。

这一段components里的数据执行的就是动态派发。

```rust
pub struct Screen {
  //Box用来定义trait对象，这个结构用来表示Box里的元素都要实现了Draw这个trait
  //而这里如果改成泛型来存储就只能存某一种元素类型，不够灵活
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw()
    }
  }
}
```

#### trait对象必须保证对象安全

只能把满足对象安全的trait转化为trait对象，Rust采用一一系列规则来确定某个对象是否安全，记住以下两条即可：

- 方法的返回类型不是Self
- 方法中不包括任何泛型类型参数

在标准库中，Clone这个trait就不是一个符合对象安全的trait对象

```rust
pub trait Clone {
    fn clone(&self) -> Self;
}
```

#### 编写例子

我们的需求是要写一个博客的发布审批，在草稿状态下只能提交审批，而发布之后才能拿到正式的博客数据。

```rust
// src/main.rs

use demo::Post;

fn main() {
  //创建博客
  let mut post = Post::new();

  //博客发布内容
  post.add_text("我今天在学Rust");

  //博客请求审批
  let post = post.request_review();

  //通过博客审批
  let post = post.approve();

  //我今天在学Rust
  println!("{}", post.content());
}
```

```rust
// src/lib.rs

//正式的内容
pub struct Post {
  content: String,
}

//草稿内容
pub struct DraftPost {
  content: String,
}

//等待审批中的内容
pub struct PendingReviewPost {
  content: String,
}

//正式发布后的Post
impl Post {
  pub fn new() -> DraftPost {
    DraftPost {
      content: String::new(),
    }
  }

  pub fn content(&self) -> &str {
    &self.content
  }
}

//草稿Post
impl DraftPost {
  pub fn add_text(&mut self, text: &str) {
    self.content.push_str(text);
  }

  pub fn request_review(self) -> PendingReviewPost {
    PendingReviewPost {
      content: self.content,
    }
  }
}

//等待审批的Post中只有通过审批的方法，审批通过之后直接将正式的Post内容返回
impl PendingReviewPost {
  pub fn approve(self) -> Post {
    Post {
      content: self.content,
    }
  }
}
```

### 模式匹配

#### 模式概念

模式是Rust中的一种特殊语法，用于匹配复杂和简单类型的结构。将模式和匹配表达式和其它构造结合使用，可以更好的控制程序的控制流。模式由以下元素组成：

- 字面值
- 结构的数组、enum、struct、tuple
- 变量
- 通配符
- 占位符

#### 用法

```rust
match value {
   Patten => Expression,
   Patten => Expression,   
}
```

match表达式要求详尽，尽可能列出所有的可能值。

一个特殊的模式符：_下划线

- 它会匹配任何东西
- 不会绑定到变量
- 通常用于match最后一个分支，或用于忽略某些值

#### if let 表达式

- `if let` 表达式主要是作为一种简短的方式来等价的代替只有一个匹配项的match  

- if let 可选的可以拥有else，包括 `else if` 、 `else if let`
- `if let` 不会检查穷尽性

#### while let 条件循环

只要模式继续满足匹配的条件，那它允许while循环一直运行

```rust
fn main() {
  let mut stack = Vec::new();
  stack.push(1);
  stack.push(2);
  stack.push(3);

  // 只要能从栈中取出值就一直弹栈
  // top, 3
  // top, 2
  // top, 1
  while let Some(top) = stack.pop() {
    println!("top, {}", top);
  }
}
```

##### for循环

Rust中最常见的循环方式，它也是属于模式的一种

```rust
fn main() {
  let arr = vec![1, 2, 3, 4];

  for (index, value) in arr.iter().enumerate() {
    println!("index: {}, value: {}", index, value);
  }
}
```

#### let语句

let用于声明变量，正常写法一般是`let a = 1`

但是它也是一个模式，所以可以使用这样的写法`let (x,y,z) = (1,2,3)`

#### 函数

函数的参数也可以是模式

```rust
fn foo(x: i32) {
    //test
}

fn print_some_thing(&(x,y): &(i32, i32)) {
    println!("x: {}, y: {}", x, y);
}

fn main() {
    let dot = (1, 2);
    print_some_thing(&dot);
}
```

#### 可辩驳性

模式有两种形式：可辩驳的、不可辩驳的

- 能匹配到任何可能传递的值的模式：不可辩驳的，意思就是不能失败的，怎么匹配都会成功，例如`let x = 5`
- 对某些可能的值，无法进行匹配的模式：可辩驳的，例如`if let Some(x) = some_value`，这个some_value可能是None。
- 函数参数、let语句、for循环只接受不可辩驳的模式
- if let和while let 接受可辩驳和不可辩驳两种模式，但是编译器会抛出警告，因为可能匹配会失败

#### 模式匹配的语法

##### 字面值

```rust
fn main() {
  let x = 1;

  match x {
    1 => println!("1"),
    2 => println!("2"),
    3 => println!("3"),
    _ => println!("other"),
  }
}
```

##### 匹配命名变量

```rust
fn main() {
  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("匹配Some(50)"),
    // 匹配到的y: 5
    Some(y) => println!("匹配到的y: {:?}", y),
    _ => println!("默认匹配"),
  }

  // 外部的x: Some(5), 外部的y: 10
  println!("外部的x: {:?}, 外部的y: {}", x, y);
}
```

##### 多重模式

match表达式中，用|语法可以匹配多种模式

```rust
fn main() {
  let x = 1;

  match x {
    1 | 2 => println!("1或2都能匹配到这"),
    _ => println!("默认匹配"),
  }
}
```

##### 匹配某个范围的值

使用`..=`运算符

```rust
fn main() {
  let x = 2;

  match x {
    1..=5 => println!("匹配1到5的值"),
    _ => println!("其它"),
  }

  let a = 'a';
  match a {
    'a'..='c' => println!("a - c"),
    'd'..='z' => println!("d - z"),
    _ => println!("other"),
  }
}
```

##### 解构以分解值

可以使用模式来解构struct、enum、tuple，从而引用这些类型值的不同部分

struct结构体

```rust
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let p = Point { x: 0, y: 7 };

  //从p中解构出x和y，如果需要重新命名变量可以这样写let Point { x: a, y: b } = p;
  let Point { x, y } = p;

  println!("x: {}, y: {}", x, y);

  //x匹配到0，y随意匹配
  match p {
    Point { x: 0, y } => println!("x匹配到0，y随意匹配"),
    Point { x, y: 7 } => println!("x随意匹配，y匹配到7"),
    Point { x, y } => println!("x和y都随意匹配到"),
  }
}
```

enum枚举

```rust
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

fn main() {
  let msg = Message::ChangeColor(255, 255, 0);

  match msg {
    Message::Quit => println!("Message::Quit"),
    Message::Move { x, y } => println!("Message::Move, x:{}, y: {}", x, y),
    Message::Write(text) => println!("Message::Write, text is {}", text),
    Message::ChangeColor(r, g, b) => println!("Message::ChangeColor, r:{},g:{},b:{}", r, g, b),
  }
}
```

解构结构体和元组

```rust
struct Point {
  x: i32,
  y: i32,
}

fn main() {
  let ((a, b), Point { x, y }) = ((1, 2), Point { x: 10, y: 20 });
}
```

##### 忽略值

**_下划线忽略整个值，需要忽略的值可以直接用下划线来代替**

```rust
fn foo(_: i32, y: i32) {
  println!("y is {}", y);
}

fn main() {
  foo(3, 4);
}
```

还可以通过_开头命名变量来忽略未使用的变量，默认情况下声明了变量不使用编译器会给出警告，想要不警告的话可以使用下划线开头命名变量

```rust
//x未使用会警告
let x = 5;
//_y未使用不会警告
let _y = 5;
```

**使用..符号来忽略某一部分**

```rust
fn main() {
  let numbers = (1, 2, 3, 4, 5);

  match numbers {
    // first is 1. last is 5
    (first, .., last) => println!("first is {}. last is {}", first, last),
  }
}
```

**match守卫，增加附加判断条件**

```rust
fn main() {
  let num = Some(1);

  match num {
    Some(x) if x < 3 => println!("num is {}", x),
    Some(x) => println!("{}", x),
    None => (),
  }
}
```

**@绑定，使用@符号可以让我们创建一个变量，该变量可以在测试某个值是否和模式匹配的同时保存该值**

```rust
enum Message {
  Hello { id: i32 },
}

fn main() {
  let msg = Message::Hello { id: 5 };

  //3到7的范围内找到id: 5
  match msg {
    Message::Hello {
      id: id_variable @ 3..=7,
    } => println!("3到7的范围内找到id: {}", id_variable),
    Message::Hello { id: 10..=12 } => println!("id在10到12范围内"),
    Message::Hello { id } => println!("其它值"),
  }
}
```

