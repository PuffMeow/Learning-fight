## JS解析

#### defer和async的区别

有`derer`的话，加载后续文档元素的过程将和 Js 的加载并行进行（异步），但是Js的执行要在所有元素解析完成之后，`DOMContentLoaded` 事件触发之前完成，并且多个defer会按照顺序进行加载。而`async`会在Js下载完成之后立即执行，哪个先加载好先执行哪个。

#### **不论是内联还是外链js都会阻塞后续dom的解析和渲染**

如果把JavaScript放在页面顶部，下载和解析JavaScript的时间里面，dom迟迟得不到解析和渲染，浏览器一直处于白屏，所以把JavaScript文件放在页面底部更有利于页面快速呈现。

#### Css阻塞渲染

对于一个HTML文档来说，不管是内联还是外链的css，都会阻碍后续的dom渲染，但是不会阻碍后续dom的解析。如果把css文件引用放在HTML文档的底部，浏览器为了防止无样式内容闪烁，会在css文件下载并解析完毕之前什么都不显示，这也就会造成白屏现象。虽然css解析也会阻塞后续dom的渲染，但是在解析css的同时也在解析dom，所以等到css解析完毕就会逐步的渲染页面了。

## document.ready和document.onload的区别

- ready：表示DOM结构已经加载完成（不包含图片等非文字媒体文件）执行先于onload
- onload：指示页面包含图片等文件在内的所有元素都加载完成

## JS代码如何执行

**预解析**：检查语法错误但不生成AST

**生成AST**：经过词法/语法分析，生成抽象语法树

**生成字节码**：基线编译器(Ignition)将AST转换成字节码

**生成机器码**：优化编译器(Turbofan)将字节码转换成优化过的机器码，此外在逐行执行字节码的过程中，如果一段代码经常被执行，那么V8会将这段代码直接转换成机器码保存起来，下一次执行就不必经过字节码，优化了执行速度

## CommonJS、ES module？

- `CommonJS`是服务器端模块的规范，Node.js 采用了这个规范。CommonJS 规范加载模块是同步的，也就是说，只有加载完成，才能执行后面的操作，在浏览器中会出现堵塞的情况。通过对module.exports 或exports的属性赋值来达到暴露模块对象。**CommonJS在运行时加载：即在输入时是先加载整个模块，生成一个对象，然后再从这个对象上面读取方法**
- ES6的导入导出是官方的实现，`export导出，import / import default导入`。**import在编译时加载：ES6 模块不是对象，而是通过 export 命令显式指定输出的代码。在import时可以指定加载某个输出值，而不是加载整个模块**，只有**ES module可以静态分析，实现Tree-shaking**

- require支持动态导入，import不支持，正在提案 (babel下可支持)
- require 同步导入， import属于异步导入
- require 是值拷贝，导出值的变化不会影响导入值； import 指向内存地址，导入值会随导出值而变化

### 从URL输入到页面显示经过了什么

1.在浏览器地址栏输入`URL`

2.浏览器查看缓存，如果请求资源在缓存中并且新鲜，跳转到转码步骤 

- 2.1 如果资源未缓存，发起新请求
- 2.2 如果已缓存，检验是否足够新鲜，足够新鲜直接提供给客户端，否则与服务器进行验证。
- 2.3 检验新鲜通常有两个HTTP头进行控制 `Expires` 和 `Cache-Control`： 
  - 2.3.1 `HTTP1.0`提供`Expires`，值为一个绝对时间表示缓存新鲜日期
  - 2.3.2 `HTTP1.1`增加了`Cache-Control: max-age=`,值为以秒为单位的最大新鲜时间

3.浏览器解析`URL`获取协议，主机，端口，`path`

4.浏览器组装一个`HTTP（GET）`请求报文

5.浏览器获取主机`ip地址`

- 5.1 浏览器缓存
- 5.2 本机缓存
- 5.3 hosts文件
- 5.4 路由器缓存
- 5.5 ISP DNS缓存
- 5.6 DNS递归查询（可能存在负载均衡导致每次IP不一致）

6.打开一个`socket` `IP地址` `三次握手`

- 6.1 客户端发送一个`TCP的SYN=1，Seq=X`的包到服务器端口
- 6.2 服务器发回`SYN=1，ACK=x+1,Seq=Y`的相应包
- 6.3 客户端发送`ACK=Y+1，Seq=z`

7.`TCP`链接建立后发送`HTTP`请求

8.服务器接收请求后解析，将请求转发到服务器程序，如虚拟主机使用`HTTP Host`头部判断请求的服务程序

9.服务器检测`HTTP`请求头是否包含缓存验证信息，如果验证缓存新鲜，返回304等对应状态

10.出合理程序读取完整请求并准备`HTTP`相应，可能需要查询数据库等操作

