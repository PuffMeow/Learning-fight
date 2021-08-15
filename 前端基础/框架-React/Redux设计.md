## 了解Redux

全局存储仓库：Redux 主要由 3 部分组成：Store、Reducer 和 Action。

- Store：它是一个单一的数据源，而且是只读的。

- Action 人如其名，是“动作”的意思，它是对变化的描述。

- Reducer 是一个函数，它负责对变化进行分发和处理，最终将新的数据返回给 Store。

Store、Action 和 Reducer 三者紧密配合，便形成了 Redux 独树一帜的工作流

![Drawing 13.png](https://s0.lgstatic.com/i/image/M00/7E/C6/CgqCHl_PVh-ATfOGAAB089LdYcY341.png)

理解 Redux 实现原理，真正需要我们关注的模块其实只有一个——createStore。createStore 方法是我们在使用 Redux 时最先调用的方法，它是整个流程的入口，也是 Redux 中最核心的 API。

```js
// 引入 redux
import { createStore } from 'redux'
// 创建 store
const store = createStore(
    reducer, //reducer
    initial_state,  //初始状态内容
    applyMiddleware(middleware1, middleware2, ...)  //中间件
);
```

## 分析源码主要逻辑

```js
function createStore(reducer, preloadedState, enhancer) {
    // 这里处理的是没有设定初始状态的情况，也就是第一个参数和第二个参数都传 function 的情况
    if (typeof preloadedState === 'function' && typeof enhancer === 'undefined') {
        // 此时第二个参数会被认为是 enhancer（中间件）
        enhancer = preloadedState;
        preloadedState = undefined;
    }
    // 当 enhancer 不为空时，便会将原来的 createStore 作为参数传入到 enhancer 中
    if (typeof enhancer !== 'undefined') {
        return enhancer(createStore)(reducer, preloadedState);
    }
    // 记录当前的 reducer，因为 replaceReducer 会修改 reducer 的内容
    let currentReducer = reducer;
    // 记录当前的 state
    let currentState = preloadedState;
    // 声明 listeners 数组，这个数组用于记录在 subscribe 中订阅的事件
    let currentListeners = [];
    // nextListeners 是 currentListeners 的快照
    let nextListeners = currentListeners;
    // 该变量用于记录当前是否正在进行 dispatch
    let isDispatching = false

    // 该方法用于确认快照是 currentListeners 的副本，而不是 currentListeners 本身
    function ensureCanMutateNextListeners() {
        if (nextListeners === currentListeners) {
            nextListeners = currentListeners.slice();
        }
    }

    // 我们通过调用 getState 来获取当前的状态
    function getState() {
        return currentState;
    }

    // subscribe 订阅方法，它将会定义 dispatch 最后执行的 listeners 数组的内容
    function subscribe(listener) {
        // 校验 listener 的类型
        if (typeof listener !== 'function') {
          throw new Error('Expected the listener to be a function.')
        }
        // 禁止在 reducer 中调用 subscribe
        if (isDispatching) {
          throw new Error(
            'You may not call store.subscribe() while the reducer is executing. ' +
              'If you would like to be notified after the store has been updated, subscribe from a ' +
              'component and invoke store.getState() in the callback to access the latest state. ' +
              'See https://redux.js.org/api-reference/store#subscribe(listener) for more details.'
          )
        }
        // 该变量用于防止调用多次 unsubscribe 函数
        let isSubscribed = true;
        // 确保 nextListeners 与 currentListeners 不指向同一个引用
        ensureCanMutateNextListeners(); 
        // 注册监听函数
        nextListeners.push(listener); 

        // 返回取消订阅当前 listener 的方法
        return function unsubscribe() {
            if (!isSubscribed) {
                return;
            }
            isSubscribed = false;
            ensureCanMutateNextListeners();
            const index = nextListeners.indexOf(listener);
            // 将当前的 listener 从 nextListeners 数组中删除 
            nextListeners.splice(index, 1);
        };
    }
    
    
    //Redux 工作流的核心：dispatch 动作
    // 定义 dispatch 方法，用于派发 action 
    function dispatch(action) {
        // 校验 action 的数据格式是否合法
        if (!isPlainObject(action)) {
          throw new Error(
            'Actions must be plain objects. ' +
              'Use custom middleware for async actions.'
          )
        }

        // 约束 action 中必须有 type 属性作为 action 的唯一标识 
        if (typeof action.type === 'undefined') {
          throw new Error(
            'Actions may not have an undefined "type" property. ' +
              'Have you misspelled a constant?'
          )
        }

        // 若当前已经位于 dispatch 的流程中，则不允许再度发起 dispatch（禁止套娃）
        if (isDispatching) {
          throw new Error('Reducers may not dispatch actions.')
        }
        try {
          // 执行 reducer 前，先"上锁"，标记当前已经存在 dispatch 执行流程
          isDispatching = true
          // 调用 reducer，计算新的 state 
          currentState = currentReducer(currentState, action)
        } finally {
          // 执行结束后，把"锁"打开，允许再次进行 dispatch 
          isDispatching = false
        }

        // 触发订阅
        const listeners = (currentListeners = nextListeners);
        for (let i = 0; i < listeners.length; i++) {
            const listener = listeners[i];
            listener();
        }
        return action;
    }

    // replaceReducer 可以更改当前的 reducer
    function replaceReducer(nextReducer) {
        currentReducer = nextReducer;
        dispatch({ type: ActionTypes.REPLACE });
        return store;
    }

    // 初始化 state，当派发一个 type 为 ActionTypes.INIT 的 action，每个 reducer 都会返回
    // 它的初始值
    dispatch({ type: ActionTypes.INIT });

    // observable 方法可以忽略，它在 redux 内部使用，开发者一般不会直接接触
    function observable() {
      // observable 方法的实现
    }

    // 将定义的方法包裹在 store 对象里返回
    return {
      dispatch,
      subscribe,
      getState,
      replaceReducer,
      [$$observable]: observable
    }
}
```

![Drawing 15.png](https://s0.lgstatic.com/i/image/M00/7E/BB/Ciqc1F_PVkCAST4AAAJfMvoaI4Q803.png)

### dispatch的流程

![å¾ç2.png](https://s0.lgstatic.com/i/image/M00/84/07/Ciqc1F_TF96ACQdrAADNE7uQ1fw478.png)

### Redux 中的“发布-订阅”模式：认识 subscribe

dispatch 中执行的 listeners 数组从订阅中来，而执行订阅需要调用 subscribe。在实际的开发中，subscribe 并不是一个严格必要的方法，只有在需要监听状态的变化时，我们才会调用 subscribe。

```js
function subscribe(listener) {
  // 校验 listener 的类型
  if (typeof listener !== 'function') {
    throw new Error('Expected the listener to be a function.')
  }
  // 禁止在 reducer 中调用 subscribe
  if (isDispatching) {
    throw new Error(
      'You may not call store.subscribe() while the reducer is executing. ' +
      'If you would like to be notified after the store has been updated, subscribe from a ' +
      'component and invoke store.getState() in the callback to access the latest state. ' +
      'See https://redux.js.org/api-reference/store#subscribe(listener) for more details.'
    )
  }
  // 该变量用于防止调用多次 unsubscribe 函数
  let isSubscribed = true;
  // 确保 nextListeners 与 currentListeners 不指向同一个引用
  ensureCanMutateNextListeners(); 
  // 注册监听函数
  nextListeners.push(listener); 
  // 返回取消订阅当前 listener 的方法
  return function unsubscribe() {
    if (!isSubscribed) {
      return;
    }
    isSubscribed = false;
    ensureCanMutateNextListeners();
    const index = nextListeners.indexOf(listener);
    // 将当前的 listener 从 nextListeners 数组中删除 
    nextListeners.splice(index, 1);
  };
}
```

![å¾ç5.png](https://s0.lgstatic.com/i/image/M00/84/13/CgqCHl_TGCiAfm8CAAEEFFa3ZxA563.png)

