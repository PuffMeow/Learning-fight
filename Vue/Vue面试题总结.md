### Vue的两个核心特征

- 数据驱动 ： 数据的改变会驱动视图的自动更新。传统的做法是需要手动改变DOM来使得视图更新，而vue只需要改变数据 

- 组件化开发： 可以很好的降低数据之间的耦合度。将常用的代码封装成组件之后，就能高度的复用，提高代码的可重用性。一个页面/模块可以由多个组件所组成。 

### Vue双向绑定原理

#### Vue2

Vue2中实现双向绑定使用的是ES5中的`Object.defineProperty` ，但是它存在一个问题，就是只能检测单个属性，而不能检测整个对象，而且无法对新增对象进行检测，我们还需要对原始数据进行拷贝。而且对于数组中数据的变化无法进行监听。数组原型上的几个方法如`['push', 'pop', 'shift', 'unshift', 'splice','reverse','sort']`都是可以改变数组的，我们就需要去重写实现监听。让我们来看看`Object.defineProperty`最基本的使用代码

```html
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Vue2双向绑定</title>
</head>

<body>
  <input type="text" class="input" />
  <div class="name"></div>
</body>
<script>
  let obj = {
    name: ''
  }

  let newObj = {
    ...obj
  }

  Object.defineProperty(obj, 'name', {
    get() {
      return newObj.name
    },
    set(val) {
      if (val === newObj.name) return
      newObj.name = val
      observe()
    }
  })

  function observe() {
    let nameEl = document.getElementsByClassName('name')[0]
    let inputEl = document.getElementsByClassName('input')[0]
    nameEl.innerHTML = obj.name
    inputEl.value = obj.name
  }
</script>

</html>
```

然后我们在控制台中输入 `obj.name = '王大锤'` ，可以看到页面立马发生了变化，此时我们就已经可以检测到name属性的实时变化了

而`Object.defineProperty`只能检测单个属性name的变化，如果对象里有更多的属性，则我们就需要做遍历和递归处理。

Vue里对数据响应式的处理很复杂，远远不及Vue3的简洁，因为`Object.defineProperty`这个API不能监听整个对象，也不能监听到新增数据和删除数据(然后就有了$set和$delete)，所以达到这个需求就需要做很多处理。

再来瞅一瞅源码是怎么样滴，源码位置在(Vue2.6.12) `src -> core -> observer`

进来`index.js文件`我们可以看到一个观察者对象，用来观察数据变动的

```javascript
export class Observer {
  value: any;
  dep: Dep;
  vmCount: number; 

  constructor (value: any) {
    this.value = value
    this.dep = new Dep()
    this.vmCount = 0
    def(value, '__ob__', this)
    //如果数据是数组的话，我们去做原型链的操作，就是那几个可以改变数组数据的方法
    if (Array.isArray(value)) {
      if (hasProto) {
        protoAugment(value, arrayMethods)
      } else {
        copyAugment(value, arrayMethods, arrayKeys)
      }
      this.observeArray(value)
    } else {
      this.walk(value)
    }
  }
```

在Vue2里，因为`Object.defineProperty`这个API不能监听数组数据的变化，只能监听对象属性的变化，所以我们需要重写数组方法，实现数组数据的监听

下列代码在`src -> core -> observer -> array.js `中，然后我们可以看到，这里对可以改变数组的七个方法进行了重写。

