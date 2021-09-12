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
