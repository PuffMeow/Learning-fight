## 学习React

当我们由浅入深地认知一样新事物的时候，往往需要遵循“Why→What→How”这样的一个认知过程。对于学习一个框架也是一样的，我们首先要知道框架是什么，然后知道怎么用，知道怎么用之后就要知道为什么要这样用？设计原理是啥？层层渐进的去学习

## 认识JSX

### JSX 语法是如何在 JavaScript 中生效的：认识 Babel

```
JSX 会被编译为 React.createElement()， React.createElement() 将返回一个叫作“React Element”的 JS 对象。
```

### 什么是 Babel 呢？

```
Babel 是一个工具链，主要用于将 ECMAScript 2015+ 版本的代码转换为向后兼容的 JavaScript 语法，以便能够运行在当前和旧版本的浏览器或其他环境中。
```

JSX 的本质是React.createElement这个 JavaScript 调用的语法糖，这也就完美地呼应上了 React 官方给出的“JSX 充分具备 JavaScript 的能力”这句话。

JSX 语法糖允许前端开发者使用我们最为熟悉的类 HTML 标签语法来创建虚拟 DOM，在降低学习成本的同时，也提升了研发效率与研发体验。

### React创建元素的方法

```js
/**
 101. React的创建元素方法
 */
export function createElement(type, config, children) {
  // propName 变量用于储存后面需要用到的元素属性
  let propName; 
  // props 变量用于储存元素属性的键值对集合
  const props = {}; 
  // key、ref、self、source 均为 React 元素的属性，此处不必深究
  let key = null;
  let ref = null; 
  let self = null; 
  let source = null; 
  // config 对象中存储的是元素的属性
  if (config != null) { 
    // 进来之后做的第一件事，是依次对 ref、key、self 和 source 属性赋值
    if (hasValidRef(config)) {
      ref = config.ref;
    }
    // 此处将 key 值字符串化
    if (hasValidKey(config)) {
      key = '' + config.key; 
    }
    self = config.__self === undefined ? null : config.__self;
    source = config.__source === undefined ? null : config.__source;
    // 接着就是要把 config 里面的属性都一个一个挪到 props 这个之前声明好的对象里面
    for (propName in config) {
      if (
        // 筛选出可以提进 props 对象里的属性
        hasOwnProperty.call(config, propName) &&
        !RESERVED_PROPS.hasOwnProperty(propName) 
      ) {
        props[propName] = config[propName]; 
      }
    }
  }
  // childrenLength 指的是当前元素的子元素的个数，减去的 2 是 type 和 config 两个参数占用的长度
  const childrenLength = arguments.length - 2; 
  // 如果抛去type和config，就只剩下一个参数，一般意味着文本节点出现了
  if (childrenLength === 1) { 
    // 直接把这个参数的值赋给props.children
    props.children = children; 
    // 处理嵌套多个子元素的情况
  } else if (childrenLength > 1) { 
    // 声明一个子元素数组
    const childArray = Array(childrenLength); 
    // 把子元素推进数组里
    for (let i = 0; i < childrenLength; i++) { 
      childArray[i] = arguments[i + 2];
    }
    // 最后把这个数组赋值给props.children
    props.children = childArray; 
  } 
  // 处理 defaultProps
  if (type && type.defaultProps) {
    const defaultProps = type.defaultProps;
    for (propName in defaultProps) { 
      if (props[propName] === undefined) {
        props[propName] = defaultProps[propName];
      }
    }
  }
  // 最后返回一个调用ReactElement执行方法，并传入刚才处理过的参数
  return ReactElement(
    type,
    key,
    ref,
    self,
    source,
    ReactCurrentOwner.current,
    props,
  );
}
```

- type：用于标识节点的类型。它可以是类似“h1”“div”这样的标准 HTML 标签字符串，也可以是 React 组件类型或 React fragment 类型。
- config：以对象形式传入，组件所有的属性都会以键值对的形式存储在 config 对象中。
- children：以对象形式传入，它记录的是组件标签之间嵌套的内容，也就是所谓的“子节点”“子元素”。

### 出参 ReactElement

```js
const ReactElement = function(type, key, ref, self, source, owner, props) {
  const element = {
    // REACT_ELEMENT_TYPE是一个常量，用来标识该对象是一个ReactElement
    $$typeof: REACT_ELEMENT_TYPE,
    // 内置属性赋值
    type: type,
    key: key,
    ref: ref,
    props: props,
    // 记录创造该元素的组件
    _owner: owner,
  };
   
  if (__DEV__) {
    // 这里是一些针对 __DEV__ 环境下的处理，对于大家理解主要逻辑意义不大，此处我直接省略掉，以免混淆视听
  }
  return element;
};
```

这个 ReactElement 对象实例，本质上是**以 JavaScript 对象形式存在的对 DOM 的描述**，也就是老生常谈的“虚拟 DOM”（**准确地说，是虚拟 DOM 中的一个节点**）

既然是“虚拟 DOM”，那就意味着和渲染到页面上的真实 DOM 之间还有一些距离，这个“距离”，就是由大家喜闻乐见的**ReactDOM.render**方法来填补的。

### render

```js
ReactDOM.render(
    // 需要渲染的元素（ReactElement）
    element, 
    // 元素挂载的目标容器（一个真实DOM）
    container,
    // 回调函数，可选参数，可以用来处理渲染结束后的逻辑
    [callback]
)

//调用例子
const rootElement = document.getElementById("root");
ReactDOM.render(<App />, rootElement);
```

## 生命周期

### 虚拟DOM工作流程

组件在初始化时，会通过调用生命周期中的 render 方法，**生成虚拟 DOM**，然后再通过调用` ReactDOM.render `方法，实现虚拟 DOM 到真实 DOM 的转换。