```javascript
const arrayProto = Array.prototype
//这里相当于把数组原型给拷贝了下来放到一个对象里
export const arrayMethods = Object.create(arrayProto)

const methodsToPatch = [
  'push',
  'pop',
  'shift',
  'unshift',
  'splice',
  'sort',
  'reverse'
]

//拦截变异的方法并向外发出事件
methodsToPatch.forEach(function (method) {
  // 对原始的数组方法进行缓存
  const original = arrayProto[method]
  
  //def方法其实是这样的，用来定义一个属性
  //export function def (obj: Object, key: string, val: any, enumerable?: boolean) {
  //Object.defineProperty(obj, key, {
  //  value: val,
  //  enumerable: !!enumerable, 双非运算符可以把属性转成布尔类型
  //  writable: true,
  //  configurable: true
  // })
  //}
  
  def(arrayMethods, method, function mutator (...args) {
    const result = original.apply(this, args)
    const ob = this.__ob__
    //这个变量用来判断数据是不是插入的数据，在Object.defineProperty监测不到属性的增加和删除,就需要做另	 //外的处理
    let inserted
    switch (method) {
      case 'push':
      case 'unshift':
        inserted = args
        break
      case 'splice':
        inserted = args.slice(2)
        break
    }
    if (inserted) ob.observeArray(inserted)
    // 给依赖收集者发出通知有数据进行了变更
    ob.dep.notify()
    return result
  })
})
```

#### Vue3

看完Vue2中数据双向绑定的基本实现，我们再来看一下Vue3中数据的双向绑定。与Vue2不同，Vue3中使用的是ES6中的Proxy做代理，我们先来实现一下和Vue2一样的功能，看看以下代码，效果和Vue2实现完全一样

```html
<!DOCTYPE html>
<html lang="en">

<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>Vue3响应式</title>
</head>

<body>
  <input type="text" class="input" />
  <div class="name"></div>
  <div class="age"></div>
</body>
<script>
  let obj = {
    name: '',
    //这里增加了年龄age
    age: ''
  }

  obj = new Proxy(obj, {
    get(target, prop) {
      return target[prop]
    },
    set(target, prop, val) {
      target[prop] = val
      observe()
    }
  })

  function observe() {
    let nameEl = document.getElementsByClassName('name')[0]
    let inputEl = document.getElementsByClassName('input')[0]
    nameEl.innerHTML = obj.name
    inputEl.value = obj.name
  }

  //打开页面后一秒页面直接显示我永远18
  let ageEl = document.getElementsByClassName('age')[0]
  setTimeout(() => {
    ageEl.innerHTML = '我永远18'
  }, 1000)
</script>

</html>
```

从这里我们可以看到，`Proxy`并不是像`Object.defineProperty`那样只能对对象里的某个属性进行检测，而是对整个对象的属性变化都可以检测到，所以我们并不需要一个个去遍历属性再去进行监听，提高了效率。而且Proxy不会影响到原数据的值，而是返回了一个新的代理对象。

另外，使用`Object.defineProperty`并不能对数组进行检测，对数组的检测需要对数组进行另外的处理，Vue2源码中重写了数组原型上的几个方法实现了对数组的检测，而Proxy则可以直接检测数组变化，不需要做另外的处理。

我们再去看看Vue3中关于响应式的源码，先来看看reactive的源码，(Vue3.0.3)源码位置在`packages -> reactivity -> src -> reactive.ts文件里`，具体的可以自己去看看

```javascript
function createReactiveObject(
  target: Target,
  isReadonly: boolean,
  baseHandlers: ProxyHandler<any>,
  collectionHandlers: ProxyHandler<any>
) {
  //const reactiveMap = new WeakMap<Target, any>()
  //const readonlyMap = new WeakMap<Target, any>()
  const proxyMap = isReadonly ? readonlyMap : reactiveMap
  const existingProxy = proxyMap.get(target)
  //对象已经被代理过的话就直接返回该对象
  if (existingProxy) {
    return existingProxy
  }
  const proxy = new Proxy(
    target,
    targetType === TargetType.COLLECTION ? collectionHandlers : baseHandlers
  )
  //设置过代理之后保存到weakMap里
  proxyMap.set(target, proxy)
  //返回代理过后的对象
  return proxy
}
```

我们可以看到这里就是直接使用Proxy对传入的对象进行相应的处理，然后返回一个代理过的对象，则对象里的数据就是响应式的了

### nextTick的原理

