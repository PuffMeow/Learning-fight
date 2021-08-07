### 函数和var的变量提升

**什么是变量提升**？
JavaScript 中，函数及变量的声明都将被提升到函数的最顶部。
JavaScript 中，变量可以在使用后声明，也就是变量可以先使用再声明。

- **函数提升只会提升函数声明，而不会提升函数表达式**
- **函数声明的变量提升比var的变量提升优先级要高**

### var、let 、const区别

- **var声明变量存在变量提升（将变量提升到当前作用域的顶部，为undefined），let和const不存在变量提升。**
- **let、const都是块级局部变量，if语句和for语句里面的{ }也属于块作用域。** 

- **const声明的变量不可修改，但是对象和数组里的属性可以修改**
- **同一作用域下let和const不能声明同名变量，会报错。而var可以，后一个变量会覆盖前一个变量。**

### function的length表示什么?参数长度包括默认值?

```js
function a() {}
a.length = 0

function b(a,b,c) {}
b.length = 3

function c(a = 1, b = 2, c = 3) {}
c.length = 0
```



### JS解析

#### defer和async的区别

有`derer`的话，加载后续文档元素的过程将和 Js 的加载并行进行（异步），但是Js的执行要在所有元素解析完成之后，`DOMContentLoaded` 事件触发之前完成，并且多个defer会按照顺序进行加载。而`async`会在Js下载完成之后立即执行，哪个先加载好先执行哪个。如果需要按顺序加载JS文件，并且这些文件有依赖关系，则使用defer，如果载入的JS文件没有依赖关系则使用async。

#### **不论是内联还是外链js都会阻塞后续dom的解析和渲染**

如果把JavaScript放在页面顶部，下载和解析JavaScript的时间里面，dom迟迟得不到解析和渲染，浏览器一直处于白屏，所以把JavaScript文件放在页面底部更有利于页面快速呈现。

#### Css阻塞渲染

对于一个HTML文档来说，不管是内联还是外链的css，都会阻碍后续的dom渲染，但是不会阻碍后续dom的解析。如果把css文件引用放在HTML文档的底部，浏览器为了防止无样式内容闪烁，会在css文件下载并解析完毕之前什么都不显示，这也就会造成白屏现象。虽然css解析也会阻塞后续dom的渲染，但是在解析css的同时也在解析dom，所以等到css解析完毕就会逐步的渲染页面了。

### document.ready和document.onload的区别

- ready：表示DOM结构已经加载完成（不包含图片等非文字媒体文件）执行先于onload
- onload：指示页面包含图片等文件在内的所有元素都加载完成

### JS代码如何执行

**预解析**：检查语法错误但不生成AST

**生成AST**：经过词法/语法分析，生成抽象语法树

**生成字节码**：基线编译器(Ignition)将AST转换成字节码

**生成机器码**：优化编译器(Turbofan)将字节码转换成优化过的机器码，此外在逐行执行字节码的过程中，如果一段代码经常被执行，那么V8会将这段代码直接转换成机器码保存起来，下一次执行就不必经过字节码，优化了执行速度

### CommonJS、ES module？

- `CommonJS`是服务器端模块的规范，Node.js 采用了这个规范。CommonJS 规范加载模块是同步的，也就是说，只有加载完成，才能执行后面的操作，在浏览器中会出现堵塞的情况。通过对module.exports 或exports的属性赋值来达到暴露模块对象。**CommonJS在运行时加载：即在输入时是先加载整个模块，生成一个对象，然后再从这个对象上面读取方法**
- ES6的导入导出是官方的实现，`export导出，import / import default导入`。**import在编译时加载：ES6 模块不是对象，而是通过 export 命令显式指定输出的代码。在import时可以指定加载某个输出值，而不是加载整个模块**，只有**ES module可以静态分析，实现Tree-shaking**

- require支持动态导入，import不支持，正在提案 (babel下可支持)
- require 同步导入， import属于异步导入
- require 是值拷贝，导出值的变化不会影响导入值； import 指向内存地址，导入值会随导出值而变化

### 说下闭包?

