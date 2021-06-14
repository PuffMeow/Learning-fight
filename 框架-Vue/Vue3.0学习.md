## Vue3.0的变动

### 代码组织管理方式

使用monorepo的方式进行代码管理，把不同功能模块拆分到不同的package中，每个package都有自己的API、类型和测试，让模块拆分更加细化，职责更加明确，提高了代码的可维护性，就像reactive响应式库是可以独立于vue之外去使用的

### 语言层面

在Vue2.0里Vue使用了Facebook的Flow去做类型系统，但是Flow在一些复杂场景类型的检查，支持不友好。所以在Vue3.0里放弃了Flow全面拥抱了Typescript，毕竟Typescript是前端的未来。

### 性能优化

- 源码体积减少：移除冷门的feature（filter、mixin等），使用了tree-shaking技术（依赖于import和export，通过编译阶段的静态分析，找到没有引入的模块打上标记，没引入的模块不会被打包，减少了项目的体积）

- 响应式系统优化：Vue2.0中使用的Object.defineProperty这个API去对数据做劫持，但是它存在着缺陷，就是必须要预先知道对象的key是什么，所以它并不能检测对象属性的添加和删除，在Vue2里为了解决这个问题提供了$set和$delete的API。当嵌套层级比较深的时候，就需要递归的去遍历这个对象，把每一层数据都变成响应式的，如果这个对象过于复杂，那么性能开销就会比较大。为了解决这个问题，Vue3就使用了Proxy API做数据劫持。

  ```javascript
  observed = new Proxy(data, {
      get() {},
      set() {}
  })// Proxy暂时没办法支持低版本浏览器(polyfill)
  ```

  这个API是监听整个对象，所以能对属性的增加和删除都可以检测到。但是Proxy这个API也不能监听到内部深层的对象变化，所以Vue3.0的处理方式是在getter中递归响应式。这样的好处就是只有真正访问到内部对象的时候才会变成响应式，而不是一股脑就去做递归，很大程度上提高了性能。

- 编译优化：Vue2.0的执行过程是这样的`new Vue -> init -> mount -> complie -> render -> vnode -> patch -> DOM`，响应式的过程发生在init阶段。除了数据劫持的优化，Vue3.0在编译阶段也进行了优化，实现了运行时patch过程的优化。Vue2数据更新并触发重新渲染的粒度是组件级的。Vue3通过在编译阶段对静态模板的分析，编译生成了Block tree（将模板基于动态节点指令切割的嵌套区块，每个区块内部节点结构固定，每个区块只需要以一个Array来追踪自身包含的动态节点）。借助Block tree，vue将vnode更新性能由模板整体大小相关提升到了跟动态内容数量相关。在编译阶段还对Slot进行了编译优化，事件侦听函数的缓存优化，并在运行时重写了diff算法。

- 语法优化：vue2使用了Options API(methods、computed、data、props、created、mounted、updated等)这些不同的的选项分类，每个option都有自己的内容，如果需要修改一段上下文相关的逻辑，当组件大了以后，就需要在单个文件中不断上下切换和寻找)。Vue3提供了composition API(把某个逻辑关注点相关的代码全放到一个函数里，当要修改一个功能的时候，就不需要在文件中上下切换)

- 优化逻辑复用：vue2复用逻辑采用了mixins，但是当组件混入大量不同的mixin时，就存在命名冲突和数据来源不清晰的问题。每个mixin都可以定义自己的props和data，很容易定义相同的变量导致命名冲突。对组件来说在模板中使用不在当前组件中定义的变量就很难找到这些变量在哪里定义，导致了数据来源不清晰。Composition API就较好的解决了这两个问题。useXxxx（类似于自定义hooks）。Composition API在逻辑复用方面有优势，会有更好的类型支持，也对tree-shaking友好，代码也更容易压缩。

## Vue核心组件实现

### vnode如何转变成真实DOM

组件是对一棵DOM树的抽象，组件渲染生成DOM的步骤是`创建vnode->渲染vnode->生成DOM`，Vue3.0应用初始化

```js
import {createApp} from 'vue'
import App from './app'
const app = createApp(App)
app.mount('#app')
```

vnode本质是用来描述DOM的JS对象，在Vue中可以描述不同类型的节点，比如说元素节点、组件节点、注释节点、纯文本节点等。

