## BFS层序遍历模板

同时适用于图的遍历和树的遍历，图的遍历本质上就是BFS，下面的cur.adj()泛指和cur相邻的节点，visited的主要作用就是为了防止走回头路，如果像一般的二叉树结构不会走回头路那就不需要visited

```javascript
function bfs(start, target) {
    const quene = []  //队列-数据结构的核心
    const visited = new Set()
    quene.push(start)
    visited.add(start)
    let step = 0 // 记录扩散的步数

    while (quene.length) {
        let length = quene.length
        //将当前队列中的所有节点向四周扩散
        for (let i = 0; i < length; i++) {
            let cur = quene.shift()
            //判断是否到达终点
            if (cur === target) {
                return step
            }
            //将cur的相邻节点加入队列
            for (let x of cur.adj()) {
                if (!visited.has(x)) {
                    quene.push(x)
                    visited.add(x)
                }
            }
        }
        //在这里更新步数
        step++
    }
}
```

## 为啥有了BFS可以找最短距离为啥还需要DFS？

BFS可以找到最短距离，但是空间复杂度高，而DFS空间复杂度较低，对DFS来说，最坏情况下就是存储树的高度O(logn)，而BFS每次都会存储二叉树的一层节点，在最坏情况下空间复杂度就是树的最下层结点的数量，为N/2，即O(N)，所以一般在找最短路径的时候用BFS而不用DFS



## BFS应用场景

图或者树的遍历，可以用来求最短距离，比如树的最小高度，图的最短距离



## [725.打开转盘锁](https://leetcode-cn.com/problems/open-the-lock/)

你有一个带有四个圆形拨轮的转盘锁。每个拨轮都有10个数字： '0', '1', '2', '3', '4', '5', '6', '7', '8', '9' 。每个拨轮可以自由旋转：例如把 '9' 变为  '0'，'0' 变为 '9' 。每次旋转都只能旋转一个拨轮的一位数字。

锁的初始数字为 '0000' ，一个代表四个拨轮的数字的字符串。

列表 deadends 包含了一组死亡数字，一旦拨轮的数字和列表里的任何一个元素相同，这个锁将会被永久锁定，无法再被旋转。

字符串 target 代表可以解锁的数字，你需要给出最小的旋转次数，如果无论如何不能解锁，返回 -1。

```javascript
输入：deadends = ["0201","0101","0102","1212","2002"], target = "0202"
输出：6
解释：
可能的移动序列为 "0000" -> "1000" -> "1100" -> "1200" -> "1201" -> "1202" -> "0202"。
注意 "0000" -> "0001" -> "0002" -> "0102" -> "0202" 这样的序列是不能解锁的，
因为当拨动到 "0102" 时这个锁就会被锁定。
```

```javascript
//直接套用BFS模板
var openLock = function (deadends, target) {
    //记录需要跳过的死亡密码
    const deads = new Set([...deadends])
    //记录已经穷举过的密码，防止走回头路
    const visited = new Set()
    const quene = ['0000']
    visited.add('0000')
    //从起点开始广度优先遍历
    let step = 0

    while (quene.length) {
        let len = quene.length
        for (let i = 0; i < len; i++) {
            let cur = quene.shift()
            //判断密码是否合法，不合法就跳过
            if (deads.has(cur)) {
                continue
            }
            //判断密码是否是终点
            if (cur === target) {
                return step
            }

            //将一个节点的未遍历相邻节点加入到队列中
            for (let j = 0; j < 4; j++) {
                let up = plusOne(cur, j)
                if (!visited.has(up)) {
                    visited.add(up)
                    quene.push(up)
                }
                let down = minusOne(cur, j)
                if (!visited.has(down)) {
                    visited.add(down)
                    quene.push(down)
                }
            }
        }
        step++
    }
    return -1
};

function plusOne(str, i) {
    const temp = str.split('')
    if (temp[i] === '9') {
        temp[i] = '0'
    } else {
        temp[i]++
    }
    return temp.join('')
}

function minusOne(str, i) {
    const temp = str.split('')
    if (temp[i] === '0') {
        temp[i] = '9'
    } else {
        temp[i]--
    }
    return temp.join('')
}
```

## 547.省份数量

```
有 n 个城市，其中一些彼此相连，另一些没有相连。如果城市 a 与城市 b 直接相连，且城市 b 与城市 c 直接相连，那么城市 a 与城市 c 间接相连。

省份 是一组直接或间接相连的城市，组内不含其他没有相连的城市。

给你一个 n x n 的矩阵 isConnected ，其中 isConnected[i][j] = 1 表示第 i 个城市和第 j 个城市直接相连，而 isConnected[i][j] = 0 表示二者不直接相连。

返回矩阵中省份的数量。
```

```js
var findCircleNum = function (isConnected) {
    const privinces = isConnected.length
    const visited = new Set()

    let count = 0
    for (let i = 0; i < privinces; i++) {
        if (!visited.has(i)) {
            dfs(visited, privinces, isConnected, i)
            count++
        }
    }
    return count

    function dfs(visited, privinces, isConnected, i) {
        for (let j = 0; j < privinces; j++) {
            if (isConnected[i][j] === 1 && !visited.has(j)) {
                visited.add(j)
                dfs(visited, privinces, isConnected, j)
            }
        }
    }
};
```