11.服务器将相应报文通过`TCP`链接发送回浏览器

12.四次挥手

- 12.1 主动方发送`Fin=1,ACK=z,Seq=x`报文
- 12.2 被动方发送`ACK=X+1,Seq=Y`报文
- 12.3 被动方发送`Fin=1,ACK=X,Seq=Y`报文
- 12.4 主动方发送`ACK=Y,Seq=x `报文

13.浏览器检查相应状态码

14.如果资源可缓存，进行缓存

15.对相应进行解码

16.根据资源类型决定如何处理

17.解析`HTML`文档，构建`DOM`树，下载资源，构建`CSSOM`树，执行js脚本，这些操作没有严格的先后顺序

18.构建DOM树： 

- 18.1 `Tokenizing`：语法分析根据HTML规范将字符流解析为标记
- 18.2 `Lexing`：词法分析将标记转换为对象并定义属性和规则
- 18.3 `DOM construction`：根据HTML标记关系将对象组成DOM树

19.解析过程中遇到图片、样式表、js文件，启动下载

20.构建`CSSOM`

- 20.1 `Tokenizing`：字符流转换为标记流
- 20.2 `Node`：根据标记创建节点
- 20.3 `CSSOM`：节点创建CSSOM树

- 21.1 从`DOM树`的根节点遍历所有可见节点，不可见节点包括：1） `script , meta `这样本身不可见的标签。2)被css隐藏的节点，如 display: none
- 21.2 对每一个可见节点，找到恰当的`CSSOM`规则并应用
- 21.3 发布可视节点的内容和计算样式

22.js解析如下 

- 22.1 浏览器创建`Document对象`并解析`HTML`，将解析到的元素和文本节点添加到文档中，此时`document.readystate为loading`
- 22.2 HTML解析器遇到没有`async和defer的script时`，将他们添加到文档中，然后执行行内或外部脚本。这些脚本会同步执行，并且在脚本下载和执行时解析器会暂停。这样就可以用`document.write()`把文本插入到输入流中。同步脚本经常简单定义函数和注册事件处理程序，他们可以遍历和操作script和他们之前的文档内容
- 22.3 当解析器遇到设置了`async属性的script`时，开始下载脚本并继续解析文档。脚本会在它下载完成后尽快执行，但是解析器不会停下来等它下载。异步脚本禁止使用`document.write()`，它们可以访问自己script和之前的文档元素
- 22.4 当文档完成解析，`document.readState变成interactive`
- 22.5 所有`defer脚本`会按照在文档出现的顺序执行，延迟脚本能访问完整文档树，禁止使用`document.write()`
- 22.6 浏览器在`Document`对象上触发`DOMContentLoaded事件`
- 22.7 此时文档完全解析完成，浏览器可能还在等待如图片等内容加载，等这些内容完成载入并且所有异步脚本完成载入和执行，`document.readState变为complete，window触发load事件`

23.CSSOM树和DOM树合成render树，然后渲染，进行复合图层的合成，GPU绘制，这其中还会发生重排和重绘，会影响页面的性能

23.显示页面（HTML解析过程中会逐步显示页面）

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

浏览器在渲染网页时会开启**两条线程**，**渲染引擎线程和JS引擎线程**，但这两条线程是互斥的，同时只能有一个线程在执行。也就是说在构建DOM树时，渲染引擎在执行：

**当遇到JS时**：渲染引擎会停止执行，控制权交给JS引擎，当执行JS代码时
如果遇到获取DOM，那么如果该DOM还没有解析，则会获取为null，如果JS代码还操作了CSS，而这个CSS如果还没有下载和构建，那么浏览器首先会阻塞JS引擎执行，然后会开启一个异步请求线程，在该线程上，去下载构建CSS规则树，CSS规则树构建完成后，再继续执行JS代码，当JS执行完以后，控制权再次交给渲染引擎去执行。

**当遇到CSS元素时**：也会开启异步线程，去下载构建CSS规则树，但同时也会继续构建后面的DOM树，也就是说DOM解析和CSS解析可以同时进行，但如果后面遇到JS元素，则会阻塞JS引擎线程执行，后面DOM树解析不受影响。



## 实现图片懒加载

图片占位符

```javascript
<img src="default.jpg" data-src="http://xxx/target.jpg">
```

### 方案一：使用clientHeight、scrollTop 和 offsetTop

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

### 方案二：getBoundingClientRect

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

### 方案三: IntersectionObserver

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

## 如何判断用户是否能上网

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

## JS如何判断对象中的属性

使用for in 或者Object.keys,Object.getOwnPropertyNames

for in一般配合obj.hasOwnProperty使用

## GET和POST的区别

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

## 寄生组合继承（比较完美的继承解决方案）

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

## 事件委托

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