当组件更新时，会再次通过调用 render 方法**生成新的虚拟 DOM**，然后借助 diff（这是一个非常关键的算法，我将在“模块二：核心原理”重点讲解）**定位出两次虚拟 DOM 的差异**，从而针对发生变化的真实 DOM 作定向更新。

`render`方法比作组件的“**灵魂**”，render 之外的生命周期方法就完全可以理解为是组件的“**躯干**”。

### React15生命周期

```js
constructor()
componentWillReceiveProps()
shouldComponentUpdate()
componentWillMount()
componentWillUpdate()
componentDidUpdate()
componentDidMount()
render()
componentWillUnmount()
```

挂载阶段：

`constructor() -> componentWillMount() -> render() -> componentDidMount()`

constructor只在挂载的时候会调用一次，在这里可以对state进行数据初始化

```js
constructor(props) {
  console.log("进入constructor");
  super(props);
  // state 可以在 constructor 里初始化
  this.state = { msg: "Hello world" };
}
```

`componentWillMount`、`componentDidMount` 方法同样只会在挂载阶段被调用一次。其中 `componentWillMount` 会在执行 render 方法前被触发。接下来 render 方法被触发。注意 render 在执行过程中并不会去操作真实 DOM（也就是说不会渲染），它的职能是**把需要渲染的内容返回出来**。真实 DOM 的渲染工作，在挂载阶段是由 `ReactDOM.render` 来承接的。

更新阶段：

`componentWillReceiveProps() -> shouldComponentUpdate() -> componentWillUpdate() -> render() -> componentDidUpdate()`

在这个生命周期方法里，nextProps 表示的是接收到新 props 内容，而现有的 props （相对于 nextProps 的“旧 props”）我们可以通过 this.props 拿到，由此便能够感知到 props 的变化。**componentReceiveProps 并不是由 props 的变化触发的，而是由父组件的更新触发的**

`componentWillUpdate` 会在 render 前被触发，它和 `componentWillMount` 类似，允许你在里面做一些不涉及真实 DOM 操作的准备工作；而 `componentDidUpdate` 则在组件更新完毕后被触发，和 `componentDidMount` 类似，这个生命周期也经常被用来处理 DOM 操作。此外，我们也常常将 `componentDidUpdate` 的执行作为子组件更新完毕的标志通知到父组件

关于`shouldComponentUpdate(nextProps, nextState)`

React 组件会根据 `shouldComponentUpdate` 的返回值，来决定是否执行该方法之后的生命周期，进而决定是否对组件进行**re-render**（重渲染）。`shouldComponentUpdate` 的默认值为 true，也就是说“无条件 re-render”。在实际的开发中，我们往往通过手动往 `shouldComponentUpdate` 中填充判定逻辑，或者直接在项目中引入 `PureComponent` 等最佳实践，来实现“有条件的 re-render”。

卸载阶段：

组件销毁的常见原因有以下两个

- 组件在父组件中被移除了
- 组件中设置了 key 属性，父组件在 render 的过程中，发现 key 值和上一次不一致，那么这个组件就会被干掉

### React16生命周期

### 生命周期流程图

