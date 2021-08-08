### 1.类组件和函数组件的本质区别

**对于类组件来说，底层只需要实例化一次，实例中保存了组件的 state 等状态。对于每一次更新只需要调用 render 方法以及对应的生命周期就可以了。但是在函数组件中，每一次更新都是一次新的函数执行，一次函数组件的更新，里面的变量会重新声明。**

### 2. 通信方式

React 一共有 5 种主流的通信方式：

- props 和 callback 方式：推荐使用

- ref 方式，父组件直接获取子组件实例并获取子组件属性和调用子组件内的方法。

- React-redux 或 React-mobx 状态管理方式，全局状态管理

- context 上下文方式，多层组件传参

- event bus 事件总线：不推荐这种方式。

  原因：1.需要手动绑定和解绑。2.对于小型项目还好，但是对于中大型项目，这种方式的组件通信，会造成牵一发动全身的影响，而且后期难以维护，组件之间的状态也是未知的。3.一定程度上违背了 React 数据流向原则。

### 3.State是同步还是异步的？

React 是有多种模式的，基本平时用的都是 legacy 模式下的 React，除了`legacy` 模式，还有 `blocking` 模式和 `concurrent` 模式， blocking 可以视为 concurrent 的优雅降级版本和过渡版本，React 在不久的未来会以 concurrent 模式作为默认版本，这个模式下会开启一些新功能。

`在legacy模式下`，setState发生的全流程。

- 首先，setState 会产生当前更新的优先级（老版本用 expirationTime ，新版本用 lane ）。
- 接下来 React 会从 fiber Root 根部 fiber 向下调和子节点，调和阶段将对比发生更新的地方，更新对比 expirationTime ，找到发生更新的组件，合并 state，然后触发 render 函数，得到新的 UI 视图层，完成 render 阶段。
- 接下来到 commit 阶段，commit 阶段，替换真实 DOM ，完成此次更新流程。
- 接下来会执行 setState 中 callback 函数,如上的`()=>{ console.log(this.state.number) }`，到此为止完成了一次 setState 全过程。

render 阶段： render 函数执行 -> commit 阶段真实 DOM 替换 -> setState 回调函数执行 callback 。

```js
function batchedEventUpdates(fn,a){
    /* 开启批量更新  */
   isBatchingEventUpdates = true;
  try {
    /* 这里执行了的事件处理函数， 比如在一次点击事件中触发setState,那么它将在这个函数内执行 */
    return batchedEventUpdatesImpl(fn, a, b);
  } finally {
    /* try 里面 return 不会影响 finally 执行  */
    /* 完成一次事件，批量更新  */
    isBatchingEventUpdates = false;
  }
}
```

在 React 事件执行之前通过 `isBatchingEventUpdates=true` 打开开关，开启事件批量更新，当该事件结束，再通过 `isBatchingEventUpdates = false;` 关闭开关，然后在 scheduleUpdateOnFiber 中根据这个开关来确定是否进行批量更新。

在异步函数中，setTimeout或者Promise里多次调用setState，这个规则就会被打破，因为是先执行把批量更新开关打开，然后再关闭，异步函数时机到了回调的时候才去执行，所以批量更新的规则就不存在了。

```js
setTimeout(()=>{
    this.setState({ number:this.state.number + 1 },()=>{   console.log( 'callback1', this.state.number)  })
    console.log(this.state.number)
    this.setState({ number:this.state.number + 1 },()=>{    console.log( 'callback2', this.state.number)  })
    console.log(this.state.number)
    this.setState({ number:this.state.number + 1 },()=>{   console.log( 'callback3', this.state.number)  })
    console.log(this.state.number)
})
```

打印 ： **callback1 1 , 1, callback2 2 , 2,callback3 3 , 3** 

React-Dom 中提供了批量更新方法 `unstable_batchedUpdates`，可以去手动批量更新，可以将上述 setTimeout 里面的内容做如下修改:

```js
import ReactDOM from 'react-dom'
const { unstable_batchedUpdates } = ReactDOM
setTimeout(()=>{
    unstable_batchedUpdates(()=>{
        this.setState({ number:this.state.number + 1 })
        console.log(this.state.number)
        this.setState({ number:this.state.number + 1})
        console.log(this.state.number)
        this.setState({ number:this.state.number + 1 })
        console.log(this.state.number) 
    })
})
```

打印： **0 , 0 , 0 , callback1 1 , callback2 1 ,callback3 1**

有人给出这个结论：

**“setState 只在合成事件和钩子函数中是“异步”的,在原生事件和 setTimeout 中都是同步的。”**

但我们可以看到其实setState本身并不是异步，只是因为React的更新机制才体现为异步。

两个图片看差异：

https://p1-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/478aef991b4146c898095b83fe3dc0e7~tplv-k3u1fbpfcp-watermark.image

https://p1-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/48e730fc687c4ce087e5c0eab2832273~tplv-k3u1fbpfcp-watermark.image

