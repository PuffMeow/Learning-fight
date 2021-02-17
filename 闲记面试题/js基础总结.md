## defer和async的区别

有`derer`的话,加载后续文档元素的过程将和 js 的加载并行进行（异步），但是js 的执行要在所有元素解析完成之后，`DOMContentLoaded` 事件触发之前完成，并且多个defer会按照顺序进行加载。而`async`会在js下载完成之后立即执行，哪个先加载好先执行哪个。

## document.ready和document.onload的区别

- ready：表示DOM结构已经加载完成（不包含图片等非文字媒体文件）执行先于onload

- onload：指示页面包含图片等文件在内的所有元素都加载完成

## CommonJS、ES module？

- `CommonJS`是服务器端模块的规范，Node.js 采用了这个规范。CommonJS 规范加载模块是同步的，也就是说，只有加载完成，才能执行后面的操作，在浏览器中会出现堵塞的情况。通过对module.exports 或exports的属性赋值来达到暴露模块对象。**CommonJS在运行时加载：即在输入时是先加载整个模块，生成一个对象，然后再从这个对象上面读取方法**
- ES6的导入导出是官方的实现，`export导出，import / import default导入`。**import在编译时加载：ES6 模块不是对象，而是通过 export 命令显式指定输出的代码。在import时可以指定加载某个输出值，而不是加载整个模块**，只有**ES module可以静态分析，实现Tree-shaking**

- require支持动态导入，import不支持，正在提案 (babel下可支持)
- require 同步导入， import属于异步导入
- require 是值拷贝，导出值的变化不会影响导入值； import 指向内存地址，导入值会随导出值而变化

## 说下闭包?

闭包是指有权访问另一个函数作用域中的变量的函数。利用闭包可以突破作用域链的限制，比如在一个函数里声明了一个变量和一个函数，里面这个函数可以访问到外面函数的变量。下面这个就是最简单的闭包实现，另外就是我们常用的`防抖和节流`其实就是闭包的应用。

```javascript
function out() {
    let i = 0
    function inner() {
        let j = i
    }
}
```

闭包具有以下特点：

- 内部函数可以引用外层函数的参数和变量
- 可以实现私有封装和缓存
- 参数和变量不会被垃圾回收
- 使用不当会造成内存泄漏

## forEach怎么跳出？

```javascript
try {
  let arr = [1, 2, 3, 4, 5, 6, 7]
  arr.forEach((item, i) => {
    console.log(item)
    if (item === 5) {
      throw Error('从5跳出forEach')
    }
  })
} catch (e) {
  console.log(e)
}
```

` forEach`不能使用`break`和`continue`这两个关键字，`forEach`和普通的for循环是不同的，它不是普通的遍历，实现`continue`的效果可以直接使用`return`。 

如果按正常使用`forEach`的话会遍历完所有元素才能结束循环

## 说一下Service Worker？

`service worker`其实就跟PWA挂钩，`service worker`做为PWA的核心技术之一，多年来一直被Google大力推广 。PWA就是渐进式web应用 **(Progressive Web App)** ，说简单点就是可以做离线应用了。

Js是单线程的，随着业务的复杂化，开发者在用Js来做一些复杂计算的时候，单线程的弊端就凸显了。`web worker`被创造出来解决这个问题，它是脱离在主线程之外的，我们可以将复杂耗费时间的事情交给`web worker`来做。而`service worker`便是在`web worker`的基础上增加了离线缓存的能力。

说说它的特点

- 事件驱动，完全异步，具有生命周期
- 无法操作DOM，无法使用localStorage
- 出于安全考虑，**只能在https中使用**
- 可以拦截处理页面的所有网络请求
- 拥有独立上下文
-  可以访问cache和indexDB 
-  支持推送 

## 知道MutationObserver吗？

`MutationObserver`翻译过来就是变动观察器[参考详情](https://developer.mozilla.org/zh-cn/docs/web/api/mutationobserver)，用来观察页面node节点变化的。

当我们创建并返回一个新的 `MutationObserver` 时它会在指定的DOM发生变化时被调用。 该API在IE中最高只支持到IE11，但是它在IE11中也是不可靠的(从Vue的源码中我们可以看到这个描述)

```javascript
// 选择需要观察变动的节点
const targetNode = document.getElementById('some-id');

// 观察器的配置（需要观察什么变动）
const config = { attributes: true, childList: true, subtree: true };

// 当观察到变动时执行的回调函数
const callback = function(mutationsList, observer) {
    // Use traditional 'for loops' for IE 11
    for(let mutation of mutationsList) {
        if (mutation.type === 'childList') {
            console.log('A child node has been added or removed.');
        } else if (mutation.type === 'attributes') {
            console.log('The ' + mutation.attributeName + ' attribute was modified.');
        }
    }
};

// 创建一个观察器实例并传入回调函数
const observer = new MutationObserver(callback);

// 以上述配置开始观察目标节点
observer.observe(targetNode, config);

// 之后，可停止观察
observer.disconnect();
```



## 说下async和await的实现原理？

这两个玩意是`ES2017引入`的，其实它们就是 `Generator` 函数的语法糖 ， `async`函数就是将 `Generator` 函数的星号（`*`）替换成`async`，将`yield`替换成`await` 。 `async`的实现原理，它的内部就是`Promise和Generator`的封装。

```javascript
async function fn(args) {
  // ...
}

// 等同于
function fn(args) {
   //返回一个自执行函数spawn
  return function spawn(genF) {
    return new Promise((resolve, reject) => {
      const gen = genF()
      
      function step(nextF) {
        let next
        
        try {
          next = nextF()
        } catch (e) {
          return reject(e)
        }
          
        if (next.done) {
          return resolve(next.value)
        }
          
        Promise.resolve(next.value)
          .then((val) => {
            step(() => {
              return gen.next(val)
            })
          }, (e) => {
            step(() => {
              return gen.throw(e);
            })
          })
      }
        
      step(() => {
        return gen.next(undefined);
      })
    })
  }
}
```

## 浏览器的解析渲染原理以及JS、CSS阻塞问题分析

https://blog.csdn.net/liu_yunzhao/article/details/91550704

