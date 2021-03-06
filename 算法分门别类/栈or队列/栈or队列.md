### 用栈实现队列

请你仅使用两个栈实现先入先出队列。队列应当支持一般队列的支持的所有操作（push、pop、peek、empty）：

实现 MyQueue 类：

void push(int x) 将元素 x 推到队列的末尾
int pop() 从队列的开头移除并返回元素
int peek() 返回队列开头的元素
boolean empty() 如果队列为空，返回 true ；否则，返回 false

```js
var MyQueue = function () {
    this.outStack = [];
    this.inStack = [];
};

/**
 * 把元素push到队列末尾
 * @param {number} x
 * @return {void}
 */
MyQueue.prototype.push = function (x) {
    this.inStack.push(x)
};

/**
 * 移除队头元素并返回这个元素
 * @return {number}
 */
MyQueue.prototype.pop = function () {
    if (!this.outStack.length) {
        while (this.inStack.length) {
            this.outStack.push(this.inStack.pop())
        }
    }
    return this.outStack.pop()
};

/**
 * 获取队头元素
 * @return {number}
 */
MyQueue.prototype.peek = function () {
    if (!this.outStack.length) {
        while (this.inStack.length) {
            this.outStack.push(this.inStack.pop())
        }
    }
    return this.outStack[this.outStack.length - 1]
};

/**
 * 判断队列是否为空
 * @return {boolean}
 */
MyQueue.prototype.empty = function () {
    return !this.inStack.length && !this.outStack.length
};

/**
 * Your MyQueue object will be instantiated and called as such:
 * var obj = new MyQueue()
 * obj.push(x)
 * var param_2 = obj.pop()
 * var param_3 = obj.peek()
 * var param_4 = obj.empty()
 */
```

