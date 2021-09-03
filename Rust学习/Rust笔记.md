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