闭包是指有权访问另一个函数作用域中的变量的函数。利用闭包可以突破作用域链的限制，比如在一个函数里声明了一个变量和一个函数，里面这个函数可以访问到外面函数的变量。下面这个就是最简单的闭包实现，另外就是我们常用的`防抖和节流`其实就是闭包的应用。

```js
function out() {
    let i = 0
    function inner() {
        let j = i
    }
}

const addOne = (function() {
    let count = 0
    return {
        inc: function() {
            return count++
        }
    }
})()

addOne.inc() //1
addOne.inc() //2
```

闭包具有以下特点：

- 内部函数可以引用外层函数的参数和变量
- 可以实现私有封装和缓存
- 参数和变量不会被垃圾回收
- 使用不当会造成内存泄漏

内存泄漏是指程序中己动态分配的堆内存由于某种原因程序未释放或无法释放，造成系统内存的浪费，导致程序运行速度减慢甚至系统崩溃等严重后果。

由于闭包会使得函数中的变量都被保存在内存中，内存消耗很大，所以不能滥用闭包，否则会造成网页的性能问题，在IE中可能导致内存泄露。解决方法是，在退出函数之前，将不使用的局部变量全部删除。

### forEach怎么跳出？

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

### 说一下Service Worker？

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

### 知道MutationObserver吗？

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

### 说下async和await的实现原理？

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

### 浏览器的解析渲染原理以及JS、CSS阻塞问题分析

https://blog.csdn.net/liu_yunzhao/article/details/91550704

浏览器在渲染网页时会开启**两条线程**，**渲染引擎线程和JS引擎线程**，但这两条线程是互斥的，同时只能有一个线程在执行。也就是说在构建DOM树时，渲染引擎在执行：

**当遇到JS时**：渲染引擎会停止执行，控制权交给JS引擎，当执行JS代码时
如果遇到获取DOM，那么如果该DOM还没有解析，则会获取为null，如果JS代码还操作了CSS，而这个CSS如果还没有下载和构建，那么浏览器首先会阻塞JS引擎执行，然后会开启一个异步请求线程，在该线程上，去下载构建CSS规则树，CSS规则树构建完成后，再继续执行JS代码，当JS执行完以后，控制权再次交给渲染引擎去执行。

**当遇到CSS元素时**：也会开启异步线程，去下载构建CSS规则树，但同时也会继续构建后面的DOM树，也就是说DOM解析和CSS解析可以同时进行，但如果后面遇到JS元素，则会阻塞JS引擎线程执行，后面DOM树解析不受影响。

### 实现图片懒加载

图片占位符

```javascript
<img src="default.jpg" data-src="http://xxx/target.jpg">
```

#### 方案一：使用clientHeight、scrollTop 和 offsetTop

```javascript
let imgs = document.getElementsByTagName("img")
let count = 0  //计数器，从第一个图片开始计

lazyload()  //第一次加载图片

//对滚动监听并做节流处理
window.addEventListioner('scroll', throttle(lazyload, 200))

function lazyload() {
    let viewHeight = document.documentElement.clientHeight //视口高度
    let scrollTop = document.documentElement.scrollTop || document.body.scrollTop //滚动条卷去的高度
    for(let i = count; i < imgs.length; i++) {
        //元素已经出现在视口当中
        if(img[i].offsetTop < scrollTop + viewHeight) {
            if(img[i].getAttribute("src") !== "default.jpg") continue;
            img[i].src = img[i].getAttribute("data-src");
            count++
        }
    }
}
```

#### 方案二：getBoundingClientRect

```javascript
function lazyload() {
  for(let i = count; i < img.length; i++) {
    // 元素现在已经出现在视口中
    if(img[i].getBoundingClientRect().top < document.documentElement.clientHeight) {
      if(img[i].getAttribute("src") !== "default.jpg") continue
      img[i].src = img[i].getAttribute("data-src")
      count++
    }
  }
}
```

#### 方案三: IntersectionObserver

这是浏览器内置的一个`API`，实现了`监听window的scroll事件`、`判断是否在视口中`以及`节流`三大功能。