### 为什么要有vnode？

- 抽象：把渲染过程抽象化，使得组件抽象能力得到提升
- 跨平台：patch vnode的过程不同平台有自己的实现，基于vnode再做服务端渲染、weex平台、小程序平台的渲染。

使用vnode性能不一定比手动操作原生DOM性能好。每次render to vnode的过程渲染组件都会有一定的Js执行耗时，特别是大组件。虽然diff算法在减少DOM操作方面足够优秀，但是还是避免不了操作DOM，所以性能不是vnode的优势

Vue3中根组件通过createVNode函数渲染

### 更新组件

更新组件主要做三件事情：更新组件 vnode 节点、渲染新的子树 vnode、根据新旧子树 vnode 执行 patch 逻辑

副作用渲染函数更新组件的过程

```javascript
const setupRenderEffect = (instance, initialVNode, container, anchor, parentSuspense, isSVG, optimized) => {
  // 创建响应式的副作用渲染函数
  instance.update = effect(function componentEffect() {
    if (!instance.isMounted) {
      // 渲染组件
    }
    else {
      // 更新组件
      let { next, vnode } = instance
      // next 表示新的组件 vnode
      if (next) {
        // 更新组件 vnode 节点信息
        updateComponentPreRender(instance, next, optimized)
      }
      else {
        next = vnode
      }
      // 渲染新的子树 vnode
      const nextTree = renderComponentRoot(instance)
      // 缓存旧的子树 vnode
      const prevTree = instance.subTree
      // 更新子树 vnode
      instance.subTree = nextTree
      // 组件更新核心逻辑，根据新旧子树 vnode 做 patch
      patch(prevTree, nextTree,
        // 如果在 teleport 组件中父节点可能已经改变，所以容器直接找旧树 DOM 元素的父节点
        hostParentNode(prevTree.el),
        // 参考节点在 fragment 的情况可能改变，所以直接找旧树 DOM 元素的下一个节点
        getNextHostNode(prevTree),
        instance,
        parentSuspense,
        isSVG)
      // 缓存更新后的 DOM 节点
      next.el = nextTree.el
    }
  }, prodEffectOptions)
}
```

### Patch过程

```js
const patch = (n1, n2, container, anchor = null, parentComponent = null, parentSuspense = null, isSVG = false, optimized = false) => {
  // 如果存在新旧节点, 且新旧节点类型不同，则销毁旧节点
  if (n1 && !isSameVNodeType(n1, n2)) {
    anchor = getNextHostNode(n1)
    unmount(n1, parentComponent, parentSuspense, true)
    // n1 设置为 null 保证后续都走 mount 逻辑
    n1 = null
  }
    
  const { type, shapeFlag } = n2
  switch (type) {
    case Text:
      // 处理文本节点
      break
    case Comment:
      // 处理注释节点
      break
    case Static:
      // 处理静态节点
      break
    case Fragment:
      // 处理 Fragment 元素
      break
    default:
      if (shapeFlag & 1 /* ELEMENT */) {
        // 处理普通 DOM 元素
        processElement(n1, n2, container, anchor, parentComponent, parentSuspense, isSVG, optimized)
      }
      else if (shapeFlag & 6 /* COMPONENT */) {
        // 处理组件
        processComponent(n1, n2, container, anchor, parentComponent, parentSuspense, isSVG, optimized)
      }
      else if (shapeFlag & 64 /* TELEPORT */) {
        // 处理 TELEPORT
      }
      else if (shapeFlag & 128 /* SUSPENSE */) {
        // 处理 SUSPENSE
      }
  }
}

function isSameVNodeType (n1, n2) {
  // n1 和 n2 节点的 type 和 key 都相同，才是相同节点
  return n1.type === n2.type && n1.key === n2.key
}
```

在这个过程中，首先判断新旧节点是否是相同的 vnode 类型，如果不同，比如一个 div 更新成一个 ul，那么最简单的操作就是删除旧的 div 节点，再去挂载新的 ul 节点。

如果是相同的 vnode 类型，就需要走 diff 更新流程了，接着会根据不同的 vnode 类型执行不同的处理逻辑，这里我们仍然只分析普通元素类型和组件类型的处理过程。