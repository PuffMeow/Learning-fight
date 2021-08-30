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

命令行依次执行命令，这几个工具可以给VS Code提供IDE般的体验，包括代码提示，代码告警，函数提示等。

```
cargo install racer
rustup component add rls
rustup component add rust-analysis
rustup component add rust-src
```

### Demo练习

#### 猜数游戏1

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

#### 猜数游戏2