```js
let img = document.document.getElementsByTagName("img");

const observer = new IntersectionObserver(changes => {
  //changes 是被观察的元素集合
  for(let i = 0, len = changes.length; i < len; i++) {
    let change = changes[i];
    // 通过这个属性判断是否在视口中
    if(change.isIntersecting) {
      const imgElement = change.target;
      imgElement.src = imgElement.getAttribute("data-src");
      observer.unobserve(imgElement);
    }
  }
})
observer.observe(img);
```

### 如何判断用户是否能上网

```javascript
　　if(window.navigator.onLine==true) {　

　　　　alert("首次 -- 已连接");

　　}else {　

　　　　alert("首次 -- 未连接");

　　}

　　window.addEventListener("online", online, false);

　　window.addEventListener("offline", offline, false);

　　function online() {  alert("重新连接");  }

　　function offline() {  alert("连接断开");  }
```

或者可以进入页面的时候先请求一下百度，如果请求错误则说明用户不能上网

### JS如何判断对象中的属性

使用for in 或者Object.keys,Object.getOwnPropertyNames

for in会遍历原型链上的属性，一般配合obj.hasOwnProperty使用

Object.keys、Object.values、Object.entries遍历不包含原型链上的属性

### GET和POST的区别

POST和GET都是向服务器提交数据，并且都会从服务器获取数据。

区别：

1、传送方式：get通过地址栏传输，post通过报文传输。

2、传送长度：get参数有长度限制（受限于url长度），而post无限制

3、GET和POST还有一个重大区别，简单的说：

GET产生一个TCP数据包；POST产生两个TCP数据包

长的说：

对于GET方式的请求，浏览器会把http header和data一并发送出去，服务器响应200（返回数据）；

而对于POST，浏览器先发送header，服务器响应100 continue，浏览器再发送data，服务器响应200 ok（返回数据）。

也就是说，GET只需要汽车跑一趟就把货送到了，而POST得跑两趟，第一趟，先去和服务器打个招呼“嗨，我等下要送一批货来，你们打开门迎接我”，然后再回头把货送过去。

1. GET与POST都有自己的语义，不能随便混用。

2. 据研究，在网络环境好的情况下，发一次包的时间和发两次包的时间差别基本可以无视。而在网络环境差的情况下，两次包的TCP在验证数据包完整性上，有非常大的优点。

3. 并不是所有浏览器都会在POST中发送两次包，Firefox就只发送一次。

建议：

1、get方式的安全性较Post方式要差些，包含机密信息的话，建议用Post数据提交方式；

2、在做数据查询时，建议用Get方式；而在做数据添加、修改或删除时，建议用Post方式；

案例：一般情况下，登录的时候都是用的POST传输，涉及到密码传输，而页面查询的时候，如文章id查询文章，用get 地址栏的链接为：article.php?id=11，用post查询地址栏链接为：article.php， 不会将传输的数据展现出来。

拓展资料：

GET在浏览器回退时是无害的，而POST会再次提交请求。

GET产生的URL地址可以被Bookmark，而POST不可以。

GET请求会被浏览器主动cache，而POST不会，除非手动设置。

GET请求只能进行url编码，而POST支持多种编码方式。

GET请求参数会被完整保留在浏览器历史记录里，而POST中的参数不会被保留。

GET请求在URL中传送的参数是有长度限制的，而POST么有。

对参数的数据类型，GET只接受ASCII字符，而POST没有限制。

GET比POST更不安全，因为参数直接暴露在URL上，所以不能用来传递敏感信息。

GET参数通过URL传递，POST放在Request body中。

### 寄生组合继承（比较完美的继承解决方案）

```javascript
function inherit(sub, super) {
    let prototype = Object.create(super.prototype) //创建对象
    prototype.constructor = sub  //增强对象
    sub.prototype = prototype   //赋值对象
}


function Super(name) {
    this.name = name
    this.colors = ['black','red','yellow']
}

Super.prototype.sayName = function () {
    console.log(this.name)
}

function Sub(name,age) {
    Super.call(this, name)
    this.age = age
}

inherit(Sub, Super)  //实现了继承
```

### 事件委托

#### 原理

 利用事件冒泡，只指定一个事件处理程序，就可以管理某一类型的所有事件。 

