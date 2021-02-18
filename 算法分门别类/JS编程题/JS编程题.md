### 解析一个URL

某次腾讯面试中的题目

```javascript
//对一段Url进行解析
//www.test.com?query=123&type=0
function parseUrl(url) {
  if (!url) return
  let domain = url.split('?')[0]
  let query = url.split('?')[1]
  let obj = {
    domain
  }
  let kV = query.split('&')
  for (let i = 0; i < kV.length; i++) {
    let key = kV[i].split('=')[0]
    let value = kV[i].split('=')[1]
    obj[key] = value
  }
  return obj
}

console.log(parseUrl('www.test.com?query=123&type=0&name=大哥'))
```

### 解析模板字符串

蚂蚁金服笔试题的题目

```javascript
// 输入："${ name }你好，欢迎来到${ company }!", {name: '张三', company: '阿里'}
// 输出：张三你好，欢迎来到阿里!
function template(str, obj) {
  if (!str.length || typeof obj !== "object") return
  Object.keys(obj).forEach(key => {
    str = str.replace(/\$\{\s*\w*\s*\}/, obj[key]);
  });
  return str;
}
console.log(
  template("${ name }你好，欢迎来到${ company }, 我是这里的负责人${master}!",
    { name: '张三', company: '滴滴', master: '王大锤' })
)
```

### 数组扁平化

蚂蚁金服笔试题

```javascript
function checkType(data) {
  if (!data) return
  return Object.prototype.toString.call(data).slice(8, -1).toLowerCase()
}

function flat(array) {
  // first method
  return array.reduce((pre, cur) => {
    return pre.concat(Array.isArray(cur) ? flat(cur) : cur)
  }, [])
}

function flat2(array) {
  //second method
  return array.flat(Infinity)
}

function flat3(array) {
  //third method
  let tempArr = []
  for (let i = 0; i < array.length; i++) {
    if (checkType(array[i]) === "array") {
      tempArr = tempArr.concat(flat3(array[i]))   //笔试的时候这里写错了
    } else {
      tempArr.push(array[i])
    }
  }
  return tempArr
}

let arr = [1, [2, 3, [4, 5]]]
console.log(flat(arr))
console.log(flat2(arr))
console.log(flat3(arr))
```

### 从m到n区间，能被k整除的数的个数

要求优化，蚂蚁金服笔试题，笔试的时候这里没写出来

```javascript
function num(m, n, k) {
  if (!m || !n || !k) return
  let res = 0
  let count = 0   //用来做性能优化的，减少循环次数
  for (let i = m; i <= n; i = count * k) {
    if (i % k === 0) {
      res += 1
    }
    count++
  }
  return res
}
```

### 实现Promise.all

网易有道的题(那时候写有bug)

```javascript
function myAll(promises) {
  let index = 0
  let res = []
  return new Promise((resolve, reject) => {
    promises.forEach((item, i) => {
      Promise.resolve(item).then(res => {
        res[i] = res    //在面试的时候这里写成了res.push(item)，这样会导致返回顺序不一致
        index++
        if (index === promises.length) {
          resolve(res)
        }
      }).catch(err => {
        reject(err)
      })
    })
  })
}
```

### 函数柯里化

```javascript
function curry(fn, currArgs) {
  return function (...args) {
    currArgs && args = args.concat(currArgs);
    if (args.length < fn.length) {
      return curry(fn, args);
    }
    return fn.apply(this, args);
  }
}

function sum(a, b, c, d, e) {
  console.log(a + b + c + d + e);
}

const fn = curry(sum);

fn(1, 2, 3, 4, 5); // 15
fn(1, 2)(3)(4)(5); // 15
fn(1)(2, 3, 4)(5); // 15
fn(1)(2)(3)(4)(5); // 15
```

### 发布订阅EventEmitter

阿里练习生计划里的题

```javascript
class EventEmitter {
  constructor() {
    this.events = {}
  }

  listion(name, fn) {
    if (!name || !fn || typeof fn !== 'function') return
    if (!this.events[name]) {
      this.events[name] = []
    }
    this.events[name].push(fn)
  }

  emit(name, ...args) {
    if (!this.events[name]) return
    this.events[name].forEach(fn => {
      fn.apply(this, args)
    })
  }

  off(name, fn) {
    if (!this.events[name]) return
    if (!fn) {
      delete this.events[name]
      return
    }
    const index = this.events[name].indexOf(fn)
    this.events[name].splice(index, 1)
  }

  once(name, fn) {
    const _once = () => {
      fn.apply(this, arguments)
      this.off(name, _once)
    }
    this.listion(name, _once)
  }
}

const emitter = new EventEmitter()
emitter.listion('test', () => console.log(1))
emitter.listion('test', () => console.log(2))
emitter.listion('test', () => console.log(3))
emitter.emit('test')
emitter.off('test')
emitter.emit('test')
emitter.once('test2', () => console.log(4))
emitter.emit('test2')
```



### 解析依赖的题

微信读书面试题

```javascript
const dependencies = {
	"name": "page.js",
	"dependencies": [{
		"name": "A.js",
		"dependencies": [{
			"name": "C.js",
			"dependencies": [{
				"name": "F.js",
			}]
		}],
	}, {
		"name": "B.js",
		"dependencies": [{
			"name": "D.js",
			"dependencies": [{
				"name": "F.js",
			}]
		}, {
			"name": "E.js",
			"dependencies": [],
		}],
	}]
};

https://rescdn.qqmail.com/assets?files=xxx,xxx,xxx
```

这道题就是一棵树，把树画出来以后，然后就是层序遍历了，就能得到一个数组[page,A,B,C,D,E,F,F]

### 洗牌算法

```javascript
function shuffle(array) {
  const length = array.length;
  let cur = length - 1;
  let random;
  while (cur > -1) {
    random = Math.floor(length * Math.random());
    [array[cur], array[random]] = [array[random], array[cur]];
    cur--;
  }
  return array;
}

let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
console.log(shuffle(arr))
```





### 爬虫在爬取页面前，需要对url列表进行标准化，实现一个处理url列表的函数-对缺少http前缀的url添加前缀，返回的url不能重复

```
例：["nodejs.org", "http://nodejs.org", "http://bytedance.com"] => ["http://nodejs.org", "http://bytedance.com"]  
```

```javascript
function formaturl(urllist) {
  let tempArr = []
  urllist.forEach(item => {
    if (!item.startsWith('http://')) {
      item = 'http://' + item
    }
    tempArr.push(item)
  })

  return [...new Set(tempArr)]
}

console.log(formaturl(["nodejs.org", "http://nodejs.org", "http://bytedance.com"]))
```

