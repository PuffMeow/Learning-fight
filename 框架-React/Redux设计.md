## 了解Redux

全局存储仓库：Redux 主要由 3 部分组成：Store、Reducer 和 Action。

- Store：它是一个单一的数据源，而且是只读的。

- Action 人如其名，是“动作”的意思，它是对变化的描述。

- Reducer 是一个函数，它负责对变化进行分发和处理，最终将新的数据返回给 Store。

Store、Action 和 Reducer 三者紧密配合，便形成了 Redux 独树一帜的工作流

![Drawing 13.png](https://s0.lgstatic.com/i/image/M00/7E/C6/CgqCHl_PVh-ATfOGAAB089LdYcY341.png)