**`dispatch`更新特点**

上述讲的批量更新和 flushSync ，在函数组件中，dispatch 更新效果和类组件是一样的，但是 useState 有一点值得注意，就是当调用改变 state 的函数dispatch，在本次函数执行上下文中，是获取不到最新的 state 值的，把上述demo 如下这么改：

```js
const [ number , setNumber ] = React.useState(0)
const handleClick = ()=>{
    ReactDOM.flushSync(()=>{ //设置高优先级
        setNumber(2) 
        console.log(number) 
    })
    setNumber(1) 
    console.log(number)
    setTimeout(()=>{
        setNumber(3) 
        console.log(number)
    })   
}
```

**结果： 0 0 0**

原因很简单，函数组件更新就是函数的执行，在函数一次执行过程中，函数内部所有变量重新声明，所以改变的 state ，只有在下一次函数组件执行时才会被更新。所以在如上同一个函数执行上下文中，number 一直为0，无论怎么打印，都拿不到最新的 state 。

#### useState使用原理

在 useState 的 dispatchAction 处理逻辑中，会浅比较两次 state ，发现 state 相同，不会开启更新调度任务。所以如果要更新一个对象或数组，要进行浅拷贝，重新开辟一个新空间才能解决更新问题。

#### 类组件中的 `setState` 和函数组件中的 `useState` 有什么异同？ 

**相同点：**从原理角度出发，setState和 useState 更新视图，底层都调用了 scheduleUpdateOnFiber 方法，而且事件驱动情况下都有批量更新规则。

**不同点**：

- 在不是 pureComponent 组件模式下， setState 不会浅比较两次 state 的值，只要调用 setState，在没有其他优化手段的前提下，就会执行更新。但是 useState 中的 dispatchAction 会默认比较两次 state 是否相同，然后决定是否更新组件。
- setState 有专门监听 state 变化的回调函数 callback，可以获取最新state；但是在函数组件中，只能通过 useEffect 来执行 state 变化引起的副作用。
- setState 在底层处理逻辑上主要是和老 state 进行合并处理，而 useState 更倾向于重新赋值。

### 4.如何选择 useEffect 和 useLayoutEffect

修改 DOM ，改变布局就用 useLayoutEffect ，其他情况就用 useEffect

### Fiber架构

React在它的V16版本推出了Fiber架构，在弄清楚什么是Fiber之前，我们应该先了解为什么需要Fiber。

首先，浏览器是多线程的，这些线程包括JS引擎线程（主线程），以及GUI渲染线程，定时器线程，事件线程等工作线程。其中，JS引擎线程和GUI渲染线程是互斥的。又因为绝大多数的浏览器页面的刷新频率取决于显示器的刷新频率，即每16.6毫秒就会通过GUI渲染引擎刷新一次。所以，如果JS引擎线程一次性执行了一个长时间（大于16.6毫秒）的同步任务，就可能出现掉帧的情况，影响用户的体验。

而在旧版本的React中，对于一个庞大的组件，无论是组件的创建还是更新都可能需要较长的时间。而Fiber的思路是将原本耗时较长的同步任务分片为多个任务单元，执行完一个任务单元后可以保存当前的状态，切换到GUI渲染线程去刷新页面，接下来再回到主线程并从上个断点继续执行任务。

我的个人体会，React中的Fiber（纤程）类似或者说就是Coroutine（协程）。ES6的Generator本身也算是协程的一种实现，或者说是状态机，通过它能够得到一个可以暂停的函数任务；而React中的Fiber，将原本耗时很长的同步任务分成多个耗时短的分片，从而实现了浏览器中互斥的主线程与GUI渲染线程之间的调度。

除此之外，对于每一个Fiber的同步任务来说，都拥有一个优先级（总共定义了6种优先级）。

当主线程刚执行完一个任务A的一个分片，若此时出现了一个优先级更高的任务B，React就可能会把任务A废弃掉，待之后重新执行一次任务A。

为什么这里要加一个可能，这是因为对于使用了Fiber的React来说，组件可以分为两个阶段，分别是“Render/Reconciliation phase”和"Commit phase"，可以在官方的生命周期图谱看到具体的信息。第一个阶段是没有副作用的，也因此可以被React暂停，废弃又或者重新执行；而第二个阶段会涉及到实际的DOM，是有副作用的，所以无法被React暂停，重新执行。

那么结合上面两段，可以知道处于“Render/Reconciliation phase”的任务A，如果执行时出现了优先级更高的任务B，任务A就会被废弃，之后重新被执行。

举个例子。由于componentWillMount已经要被React废弃了，所以在以上链接中的图谱没有被标出来，它其实也是属于"Render/Reconciliation phase"的。那么当一个组件即将挂载时，就会调用这个生命周期钩子，假如在这之后我们就碰到了优先级更高的任务，那么原本的任务就会被废弃，并在之后被重新调用。导致的结果就是componentWillMount被调用了两次，这是一个值得注意的点。