nextTick的作用是为了在数据变化之后等待 Vue 完成更新 DOM ，可以在数据变化之后立即使用 Vue.nextTick(callback)，Js是单线程的，拥有事件循环机制，nextTick的实现就是利用了事件循环的宏任务和微任务。

当我们调用nextTick的时候，传入一个回调函数，这时候会发生以下的步骤：

- 将回调函数传入一个`callbacks`数组中
- 判断采用哪种异步回调方式
  - 首先尝试使用`Promise.then`（微任务）
  - 尝试使用`MuationObserver(微任务)`回调 
  - 尝试使用 `setImmediate(宏任务)`回调 
  - 最后尝试使用`setTimeout(宏任务)`回调 

- 到最后执行 `flushCallbacks()` 方法，遍历`callbacks`数组，依次执行里边的每个函数 

我们再看一眼源码，源码在(Vue2.6.12版本) `src -> core -> util -> next-tick.js`

```javascript
//判断宏任务和微任务的变量
//true为正在使用微任务
export let isUsingMicroTask = false

//存放回调函数的数组
const callbacks = []
//该变量用来设置异步锁
let pending = false

//用来遍历执行回调函数数组里的函数
function flushCallbacks () {
  //执行回调函数时将异步锁给重置
  pending = false
  //防止出现nextTick中包含nextTick时出现问题，浅拷贝callbacks数组之后将其清空
  const copies = callbacks.slice(0)
  callbacks.length = 0
  for (let i = 0; i < copies.length; i++) {
    copies[i]()
  }
}

let timerFunc

if (typeof Promise !== 'undefined' && isNative(Promise)) {
  //判断是否支持Promise
  const p = Promise.resolve()
  timerFunc = () => {
    //执行环境支持Promise则使用Promise.then(微任务)去执行回调函数
    p.then(flushCallbacks)
    if (isIOS) setTimeout(noop)
  }
  //设置为true，正在使用微任务
  isUsingMicroTask = true
} else if (!isIE && typeof MutationObserver !== 'undefined' && (
  isNative(MutationObserver) ||
  MutationObserver.toString() === '[object MutationObserverConstructor]'
)) {
   //当执行环境不支持Promise的时候，我们就做此处的判断
   //如果执行环境不是IE浏览器以及支持MutationObserver这个API的话就执行这里
   //比如在IOS7,Android 4.4的环境下 （IE11中的MutationObserver是不可靠的）
  let counter = 1
  const observer = new MutationObserver(flushCallbacks)
  const textNode = document.createTextNode(String(counter))
  observer.observe(textNode, {
    characterData: true
  })
  timerFunc = () => {
    counter = (counter + 1) % 2
    textNode.data = String(counter)
  }
  isUsingMicroTask = true
} else if (typeof setImmediate !== 'undefined' && isNative(setImmediate)) {
  // 如果Promise和MutationObserver都不支持那就使用setImmediate
  // 该方法优先级依然比setTimeout高，setImmediate属于宏任务
  timerFunc = () => {
    setImmediate(flushCallbacks)
  }
} else {
  //如果上面三种方法都不支持才使用setTimeout，setTimeout也属于宏任务
  timerFunc = () => {
    setTimeout(flushCallbacks, 0)
  }
}

//传入一个回调函数和一个上下文对象
export function nextTick (cb?: Function, ctx?: Object) {
  let _resolve
  callbacks.push(() => {
    if (cb) {
      try {
        cb.call(ctx)
      } catch (e) {
        handleError(e, ctx, 'nextTick')
      }
    } else if (_resolve) {
      _resolve(ctx)
    }
  })
  //如果异步锁没锁的话就锁上异步锁，调用异步函数，准备等同步函数执行完后，就开始执行回调函数队列
  if (!pending) {
    pending = true
    timerFunc()
  }
    
  //当我们不传回调函数的时候，提供一个Promise化的调用
  //相当于可以nextTick().then(() => {})
  if (!cb && typeof Promise !== 'undefined') {
    return new Promise(resolve => {
      _resolve = resolve
    })
  }
}

```