#### 实现

```html
<ul id="ul1">
    <li>111</li>
    <li>222</li>
    <li>333</li>
    <li>444</li>
</ul>
```

```js
window.onload = function(){		    　　
    var oUl = document.getElementById("ul1");		    　　
    oUl.onclick = function(e){		    	　　　
        let event = e || window.event;		   		　　　
        let target = event.target || event.srcElement;		     
        if(target.nodeName.toLowerCase() == 'li'){		        　 　　　　　　	
            alert(123);		　　　　　　　 
            alert(target.innerHTML);				　　
        }		    
    }		
}
```

#### 优点

1. 减少事件注册，节省内存。比如，
   - 在table上代理所有td的click事件。
   - 在ul上代理所有li的click事件。
2. 简化了dom节点更新时，相应事件的更新。比如
   - 不用在新添加的li上绑定click事件。
   - 当删除某个li时，不用移解绑上面的click事件。

ps: 不支持冒泡的事件

- UI事件 
  - load
  - unload
  - scroll
  - resize
- 焦点事件 
  - blur
  - focus
- 鼠标事件 
  - mouseleave
  - mouseenter

#### 缺点

1. 事件委托基于冒泡，对于不冒泡的事件不支持。
2. 层级过多，冒泡过程中，可能会被某层阻止掉。
3. 理论上委托会导致浏览器频繁调用处理函数，虽然很可能不需要处理。所以建议就近委托，比如在table上代理td，而不是在document上代理td。
4. 把所有事件都用代理就可能会出现事件误判。比如，在document中代理了所有button的click事件，另外的人在引用改js时，可能不知道，造成单击button触发了两个click事件。

### oninput和onchange的区别

oninput是在输入(元素或值发生改变)的时候立刻触发事件，onchange是在失去焦点的时候触发事件

### JS动画和CSS动画的区别

CSS动画

 优点：

- 浏览器可以对动画进行优化。
- 代码相对简单,性能调优方向固定
- 对于帧速表现不好的低版本浏览器，CSS3可以做到自然降级，而JS则需要撰写额外代码

 缺点：

- 运行过程控制较弱,无法附加事件绑定回调函数。CSS动画只能暂停,不能在动画中寻找一个特定的时间点，不能在半路反转动画，不能变换时间尺度，不能在特定的位置添加回调函数或是绑定回放事件,无进度报告
-   代码冗长。想用 CSS 实现稍微复杂一点动画,最后CSS代码都会变得非常笨重。

JS动画

 优点：

- JavaScript动画控制能力很强, 可以在动画播放过程中对动画进行控制：开始、暂停、回放、终止、取消都是可以做到的。
- 动画效果比css3动画丰富,有些动画效果，比如曲线运动,冲击闪烁,视差滚动效果，只有JavaScript动画才能完成
- CSS3有兼容性问题，而JS大多时候没有兼容性问题

缺点：

- JavaScript在浏览器的主线程中运行，而主线程中还有其它需要运行的JavaScript脚本、样式计算、布局、绘制任务等,对其干扰导致线程可能出现阻塞，从而造成丢帧的情况。
- 代码的复杂度高于CSS动画

小结:
 如果动画只是简单的状态切换，不需要中间过程控制，在这种情况下，css动画是优选方案。
 它可以让你将动画逻辑放在样式文件里面，而不会让你的页面充斥 Javascript 库。
 然而如果你在设计很复杂的富客户端界面或者在开发一个有着复杂UI状态的 APP。那么你应该使用js动画，这样你的动画可以保持高效，并且你的工作流也更可控。
 所以，在实现一些小的交互动效的时候，就多考虑考虑CSS动画。对于一些复杂控制的动画，使用JS比较可靠。

### 页面上隐藏元素的方法

##### 占位:

- `visibility: hidden;`
- `margin-left: -100%;`
- `opacity: 0;`
- `transform: scale(0);`

##### 不占位:

- `display: none;`
- `width: 0; height: 0; overflow: hidden;`

仅对块内文本元素:

- `text-indent: -9999px;`
- `font-size: 0;`
