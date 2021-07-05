目录：packages/react/src/ReactHooks



```typescript
/**
 * @flow
 */

import type {
  MutableSource,
  MutableSourceGetSnapshotFn,
  MutableSourceSubscribeFn,
  ReactContext,
} from 'shared/ReactTypes';
import type { OpaqueIDType } from 'react-reconciler/src/ReactFiberHostConfig';

import invariant from 'shared/invariant';

//拿到当前的调度器
import ReactCurrentDispatcher from './ReactCurrentDispatcher';

type BasicStateAction<S> = (S => S) | S;
type Dispatch<A> = A => void;

/**
 * 该方法返回当前跟踪到的的调度器
 */
function resolveDispatcher() {
  const dispatcher = ReactCurrentDispatcher.current;
  invariant(
    dispatcher !== null,
    'Invalid hook call. Hooks can only be called inside of the body of a function component. This could happen for' +
    ' one of the following reasons:\n' +
    '1. You might have mismatching versions of React and the renderer (such as React DOM)\n' +
    '2. You might be breaking the Rules of Hooks\n' +
    '3. You might have more than one copy of React in the same app\n' +
    'See https://reactjs.org/link/invalid-hook-call for tips about how to debug and fix this problem.',
  );
  return dispatcher;
}

/**
 * 用法：const context = useContext(MyContext);
 * useContext<泛型>(Context, unstable_observedBits):泛型
 * 传入两个参数，第一个参数是你自己createContext出来的需要传递的数据
 * 第二个参数目前暂时不支持传入
 * 官方解释：接收一个 context 对象（React.createContext 的返回值）并返回该 context 的当前值
 * 
 * 优化方式：调用了 useContext 的组件总会在 context 值变化时重新渲染。
 * 如果重渲染组件的开销较大，你可以 通过使用 memoization 来优化。
 * 
 */
export function useContext<T>(
  Context: ReactContext<T>,
  unstable_observedBits: number | boolean | void,
): T {
  const dispatcher = resolveDispatcher();  //拿到当前的调度器
  if (__DEV__) {
    if (unstable_observedBits !== undefined) {
      console.error(
        'useContext()第二个参数可能会在未来删除 ' +
        '目前不支持传递该参数 ' +
        '你传递了: %s.%s',
        unstable_observedBits,
        typeof unstable_observedBits === 'number' && Array.isArray(arguments[2])
          ? '\n\n你是否在调用array.map(useContext)？' +
          '不支持在循环中调用 ' +
          '更多详情请看 https://reactjs.org/link/rules-of-hooks'
          : '',
      );
    }

    // TODO: 通用场景下增加更多对无用值的警告
    if ((Context: any)._context !== undefined) {
      const realContext = (Context: any)._context;
      //不要删除重复数据因为可能会引发bug，任何人都不应该在现有代码中使用这样错误的判断方式
      // 别忘记 useContext 的参数必须是 context 对象本身：
      // 正确： useContext(MyContext)
      // 错误： useContext(MyContext.Consumer)
      // 错误： useContext(MyContext.Provider)
      if (realContext.Consumer === Context) {
        console.error(
          '不支持调用useContext(Context.Consumer)，可能会产生bug，将在未来的主要版本中被移除。你想调用的是useContext(Context)吗？'
        );
      } else if (realContext.Provider === Context) {
        console.error(
          '不支持调用useContext(Context.Consumer),你想调用的是useContext(Context)吗？'
        );
      }
    }
  }
  //将useContext挂载到当前的调度器上
  return dispatcher.useContext(Context, unstable_observedBits);
}

    
/**
 * 用法：const [state, setState] = useState(initialState);
 * 传入一个参数，支持泛型传入 useState<string>('Test')
 * 并返回一个解构数组:[传入的状态，设置该状态的方法]
 * type BasicStateAction<S> = (S => S) | S;
 * type Dispatch<A> = A => void;
 * 
 * 官方解释：
 * 与 class 组件中的 setState 方法不同，useState 不会自动合并更新对象。
 * 你可以用函数式的 setState 结合展开运算符来达到合并更新对象的效果。
 * setState(prevState => ({...prevState, ...updatedValues}));
 */
export function useState<S>(
  initialState: (() => S) | S,
): [S, Dispatch<BasicStateAction<S>>] {
  const dispatcher = resolveDispatcher();
  //将useStatae挂载到当前的调度器上
  return dispatcher.useState(initialState);
}

/**
 * 用法：const [state, dispatch] = useReducer(reducer, initialArg, init);
 * useState 的替代方案，在复杂的场景下比useState更加适用
 * 例如 state 逻辑较复杂且包含多个子值，或者下一个 state 依赖于之前的 state等
 */
export function useReducer<S, I, A>(
  reducer: (S, A) => S,
  initialArg: I,
  init?: I => S,
): [S, Dispatch<A>] {
  //获取当前的调度器
  const dispatcher = resolveDispatcher();
  //将useReducer挂载到当前调度器上调用
  return dispatcher.useReducer(reducer, initialArg, init);
}

/**
 * 用法：const inputRef = useRef(null);  
 * <input ref={inputRef} />
 * inputRef.current.focus()
 * 
 * 
 * useRef返回一个可变的 ref 对象，其 .current属性被初始化为传入的参数（initialValue）。
 * 返回的 ref 对象在组件的整个生命周期内保持不变。
 * 可以用来获取节点DOM元素
 * useRef 会在每次渲染时返回同一个 ref 对象。它可以很方便地保存任何可变值。
 * 变更 .current 属性不会引发组件重新渲染
 */
export function useRef<T>(initialValue: T): {| current: T |} {
  const dispatcher = resolveDispatcher();
  return dispatcher.useRef(initialValue);
}


export function useEffect(
  create: () => (() => void) | void,
  deps: Array<mixed> | void | null,
): void {
  const dispatcher = resolveDispatcher();
  return dispatcher.useEffect(create, deps);
}

export function useLayoutEffect(
  create: () => (() => void) | void,
  deps: Array<mixed> | void | null,
): void {
  const dispatcher = resolveDispatcher();
  return dispatcher.useLayoutEffect(create, deps);
}

export function useCallback<T>(
  callback: T,
  deps: Array<mixed> | void | null,
): T {
  const dispatcher = resolveDispatcher();
  return dispatcher.useCallback(callback, deps);
}

export function useMemo<T>(
  create: () => T,
  deps: Array<mixed> | void | null,
): T {
  const dispatcher = resolveDispatcher();
  return dispatcher.useMemo(create, deps);
}

/**
 * useImperativeHandle 可以让你在使用 ref 时自定义暴露给父组件的实例值
 * useImperativeHandle 应当与 forwardRef 一起使用
 * 
 * function FancyInput(props, ref) {
 * const inputRef = useRef();
 * useImperativeHandle(ref, () => ({
 *   focus: () => {
 *     inputRef.current.focus();
 *  }
 *}));
 * return <input ref={inputRef} ... />;
 *}
 * FancyInput = forwardRef(FancyInput);
 */
export function useImperativeHandle<T>(
  ref: {| current: T | null|} | ((inst: T | null) => mixed) | null | void,
    create: () => T,
      deps: Array < mixed > | void | null,
): void {
  const dispatcher = resolveDispatcher();
  return dispatcher.useImperativeHandle(ref, create, deps);
}

/**
 * useDebugValue 可用于在 React 开发者工具中显示自定义 hook 的标签。
 * 只在开发模式下有用
 */
export function useDebugValue<T>(
  value: T,
  formatterFn: ?(value: T) => mixed,
): void {
  if (__DEV__) {
    const dispatcher = resolveDispatcher();
    return dispatcher.useDebugValue(value, formatterFn);
  }
}

export const emptyObject = {};



//以下几个API请看官网详情：https://zh-hans.reactjs.org/docs/concurrent-mode-reference.html#concurrent-mode

/**
 * API解释：https://zh-hans.reactjs.org/docs/concurrent-mode-patterns.html#transitions
 * 
 * 传统的 React 中，当我们变更状态进入一个新屏幕时，经历的是 
 * - Receded -> Skeleton -> Complete 路径。在此之前要实现 
 * - Pending -> Skeleton -> Complete 这种加载路径比较困难。 
 * 
 * 而useTransition 可以改变这个局面
 * useTransition 接收一个超时时间，返回一个startTransition 函数，以及一个 pending
 * const [startTransition, pending] = useTransition({ timeoutMs: 10000 });
 * useTransition 实现过渡状态必须和 Suspense 配合
 * 
 * 但是同一个组件的两个版本的是如何同时存在的呢？
 *
 *这原因就在于 Concurrent 模式本身。我们 之前提到 它有点像在 “branch” 上运行的的一个 state 更新。
 *或者我们可以想象成，当我们把 state 更新包裹在 startTransition 的时候会在“另一个宇宙中”开始渲染，
 *就像科幻电影一样。我们并不能直接看到那个宇宙 — 但是我们能够从那个宇宙探知一些事情正在发生的事情（isPending）。
 *当更新完成的时候，我们的“多个宇宙”合并成一个，我们在屏幕上看到最终的结果
 */
export function useTransition(): [(() => void) => void, boolean] {
  const dispatcher = resolveDispatcher();
  return dispatcher.useTransition();
}

export function useDeferredValue<T>(value: T): T {
  const dispatcher = resolveDispatcher();
  return dispatcher.useDeferredValue(value);
}

export function useOpaqueIdentifier(): OpaqueIDType | void {
  const dispatcher = resolveDispatcher();
  return dispatcher.useOpaqueIdentifier();
}

/**
 * 关于这个API的解释请看下面
 * https://github.com/bvaughn/rfcs/blob/useMutableSource/text/0000-use-mutable-source.md
 */
export function useMutableSource<Source, Snapshot>(
  source: MutableSource<Source>,
  getSnapshot: MutableSourceGetSnapshotFn<Source, Snapshot>,
  subscribe: MutableSourceSubscribeFn<Source, Snapshot>,
): Snapshot {
  const dispatcher = resolveDispatcher();
  return dispatcher.useMutableSource(source, getSnapshot, subscribe);
}

```