### Vue-router的原理？

我们都知道`vue-router`中有三种模式，分别是

- **abstract: **适用于所有JavaScript环境，用来在不支持浏览器API的环境中，充当`fallback`，而不论是`hash`还是`history`模式都会对浏览器上的url产生作用 

- **hash [参考链接](https://developer.mozilla.org/zh-CN/docs/Web/API/Window/onhashchange):**使用了`onhashchange`事件监听url变化，URL的hash就是带描点#，本质上是改变`window.location`的href属性，hash兼容性比较好，在老版本IE可以运行，但是缺点会带上一个#，看着会有点别扭?(强迫症不能忍)
- **history [参考链接](https://developer.mozilla.org/zh-CN/docs/Web/API/History):**   `history`接口是在HTML5中新增的，这种方式url不会带上#，但是当页面刷新的时候，浏览器还是会向服务器发送请求，所以使用`history`模式要服务器做配合，将所有路由都重定向到根页面( 如果浏览器不支持`history`新特性,则采用`hash`方式 )

我们今天的重点是`hash`和`history`模式，我们来看看实现原理

**hash模式的实现原理：**

```javascript
window.addEventListener('hashchange', () => {
    switch(location.hash) {
        case '#/home':
            // home页面...做一些操作
            break
        case '#/about':
            //about页面...做一些操作
        default:
            //...
    }
})
```

**history模式的实现原理：**

history接口有六种模式可以改变URL而不刷新页面

- **replaceState:** 替换原来的路径
- **pushState:** 使用新路径
- **popState:** 路径回退
- **go:** 向前或向后改变路径
- **forward: **向前改变路径
- **back:** 向后改变路径

```javascript
window.addEventListener('popstate', historyChange)
window.addEventListener('go', historyChange)

function historyChange() {
    switch(location.pathname) {
        case '/home':
            //..做些操作
         	break
        case '/about':
            //..做些操作
            break
        default:
            //...
    }
}
```

该两种方式去改变URL都不会去刷新页面。history模式本质上是用类似于一种栈的结构去存取路由，然后可以push和pop去操纵历史记录。

如果面试官问到我们Vue-router的原理的时候，我们只要能把它们的基本实现方式说出来就差不多了。

### 说说Vue的几个watcher

[computed watcher](https://juejin.cn/post/6844904120290131982)

- 负责敦促视图更新的render-watcher
- 执行敦促计算属性更新的computed-watcher
- 用户注册的普通watcher（watch-api或watch属性）

ComputedWatcher 和普通 Watcher 的区别：
1. 用 lazy 为 true 标示为它是一个计算Watcher
2. 计算Watcher的get和set是在初始化`(initComputed)`时经过` defineComputed() `方法重写了的
3. 当它所依赖的属性发生改变时虽然也会调用`ComputedWatcher.update()`，但是因为它的lazy属性为true，所以只执行把dirty设置为true这一个操作，并不会像其它的Watcher一样执行queueWatcher()或者run()
 4. 当有用到这个`ComputedWatcher`的时候，例如视图渲染时调用了它时，才会触发`ComputedWatcher`的get，但又由于这个get在初始化时被重写了，其内部会判断dirty的值是否为true来决定是否需要执行evaluate()重新计算
5. 因此才有了这么一句话：当计算属性所依赖的属性发生变化时并不会马上重新计算(只是将dirty设置为了true而已)，而是要等到其它地方读取这个计算属性的时候(会触发重写的get)时才重新计算，因此它具备懒计算特性。

### $set和$delete

$set主要是用来避开Vue不能侦听属性被添加的限制。我们为一个data里的对象obj添加了一个新属性name。Vue是得不到通知的，新增的属性也不是响应式。就像我们不能使用`array.length = 0`来清空数组一样

```
data: {
	obj: {}
}

this.obj.name = '大锤'
```

对于数组中元素的添加，是通过`target.splice(key, 1, val)`把val设置到target中的时候，数组拦截器会侦测到target发生了变化，会自动把这个val转成响应式的。

给对象添加一个新的key的时候，使用了`Object.defineReactive`将新增的属性转换成getter/setter形式。最后向target的依赖触发变化通知。

$delete是用来解决`Object.defineReacttive`不能发现数据被删除而更新视图的问题。

对于数组的话，直接使用`target.splice(key, 1)`删除就行，数组拦截器会自动向依赖发送通知。对于对象就使用delete删除，然后发送依赖变化通知就行。

```
const ob = (target).__ob__
delete target[key]
ob.dep.notify()
```

### 虚拟DOM（vnode）

#### 介绍

目前三大主流框架都是声明式操作DOM。通过描述状态和DOM之间的映射关系就可以将状态渲染成视图。由于DOM操作昂贵，频繁操作DOM会消耗很大的性能，状态变化往往只有几个节点发生变化，如果把所有DOM都删掉生成新DOM的话性能开销特别大，比如一个ul下有很多li，其中只有一个li发生了变化，如果用新的ul替换旧的ul，其实除了那个发生了变化的li之外其它的li都不需要渲染，所以只需要找到需要更新的地方，尽可能少的访问DOM就行了。

虚拟DOM就是通过状态生成一个虚拟节点树，然后使用虚拟节点树进行渲染，渲染之前，使用新生成的虚拟节点树和上一次生成的虚拟节点树进行比较，只渲染不同的部分。

虚拟DOM要做的就是找到新旧两个vnode不同的地方，然后根据比对结果操作DOM更新视图。

#### 虚拟DOM的优缺点

优点：

- **保证性能下限:** 虚拟DOM可以经过diff找出最小差异,然后批量进行patch，这种操作虽然比不上手动优化，但是比起粗暴的DOM操作性能要好很多,因此虚拟DOM可以保证性能下限
- **无需手动操作DOM:** 虚拟DOM的diff和patch都是在一次更新中自动进行的，我们无需手动操作DOM，极大提高开发效率
- **跨平台:** 虚拟DOM本质上是JavaScript对象,而DOM与平台强相关,相比之下虚拟DOM可以进行更方便地跨平台操作,例如服务器渲染、移动端开发等等

缺点：

- **无法进行极致优化:** 在一些性能要求极高的应用中虚拟DOM无法进行针对性的极致优化，比如VScode采用直接手动操作DOM的方式进行极端的性能优化

### Patch(Diff算法)

DOM操作速度远远不如JS运算速度快，patch就是比对新旧两个vnode有哪些不同，找出需要更新的节点进行更新。在虚拟DOM上进行操作可以节省性能开销。

#### patch过程

是否有oldVnode，没有就创建vnode插入视图，有就判断oldVnode和node是否同一个节点，是的话就进行更详细的对比和更新，不是的话就使用vnode创建真实节点并插入到视图旧节点旁，然后删除旧节点

#### 静态节点

指的是一旦渲染到页面上后，无论以后状态怎么变化都不会发生变化的节点，比如p标签。遇到静态节点就跳过更新。

### 子组件为啥不能修改父组件传递的Prop

一个父组件不只有一个子组件，使用这份prop的不只你一个子组件，如果每个子组件都可以修改prop的话，拿将导致修改数据的源头不止一处，出现bug难以找到源头。如果要修改prop只能委托父组件去修改，保证数据源唯一。传入的prop如果是基本数据类型，子组件修改父组件的prop会警告，并且不能修改，如果是引用数据类型，那么是可以修改，组件对prop的监听是浅监听。

### Vue的父组件和子组件生命周期执行顺序

加载渲染过程：

父beforeCreate -> 父created -> 父beforeMount -> 子beforeCreate -> 子created -> 子beforeMount -> 子mounted -> 父mounted

更新过程：

父beforeUpdate -> 子beforeUpdate -> 子updated -> 父updated

销毁过程

父beforeDestroy -> 子beforeDestroy -> 子destroyed -> 父destroyed