![Drawing 0.png](https://s0.lgstatic.com/i/image/M00/5D/D9/CgqCHl-FVVeAaMJvAAKXOyLlUwM592.png)

### 对比

![图片1.png](https://s0.lgstatic.com/i/image/M00/5F/B0/Ciqc1F-Klv6AIeOPAADAZZgLu7U105.png)

### getDerivedStateFromProps

```js
static getDerivedStateFromProps(props, state) {
  //记得return 一个东西，不需要return什么的时候就return null
  return {
    
  }
}
```

React 15 生命周期和 React 16.3 生命周期在挂载阶段的主要差异在于，**废弃了` componentWillMount`，新增了 `getDerivedStateFromProps`** 。 这个 API设计的初衷不是试图替换掉 **`componentWillMount`**，而是试图替换掉 **`componentWillReceiveProps`**，因此它有且仅有一个用途：**使用 props 来派生/更新 state**。`getDerivedStateFromProps `需要一个对象格式的返回值。`getDerivedStateFromProps` 这个 API，它相对于早期的` componentWillReceiveProps `来说，正是做了“**合理的减法**”。而做这个减法的决心之强烈，从 `getDerivedStateFromProps `直接被定义为 static 方法这件事上就可见一斑—— static 方法内部**拿不到组件实例的 this**，这就导致你无法在` getDerivedStateFromProps `里面做任何类似于 `this.fetch()`、不合理的 `this.setState`（会导致死循环的那种）这类可能会产生副作用的操作。

###  getSnapshotBeforeUpdate

```js
getSnapshotBeforeUpdate(prevProps, prevState) {
  // ...
}
```

和 `componentDidUpdate `间的通信过程

```js
// 组件更新时调用
getSnapshotBeforeUpdate(prevProps, prevState) {
  console.log("getSnapshotBeforeUpdate方法执行");
  return "haha";
}

// 组件更新后调用
componentDidUpdate(prevProps, prevState, valueFromSnapshot) {
  console.log("componentDidUpdate方法执行");
  console.log("从 getSnapshotBeforeUpdate 获取到的值是", valueFromSnapshot);
}
```

这个生命周期的设计初衷，是为了“与 componentDidUpdate 一起，涵盖过时的 componentWillUpdate 的所有用例。

### Fiber架构

**Fiber 会使原本同步的渲染过程变成异步的**。**Fiber 会将一个大的更新任务拆解为许多个小任务**。每当执行完一个小任务时，**渲染线程都会把主线程交回去**，看看有没有优先级更高的工作要处理，确保不会出现其他任务被“饿死”的情况，进而避免同步渲染带来的卡顿。

Fiber特性:

- 任务拆解
- 可打断
- 异步渲染

我们再看React16的生命周期流程图，其被划分为了 render 和 commit 两个阶段，而 commit 阶段又被细分为了 pre-commit 和 commit。

- render 阶段：纯净且没有副作用，可能会被 React 暂停、终止或重新启动。
- pre-commit 阶段：可以读取 DOM。
- commit 阶段：可以使用 DOM，运行副作用，安排更新。涉及到真实dom的渲染，必须以同步方式来求稳。

## 组件

组件，从概念上类似JS函数，接受任何入参（props），并返回用于描述页面展示内容的React元素。React 的核心特征是“**数据驱动视图**”。

### 基于 props 的单向数据流

所谓单向数据流，指的就是当前组件的 state 以 props 的形式流动时，**只能流向组件树中比自己层级更低的组件。** 比如在父-子组件这种嵌套关系中，只能由父组件传 props 给子组件，而不能反过来。了解组件我们首先得知道组件之间的通信。

- 父子通信 -> 将属性通过props传入到子组件

- 子父通信 -> 将函数通过props传入子组件，子组件执行该函数并将参数传递，在父组件的函数就可以拿到子组件传递过来的参数。

- 兄弟组件通信 -> 依赖于父组件做中转，兄弟1将数据传递到父组件，父组件再将数据传递到子组件

- 深层级嵌套传递数据 / 其他类型的数据传递-> 使用发布订阅模式 EventEmitter(event-bus也称为事件总线)，在一个组件中发布的事件，在另一个组件可以很轻松的监听到，可以实现任何组件之间的数据流动

   ```js
  //实现代码
  ```

- 顶层注入数据，全局通信模式 -> Context API 是 React 官方提供的一种组件树全局通信的方式。Context API 有 3 个关键的要素：React.createContext、Provider、Consumer。可以跨越层级进行传递数据。

  ```js
  const defaultValue = {} //可以设置默认参数
  const AppContext = React.createContext(defaultValue)
  
  //从创建出的 context 对象中，我们可以读取到 Provider(提供者)和 Consumer(消费者)
  const { Provider, Consumer } = AppContext
  
  //使用方式，在顶层注入传递的数据
  <Provider value={title: this.state.title, content: this.state.content}>
    <Title />
    <Content />
  </Provider>
  
  //在子组件中通过消费者使用传递入的数据
  <Consumer>
    {value => <div>{value.title}</div>}
  </Consumer>
  ```

- Redux -> 一个全局状态管理的容器(存放公共数据的仓库)，每一个react组件都可以从这里拿数据。**在 Redux 的整个工作过程中，数据流是严格单向的（重点）**。

  概念：store、reducer 和 action。

  - store就是存放数据的仓库了，表示一个单一数据源，是只读的。

    ```js
    // 使用 dispatch 派发 action，action 会进入到reducer里触发对应的更新
    store.dispatch(action)
    ```

  - action就是对变化的描述，表示一个动作。用来通知reducer，让改变发生。

    ```js
    const action = {
      type: "ADD_ITEM",
      payload: '<li>text</li>'
    }
    ```

  - reducer 是一个函数，它负责**对变化进行分发和处理，** 最终将新的数据返回给 store。

    ```js
    const reducer = (state, action) => {
        // 此处是各种样的 state处理逻辑
        return newState
    }
    ```

  Redux的基本使用：

  ```js
  // 引入 redux
  import { createStore } from 'redux'
  // 创建 store
  const store = createStore(
      reducer,
      initialState, //初始化数据
      applyMiddleware(middleware1, middleware2, ...) //中间件
  );
  ```

  

![图片7.png](https://s0.lgstatic.com/i/image/M00/81/9F/CgqCHl_Rii2AVvUbAADn4s_6rB8369.png)

## 了解Hooks

早期React有类组件和函数式组件，类组件可以维护自己内部的状态，但是函数组件在Hooks没有出来之前是没法在内部维护状态的。**Hooks 的本质：一套能够使函数组件更强大、更灵活的“钩子”**

比较类组件和函数组件：

- 类组件需要继承 class，函数组件不需要；
- 类组件可以访问生命周期方法，函数组件不能；
- 类组件中可以获取到实例化后的 this，并基于这个 this 做各种各样的事情，而函数组件不可以；
- 类组件中可以定义并维护 state（状态），而函数组件不可以；

看一个公式：UI = f(data) ，所以说**React 组件本身的定位就是函数，一个吃进数据、吐出 UI 的函数**。 相比于函数组件来说，类组件实在是太沉重了，而函数组件是比较轻量的，也不需要维护内部的this状态，减少了开发人员的心智负担。

### useState

useState()：为函数组件引入状态

```js
const [state, setState] = useState(initialState);
```

这里面的initialState可以是对象，数组，字符串，布尔型...，一个state对应一个属性，允许存任何类型的值。

对比一下class组件和函数式组件实现一样的效果

类组件

```js
import React, { Component } from "react";
export default class TextButton extends Component {
  constructor(props) {
    super(props);
    this.state = {
      msg: "Hello world"
    };
  }

  changeMsg = () => {
    this.setState({msg: "Hello 王大锤"})
  };

  render() {
    const { msg } = this.state;
    return (
      <div className="button">
        <p>{text}</p>
        <button onClick={this.changeMsg}>点击修改</button>
      </div>
    );
  }
}
```

函数式组件

```js
import React, { useState } from "react";
export default function Button() {
  const [msg, setMsg] = useState("Hello world");
  
  function changeMsg() {
    return setMsg("Hello 王大锤");
  }
  
  return (
    <div className="button">
      <p>{text}</p>
      <button onClick={changeMsg}>点击修改</button>
    </div>
  );
}
```

**同样逻辑的函数组件相比类组件而言，复杂度要低得多得多**。所以我们为什么说类组件太沉重了。

### useEffect

useEffect()：允许函数组件执行副作用操作。弥补了函数式组件中没有生命周期的缺陷。在类组件中的`componentDidMount`、`componentDidUpdate` 和 `componentWillUnmount` 三个生命周期里做的事，现在可以放在 useEffect 里来做，比如操作 DOM、订阅事件、调用外部 API 获取数据等。

useEffect 可以接收两个参数，分别是回调函数与依赖数组

```
useEffect(callBack, [])
```

看看用法：

如果不传第二个参数，就会在每次页面渲染时都去执行useEffect，然后打印111

```js
const [state, setState] = useState(111)

useEffect(() => {
  console.log(state)
})
```

如果第二个参数传一个空数组，则只会在初次挂载时执行且仅执行一次，也就是说以后去setState的时候也不会再去触发useEffect。

```js
const [state, setState] = useState(111)

useEffect(() => {
  console.log(state)
}, [])
```

**如果第二个参数传递一个在useEffect里使用到了的数据，那么每当这个数据发生变更的时候，都会重新执行一次useEffect。**React 就会在新的一次渲染后去对比前后两次的渲染，查看数组内是否有变量发生了更新（如果数组传递了多个数据，那么里面只要有一个数组元素变了，就会被认为更新发生了），并在有更新的前提下去触发 useEffect 中定义的逻辑。

```js
const [state, setState] = useState(111)

useEffect(() => {
  console.log(state)
}, [state, ...])
```

useEffect返回一个函数，这个函数会在组件卸载阶段去触发，可以做一些收尾的操作，例如清除定时器。类似于`componentWillUnmount `方法

```js
const [state, setState] = useState(111)

useEffect(() => {
  const timer = setTimeout(() => {console.log(1)}, 1000)
  
  return () => {
    clearTimeout(timer)
  }
}, [])
```

React Hooks解决的问题

- 告别难以理解的 Class，主要是this和生命周期
- 解决业务逻辑难以拆分的问题，比如在`componentDidMount`中做了一大堆操作以及订阅数据，然后在`componentWillUnmout`中卸载订阅，这里的逻辑其实是有关联的（订阅和取消订阅），但是却被拆分到了两个不同的生命周期函数中，变得难以维护（Vue2中的Options api也存在这个问题，所以Vue3用了Composition api去解决这个问题)。
- 使状态逻辑复用变得简单可行，在类组件中进行逻辑复用，靠的是 HOC（高阶组件）和 Render Props 这些组件设计模式，而我们在hooks中可以使用自定义hooks
- 函数组件从设计思想上来看，更加契合 React 的理念。

Hooks也有局限性：

- 使用层面拥有严格的约束规则，只在react函数中使用Hooks，不要在循环、条件、嵌套函数中调用Hooks，这是为了确保Hooks在每次渲染时都保持同样的执行顺序
- 函数组件对开发者水平要求更高

## 深入Hooks

React底层是一个顺序链表，我们得了解一下Hooks的执行顺序。

useState的执行过程： `useState -> 通过resolveDispatcher获取dispatcher -> 调用dispatcher.useState -> 调用updateState(更新阶段) -> 调用mountState -> 返回目标数组 [state, useState]`

```js
// 进入 mounState 逻辑
function mountState(initialState) {

  // 将新的 hook 对象追加进链表尾部
  var hook = mountWorkInProgressHook();

  // initialState 可以是一个回调，若是回调，则取回调执行后的值
  if (typeof initialState === 'function') {
    // $FlowFixMe: Flow doesn't like mixed types
    initialState = initialState();
  }

  // 创建当前 hook 对象的更新队列，这一步主要是为了能够依序保留 dispatch
  const queue = hook.queue = {
    last: null,
    dispatch: null,
    lastRenderedReducer: basicStateReducer,
    lastRenderedState: (initialState: any),
  };

  // 将 initialState 作为一个“记忆值”存下来
  hook.memoizedState = hook.baseState = initialState;

  // dispatch 是由上下文中一个叫 dispatchAction 的方法创建的，这里不必纠结这个方法具体做了什么
  var dispatch = queue.dispatch = dispatchAction.bind(null, currentlyRenderingFiber$1, queue);
  // 返回目标数组，dispatch 其实就是示例中常常见到的 setXXX 这个函数，想不到吧？哈哈
  return [hook.memoizedState, dispatch];
}
```

mountState主要作用就是初始化Hooks，而里面的`mountWorkInProgressHook `方法，我们可以看到这是一个单向链表

```js
function mountWorkInProgressHook() {
  //单个 hook 是以对象的形式存在的
  var hook = {
    memoizedState: null,
    baseState: null,
    baseQueue: null,
    queue: null,
    next: null
  };
  if (workInProgressHook === null) {
    // 这行代码每个 React 版本不太一样，但做的都是同一件事：将 hook 作为链表的头节点处理
    firstWorkInProgressHook = workInProgressHook = hook;
  } else {
    // 若链表不为空，则将 hook 追加到链表尾部
    workInProgressHook = workInProgressHook.next = hook;
  }
  // 返回当前的 hook
  return workInProgressHook;
}
```

在更新时，会多一个updateState的过程。这个过程主要是**按顺序去遍历之前构建好的链表，取出对应的数据信息进行渲染**。挂载和渲染时执行的操作：mountState（首次渲染）构建链表并渲染；updateState 依次遍历链表并渲染。

## 虚拟DOM

虚拟 DOM（Virtual DOM）本质上是**JS 和 DOM 之间的一个映射缓存**，其实就是一个可以描述DOM 结构和属性的 **JS 对象**。虚拟 DOM 并不一定会带来更好的性能，React 官方也从来没有把虚拟 DOM 作为性能层面的卖点对外输出过。虚拟 DOM 的优越之处在于它能够在提供更高效的研发体验（也就是函数式的 UI 编程方式）的同时，仍然保持一个还不错的性能。虚拟 DOM 不一定比自己手动操作真实DOM要快，它只是保证了我们的性能下限，同时给我们提供了很好的开发体验和效率。

- **挂载阶段：**结合 JSX 的描述，构建出虚拟 DOM 树，然后通过 ReactDOM.render 实现虚拟 DOM 到真实 DOM 的映射（触发渲染流水线）
- **更新阶段**：页面的变化在作用于真实 DOM 之前，会先在 JS 层借助diff算法先对比出具体有哪些真实 DOM 需要被改变，然后再将这些改变作用于真实 DOM

虚拟DOM解决了什么问题呢？

- 开发效率／体验的问题
- 跨平台的问题（对真实渲染内容进行抽象）

## 了解Diff

调和（Reconciliation）过程与 Diff 算法。调和指的是将虚拟 DOM映射到真实 DOM 的过程。传统的计算方法是通过循环递归进行树节点的一一对比,  这个过程的算法复杂度是 O (n3)。DOM 节点之间的跨层级操作并不多，同层级操作是主流。

- Diff 算法性能突破的关键点在于“分层对比”；

- 类型一致的节点才有继续 Diff 的必要性；

- key 属性的设置，可以帮我们尽可能重用同一层级内的节点。

![Drawing 1.png](https://s0.lgstatic.com/i/image/M00/6C/14/Ciqc1F-qYhGAEPpKAAEByai_5tk134.png)

React做的只是根据两棵DOM树的层级进行从第一层到第二层到第三层进行层层比较，这样时间复杂度就从O(n3)变成了O(n)。React 认为，只有同类型的组件，才有进一步对比的必要性；若参与 Diff 的两个组件类型不同，那么直接放弃比较，原地替换掉旧的节点。

key属性：

用来帮助React记住节点，以便于更好的去复用节点，节省开销。key 是用来帮助 React 识别哪些内容被更改、添加或者删除。key 需要写在用数组渲染出来的元素内部，并且需要赋予其一个稳定的值。稳定在这里很重要，因为如果 key 值发生了变更，React 则会触发 UI 的重渲染。

![Drawing 9.png](https://s0.lgstatic.com/i/image/M00/6C/15/Ciqc1F-qYkOANYXaAAC2tCBcU4k280.png)

虚拟 DOM 中还有一个叫作“batch”的东西。“batch”描述的是“批处理”机制，这个机制和 Diff 一样，在 React 中都可以由 setState 来触发

## 了解setState

我们在这主要看的是React15。在React16中已经改成了Fiber架构，对state的修改全都是异步的行为了，在16ms的刷新周期里有剩余时间才会被执行。

```js
import React from "react";
import "./styles.css";
export default class App extends React.Component{
  state = {
    count: 0
  }
  increment = () => {
    console.log('increment setState前的count', this.state.count)
    this.setState({
      count: this.state.count + 1
    });
    console.log('increment setState后的count', this.state.count)
  }
  triple = () => {
    console.log('triple setState前的count', this.state.count)
    this.setState({
      count: this.state.count + 1
    });
    this.setState({
      count: this.state.count + 1
    });
    this.setState({
      count: this.state.count + 1
    });
    console.log('triple setState后的count', this.state.count)
  }
  reduce = () => {
    setTimeout(() => {
      console.log('reduce setState前的count', this.state.count)
      this.setState({
        count: this.state.count - 1
      });
      console.log('reduce setState后的count', this.state.count)
    },0);
  }
  render(){
    return <div>
      <button onClick={this.increment}>点我增加</button>
      <button onClick={this.triple}>点我增加三倍</button>
      <button onClick={this.reduce}>点我减少</button>
    </div>
  }
}
```

打印结果

```js
increment setState 前的count 0
increment setState 后的count 0
(异步)

triple setState 前的count 1
triple setState 后的count 1
(异步)

reduce setState 前的count 2
reduce setState 后的count 1
(同步)
```

setState设置成异步的动机：避免频繁触发视图更新

在实际的 React 运行时中，setState 异步的实现方式有点类似于 Vue 的 $nextTick 和浏览器里的 Event-Loop：每来一个 setState，就把它塞进一个队列里“攒起来”。等时机成熟，再把“攒起来”的 state 结果做合并，最后只针对最新的 state 值走一次更新流程。这个过程，叫作“批量更新”。

所以 triple里面发生的过程其实是这样的

```js
this.setState({
  count: this.state.count + 1    ===>    入队，[count+1的任务]
});
this.setState({
  count: this.state.count + 1    ===>    入队，[count+1的任务，count+1的任务]
});
this.setState({
  count: this.state.count + 1    ===>    入队, [count+1的任务，count+1的任务, count+1的任务]
});
                                          ↓
                                         合并 state，[count+1的任务]
                                          ↓
                                         执行 count+1的任务
```

然而为什么在setTimeout里的setState却是同步的呢？？？

其实并不是 setTimeout 改变了 setState，而是 setTimeout 帮助 setState “逃脱”了 React 对它的管控。只要是在 React 管控下的 setState，一定是异步的。看一张setState的流程图：

![3.png](https://s0.lgstatic.com/i/image2/M01/04/81/Cip5yF_yswuAWzDfAAEc1lISh-Q211.png)

```js
ReactComponent.prototype.setState = function (partialState, callback) {
  this.updater.enqueueSetState(this, partialState);
  if (callback) {
    this.updater.enqueueCallback(this, callback, 'setState');
  }
};
```

```js
enqueueSetState: function (publicInstance, partialState) {
  // 根据 this 拿到对应的组件实例
  var internalInstance = getInternalInstanceReadyForUpdate(publicInstance, 'setState');
  // 这个 queue 对应的就是一个组件实例的 state 数组
  var queue = internalInstance._pendingStateQueue || (internalInstance._pendingStateQueue = []);
  queue.push(partialState);
  //  enqueueUpdate 用来处理当前的组件实例
  enqueueUpdate(internalInstance);
}
```

enqueueSetState 做了两件事：

- 将新的 state 放进组件的状态队列里；

- 用 enqueueUpdate 来处理将要更新的实例对象。

```js
function enqueueUpdate(component) {
  ensureInjected();
  // 注意这一句是问题的关键，isBatchingUpdates标识着当前是否处于批量创建/更新组件的阶段
  if (!batchingStrategy.isBatchingUpdates) {
    // 若当前没有处于批量创建/更新组件的阶段，则立即更新组件
    batchingStrategy.batchedUpdates(enqueueUpdate, component);
    return;
  }
  // 否则，先把组件塞入 dirtyComponents 队列里，让它“再等等”
  dirtyComponents.push(component);
  if (component._updateBatchNumber == null) {
    component._updateBatchNumber = updateBatchNumber + 1;
  }
}
```

batchingStrategy 是 React 内部专门用于管控批量更新的对象。

```js
/**
 * batchingStrategy源码
**/
var ReactDefaultBatchingStrategy = {
  // 全局唯一的锁标识
  isBatchingUpdates: false,
  // 发起更新动作的方法
  batchedUpdates: function(callback, a, b, c, d, e) {
    // 缓存锁变量
    var alreadyBatchingStrategy = ReactDefaultBatchingStrategy. isBatchingUpdates
    // 把锁“锁上”
    ReactDefaultBatchingStrategy.isBatchingUpdates = true
    if (alreadyBatchingStrategy) {
      callback(a, b, c, d, e)
    } else {
      // 启动事务，将 callback 放进事务里执行
      transaction.perform(callback, null, a, b, c, d, e)
    }
  }
}
```

batchingStrategy 对象可以理解为它是一个“锁管理器”。

这里的“锁”，是指 React 全局唯一的 `isBatchingUpdates` 变量，`isBatchingUpdates` 的初始值是 false，意味着“当前并未进行任何批量更新操作”。每当 React 调用` batchedUpdate` 去执行更新动作时，会先把这个锁给“锁上”（置为 true），表明“现在正处于批量更新过程中”。当锁被“锁上”的时候，任何需要更新的组件都只能暂时进入 `dirtyComponents` 里排队等候下一次的批量更新，而不能随意“插队”。此处体现的“任务锁”的思想，是 React 面对大量状态仍然能够实现有序分批处理的基石。

transaction事务的理解：

```js
* <pre>
 *                       wrappers (injected at creation time)
 *                                      +        +
 *                                      |        |
 *                    +-----------------|--------|--------------+
 *                    |                 v        |              |
 *                    |      +---------------+   |              |
 *                    |   +--|    wrapper1   |---|----+         |
 *                    |   |  +---------------+   v    |         |
 *                    |   |          +-------------+  |         |
 *                    |   |     +----|   wrapper2  |--------+   |
 *                    |   |     |    +-------------+  |     |   |
 *                    |   |     |                     |     |   |
 *                    |   v     v                     v     v   | wrapper
 *                    | +---+ +---+   +---------+   +---+ +---+ | invariants
 * perform(anyMethod) | |   | |   |   |         |   |   | |   | | maintained
 * +----------------->|-|---|-|---|-->|anyMethod|---|---|-|---|-|-------->
 *                    | |   | |   |   |         |   |   | |   | |
 *                    | |   | |   |   |         |   |   | |   | |
 *                    | |   | |   |   |         |   |   | |   | |
 *                    | +---+ +---+   +---------+   +---+ +---+ |
 *                    |  initialize                    close    |
 *                    +-----------------------------------------+
 * </pre>
```

Transaction 就像是一个“壳子”，它首先会将目标函数用 wrapper（一组 initialize 及 close 方法称为一个 wrapper） 封装起来，同时需要使用 Transaction 类暴露的 perform 方法去执行它。如上面的注释所示，在 anyMethod 执行之前，perform 会先执行所有 wrapper 的 initialize 方法，执行完后，再执行所有 wrapper 的 close 方法。这就是 React 中的事务机制。

同步现象的本质：

对于ReactDefaultBatchingStrategy 这个对象。`ReactDefaultBatchingStrategy `其实就是一个批量更新策略事务，它的 wrapper 有两个：`FLUSH_BATCHED_UPDATES `和` RESET_BATCHED_UPDATES`。

```js
var RESET_BATCHED_UPDATES = {
  initialize: emptyFunction,
  close: function () {
    ReactDefaultBatchingStrategy.isBatchingUpdates = false;
  }
};
var FLUSH_BATCHED_UPDATES = {
  initialize: emptyFunction,
  close: ReactUpdates.flushBatchedUpdates.bind(ReactUpdates)
};
var TRANSACTION_WRAPPERS = [FLUSH_BATCHED_UPDATES, RESET_BATCHED_UPDATES];
```

在callback执行完之后，`RESET_BATCHED_UPDATES`将isBatchingUpdates设置为false，FLUSH_BATCHED_UPDATES执行flushBatchedUpdates，然后里面会循环所有dirtyComponent，调用updateComponent来执行所有生命周期方法。最后再实现组件的更新。

```js
// ReactMount.js
_renderNewRootComponent: function( nextElement, container, shouldReuseMarkup, context ) {
  // 实例化组件
  var componentInstance = instantiateReactComponent(nextElement);
  // 初始渲染直接调用 batchedUpdates 进行同步渲染
  ReactUpdates.batchedUpdates(
    batchedMountComponentIntoNode,
    componentInstance,
    container,
    shouldReuseMarkup,
    context
  );
  ...
}
```

这段代码是在首次渲染组件时会执行的一个方法，我们看到它内部调用了一次 batchedUpdates，这是因为在组件的渲染过程中，会按照顺序调用各个生命周期函数。开发者很有可能在声明周期函数中调用 setState。因此，我们需要通过开启 batch 来确保所有的更新都能够进入 dirtyComponents 里去，进而确保初始渲染流程中所有的 setState 都是生效的。



下面代码是 React 事件系统的一部分。当我们在组件上绑定了事件之后，事件中也有可能会触发 setState。为了确保每一次 setState 都有效，React 同样会在此处手动开启批量更新。

```js
// ReactEventListener.js
dispatchEvent: function (topLevelType, nativeEvent) {
  ...
  try {
    // 处理事件
    ReactUpdates.batchedUpdates(handleTopLevelImpl, bookKeeping);
  } finally {
    TopLevelCallbackBookKeeping.release(bookKeeping);
  }
}
```

isBatchingUpdates 这个变量，在 React 的生命周期函数以及合成事件执行前，已经被 React 悄悄修改为了 true，这时我们所做的 setState 操作自然不会立即生效。当函数执行完毕后，事务的 close 方法会再把 isBatchingUpdates 改为 false。

再看之前的代码，那么问题我们就能看出来。在 isBatchingUpdates 的约束下，setState 只能是异步的。

```js
increment = () => {
  // 进来先锁上
  isBatchingUpdates = true
  console.log('increment setState前的count', this.state.count)
  this.setState({
    count: this.state.count + 1
  });
  console.log('increment setState后的count', this.state.count)
  // 执行完函数再放开
  isBatchingUpdates = false
}
```

开头锁上的那个 isBatchingUpdates，对 setTimeout 内部的执行逻辑完全没有约束力。因为 isBatchingUpdates 是在同步代码中变化的，而 setTimeout 的逻辑是异步执行的。当 this.setState 调用真正发生的时候，isBatchingUpdates 早已经被重置为了 false

```js
reduce = () => {
  // 进来先锁上
  isBatchingUpdates = true
  setTimeout(() => {
    console.log('reduce setState前的count', this.state.count)
    this.setState({
      count: this.state.count - 1
    });
    console.log('reduce setState后的count', this.state.count)
  },0);
  // 执行完函数再放开
  isBatchingUpdates = false
}
```

总结：

setState 并不是单纯同步/异步的，它的表现会因调用场景的不同而不同：在 React 钩子函数及合成事件中，它表现为异步；而在 setTimeout、setInterval 等函数中，包括在 DOM 原生事件中，它都表现为同步。这种差异，本质上是由 React 事务机制和批量更新机制的工作方式来决定的。

## 了解Fiber

React15使用的Stack Reconciler，Stack Reconciler 是一个同步的递归过程，在diff算法递归遍历的过程中是同步执行的，不可以打断，当处理结构相对复杂、体量相对庞大的虚拟 DOM 树时，Stack Reconciler 需要的调和时间会很长，这就意味着 JavaScript 线程将长时间地霸占主线程，进而导致我们上文中所描述的渲染卡顿/卡死、交互长时间无响应等问题，随着业务复杂度的提升在体验方面体现出了疲态。为了更进一步贯彻“快速响应”的原则，在 16.x 版本中将其最为核心的 Diff 算法整个重写，使其以“Fiber Reconciler”的全新面貌示人。

什么是Fiber？

在计算机科学里，我们有进程、线程之分，而 Fiber 就是比线程还要纤细的一个过程，也就是所谓的“纤程”。纤程的出现，意在对渲染过程实现更加精细的控制。

Fiber 架构的应用目的是实现“增量渲染”。所谓“增量渲染”，通俗来说就是把一个渲染任务分解为多个渲染任务，而后将其分散到多个帧里面。不过严格来说，增量渲染其实也只是一种手段，实现增量渲染的目的，是为了实现任务的可中断、可恢复，并给不同的任务赋予不同的优先级，最终达成更加顺滑的用户体验。

在React15是两层架构

Reconciler(找不同) -> Renderer(渲染不同)

在React16改成了三层架构

Scheduler(调度更新优先级) -> Reconciler -> Renderer

在这套架构模式下，更新的处理工作流变成了这样：首先，每个更新任务都会被赋予一个优先级。当更新任务抵达调度器时，高优先级的更新任务（记为 A）会更快地被调度进 Reconciler 层；此时若有新的更新任务（记为 B）抵达调度器，调度器会检查它的优先级，若发现 B 的优先级高于当前任务 A，那么当前处于 Reconciler 层的 A 任务就会被中断，调度器会将 B 任务推入 Reconciler 层。当 B 任务完成渲染后，新一轮的调度开始，之前被中断的 A 任务将会被重新推入 Reconciler 层，继续它的渲染之旅，这便是所谓“可恢复”。

![Drawing 7.png](https://s0.lgstatic.com/i/image/M00/6E/CC/Ciqc1F-zlhKAMu6ZAACYDSGoCUY002.png)

在 render 阶段，一个庞大的更新任务被分解为了一个个的工作单元，这些工作单元有着不同的优先级，React 可以根据优先级的高低去实现工作单元的打断和恢复。由于 render 阶段的操作对用户来说其实是“不可见”的，所以就算打断再重启，对用户来说也是 0 感知。但是，工作单元（也就是任务）的重启将会伴随着对部分生命周期的重复执行，这些生命周期是：

- componentWillMount

- componentWillUpdate

- shouldComponentUpdate

- componentWillReceiveProps

## React事件流

要想理解好 React 事件机制，就必须对原生 DOM 事件流有扎实的掌握。W3C 标准约定了一个事件的传播过程要经过以下 3 个阶段：

- 事件捕获阶段

- 目标阶段

- 事件冒泡阶段

当事件被触发时，首先经历的是一个捕获过程：事件会从最外层的元素开始“穿梭”，逐层“穿梭”到最内层元素，这个过程会持续到事件抵达它目标的元素（也就是真正触发这个事件的元素）为止；此时事件流就切换到了“目标阶段”——事件被目标元素所接收；然后事件会被“回弹”，进入到冒泡阶段——它会沿着来时的路“逆流而上”，一层一层再走回去。

### React 事件系统是如何工作的

React 的事件系统沿袭了事件委托的思想。在 React 中，除了少数特殊的不可冒泡的事件（比如媒体类型的事件）无法被事件系统处理外，绝大部分的事件都不会被绑定在具体的元素上，而是统一被绑定在页面的 document 上。当事件在具体的 DOM 节点上被触发后，最终都会冒泡到 document 上，document 上所绑定的统一事件处理程序会将事件分发到具体的组件实例。在分发事件之前，React 首先会对事件进行包装，把原生 DOM 事件包装成合成事件。

![Drawing 8.png](https://s0.lgstatic.com/i/image/M00/78/87/CgqCHl_KCjaALFKsAAHNjlT3rrw342.png)

## 性能优化

### 类组件

**shouldComponentUpdate**：`shouldComponentUpdate(nextProps, nextState)`，默认返回true。React 组件会根据 `shouldComponentUpdate` 的返回值，来决定是否执行该方法之后的生命周期，进而决定是否对组件进行 re-render（重渲染）。

**PureComponent**：`PureComponent` 与 `Component` 的区别点，就在于它内置了对 `shouldComponentUpdate` 的实现：`PureComponent` 将会在 `shouldComponentUpdate` 中对组件更新前后的 props 和 state 进行浅比较，并根据浅比较的结果，决定是否需要继续更新流程。“浅比较”将针对值类型数据对比其值是否相等，而针对数组、对象等引用类型的数据则对比其引用是否相等。

**Immutable**：“不可变值”。若数据内容没变，但是引用变了，那么浅比较仍然会认为“数据发生了变化”，进而触发一次不必要的更新，导致过度渲染；若数据内容变了，但是引用没变，那么浅比较则会认为“数据没有发生变化”，进而阻断一次更新，导致不渲染。这时候就可以用上Immutable了。数据只要被创建出来了，就不能被更改。我们对当前数据的任何修改动作，都会导致一个新的对象的返回。

### 函数式组件

**React.memo** ：`React.memo` 会帮我们“记住”函数组件的渲染结果，在组件前后两次 props 对比结果一致的情况下，它会直接复用最近一次渲染的结果

```js
import React from "react";
// 定义一个函数组件
function FunctionDemo(props) {
  return xxx
}
// areEqual 函数是 memo 的第二个入参，我们之前放在 shouldComponentUpdate 里面的逻辑就可以转移至此处
function areEqual(prevProps, nextProps) {
  /*
  return true if passing nextProps to render would return
  the same result as passing prevProps to render,
  otherwise return false
  */
}
// 使用 React.memo 来包装函数组件
export default React.memo(FunctionDemo, areEqual);
```

这里的 areEqual 函数是一个可选参数，当我们不传入 `areEqual` 时，`React.memo` 也可以工作，此时它的作用就类似于 `PureComponent`——`React.memo` 会自动为你的组件执行 props 的浅比较逻辑。

**useMemo**：  `React.memo` 控制是否需要重渲染一个组件，而 `useMemo` 控制的则是是否需要重复执行某一段逻辑。

## 逻辑复用

### 高阶组件HOC

接收函数作为输入，或者输出另一个函数的一类函数，就是高阶函数。

```js
// 假设 checkUserAccess 已经在 utils 文件中被封装为了一段独立的逻辑
import checkUserAccess from './utils
// 用高阶组件包裹目标组件
const withCheckAccess = (WrappedComponent) => {
    // 这部分是通用的逻辑：判断用户身份是否合法
    const isAccessible = checkUserAccess()  
    // 将 isAccessible（是否合法） 这个信息传递给目标组件
    const targetComponent = (props) => (
        <div className="wrapper-container">
            <WrappedComponent {...props} isAccessible={isAccessible} />
        </div>
    );
    return targetComponent;
};

//需要复用逻辑时，就直接包裹这个组件，EnhancedAComponent 轻松具备了校验用户合法性的能力
const EnhancedAComponent = withCheckAccess(Acomponent);
```

### Render props(另一种逻辑复用方式)

高阶组件的使用姿势是用“函数”包裹“组件”，而 render props 恰恰相反，它强调的是用“组件”包裹“函数”。

```js
// 假设 checkUserAccess 已经在 utils 文件中被封装为了一段独立的逻辑
import checkUserAccess from './utils
// 定义 render props 组件
const CheckAccess = (props) => {
    // 这部分是通用的逻辑：判断用户身份是否合法
    const isAccessible = checkUserAccess()  
    // 将 isAccessible（是否合法） 这个信息传递给目标组件
    return <React.Fragment>
        {props.children({ ...props, isAccessible })}
      </React.Fragment>
};


<CheckAccess>
  {
    (props) => {
      const { isAccessible } = props;
      return <ChildComponent {...props} isAccessible={isAccessible} />
    }
  }
</CheckAccess>
```

render props 和高阶组件一个非常重要的区别，在于对数据的处理上：在高阶组件中，目标组件对于数据的获取没有主动权，数据的分发逻辑全部收敛在高阶组件的内部；而在 render props 中，除了父组件可以对数据进行分发处理之外，子组件也可以选择性地对数据进行接收。

### 总结：

现在，当我们想要去复用一段逻辑时，第一反应肯定不是“高阶函数”或者“render props”，而应该是“自定义 Hook”。Hooks 能够很好地规避掉旧时类组件中各种设计模式带来的弊端，比如说它不存在嵌套地狱，允许属性重命名、允许我们在任何需要它的地方引入并访问目标状态等。