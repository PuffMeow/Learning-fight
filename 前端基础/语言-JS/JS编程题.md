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


//使用正则表达式的方式
function parseUrl(url) {
  let reg = /([^?&=]+)=([^?&=]+)/g
  let obj = {}
  url.replace(reg, (...args) => {
    obj[args[1]] = args[2]
  })
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
    str = str.replace(/\${.*?}/, obj[key]);
  });
  return str;
}
console.log(
  template("${ name }你好，欢迎来到${ company }, 我是这里的负责人${master}!",
    { name: '张三', company: '滴滴', master: '王大锤' })
)
```

### 数组扁平化

可以把嵌套的数组拍平，比如 arr = [1, 2, [3, 4, [5, 6]]]， 这是一个三层嵌套数组，用了数组扁平化之后就会直接变成一层的结构。

arr = [1, 2, 3, 4, 5, 6]

```javascript
function checkType(data) {
  if (!data) return
  return Object.prototype.toString.call(data).slice(8, -1).toLowerCase()
}

function flat(array) {
  // 第一种方式
  return array.reduce((pre, cur) => {
    return pre.concat(Array.isArray(cur) ? flat(cur) : cur)
  }, [])
}

function flat2(array) {
  // 第二种方式
  return array.flat(Infinity)
}

function flat3(array) {
  // 第三种方式
  let tempArr = []
  for (let i = 0; i < array.length; i++) {
    if (checkType(array[i]) === "array") {
      tempArr = tempArr.concat(flat3(array[i])) 
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

### 数组去重及对象数组去重

```javascript
function uniqe(arr) {
  let res = []
  for (let i of arr) {
    if (!res.includes(i)) {
      res.push(i)
    }
  }
  return res
}

function uniqe2(arr) {
  return arr.filter((item, index) => {
    return arr.indexOf(item) === index
  })
}

function uniqe3(arr) {
  return [...new Set(arr)]
}

let arr = [1, 2, 1, 2, 1, 2, 1, 2, 3, 4, 5, 6]
console.log(uniqe(arr))
console.log(uniqe2(arr))
console.log(uniqe3(arr))


//对象数组去重
const objArr = [
  { id: 1, parentid: 0 },
  { id: 2, parentid: 1 },
  { id: 3, parentid: 1 },
  { id: 3, parentid: 1 },
  { id: 4, parentid: 2 },
  { id: 5, parentid: 2 },
  { id: 6, parentid: 0 },
  { id: 6, parentid: 0 },
  { id: 7, parentid: 0 },
];

function objUniqe(arr) {
  const obj = {}
  const res = []
  for (let item of arr) {
    if (!obj[item.id]) {
      res.push(item)
      obj[item.id] = true
    }
  }
  return res
}

function objUniqe2(arr) {
  const obj = {}
  return arr.reduce((prev, item) => {
    obj[item.id] ? '' : obj[item.id] = true && prev.push(item)
    return prev
  }, [])
}

console.log(objUniqe(objArr))
console.log(objUniqe2(objArr))
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

网易有道

```javascript
function myAll(promises) {
  let index = 0
  const res = []
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
    currArgs && (args = args.concat(currArgs))
      
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

fn(1, 2)(3)(4)(5)  //15
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
    function _once(...args) {
      fn.apply(this, args)
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

**这道题当初没做出来是因为没理解这是一棵树的层序遍历**

```javascript
// 按page,A,B,C,D,E,F,F的顺序保存到一个数组
// https://rescdn.qqmail.com/assets?files=xxx,xxx,xxx
let url = 'https://rescdn.qqmail.com/assets?files='

function bfs(dependencies) {
  if (!dependencies) return []
  const quene = [dependencies]
  const res = []
  while (quene.length) {
    const len = quene.length
    for (let i = 0; i < len; i++) {
      let node = quene.shift()
      res.push(node.name)
      if (node.dependencies && node.dependencies.length) {
        for (let j = 0; j < node.dependencies.length; j++) {
          quene.push(node.dependencies[j])
        }
      }
    }
  }
  return res
}
// console.log(bfs(dependencies))
const res = bfs(dependencies)
let str = url + res.toString()
console.log(str)
//得到最后的结果
//https://rescdn.qqmail.com/assets?files=page.js,A.js,B.js,C.js,D.js,E.js,F.js,F.js
```



### 洗牌算法

```javascript
function shuffle(array) {
  const length = array.length;
  let cur = length - 1;
  let random;
  while (cur >= 0) {
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

### 大数相加

```javascript
/**
 * @param {string} num1
 * @param {string} num2
 * @return {string}
 */

// '10086'
// '12345'
var addStrings = function (num1, num2) {
    if (num1 === '0') return num2
    if (num2 === '0') return num1
    let len = Math.max(num1.length, num2.length)
    //先在前面补0
    num1 = num1.padStart(len, 0)
    num2 = num2.padStart(len, 0)

    let up = 0, res = ''
    let i = num1.length - 1
    let j = num2.length - 1
    while (i >= 0 || j >= 0) {
        let num = (+num1[i]) + (+num2[j])
        if (up) num += up
        if (num > 9) {
            up = Math.floor(num / 10)
            let temp = num % 10
            res += temp
        } else {
            up = 0
            res += num
        }
        i--
        j--
    }
    res = res.split('').reverse().join('')
    return up === 1 ? 1 + res : res
};
```

### 字符串转base64

```js
function encode(str){
    // 对字符串进行编码
    let encode = encodeURI(str);
    // 对编码的字符串转化base64
    let base64 = btoa(encode);
    return base64;
}

// base64转字符串
function decode(base64){
    // 对base64转编码
    let decode = atob(base64);
    // 编码转字符串
    let str = decodeURI(decode);
    return str;
}

//获取userAgent转成base64后并找出里面出现次数最多的字符
let base64 = enCode(navigator.userAgent)
const map = new Map()
for(let char of base64) {
    ...
}
```

### 千分位实现

正则表达式

1、正则表达式 \d{1,3}(?=(\d{3})+$)  表示前面有1~3个数字，后面的至少由一组3个数字结尾

2、?=表示正向引用，可以作为匹配的条件，但匹配到的内容不获取，并且作为下一次查询的开始

3、$& 表示与正则表达式相匹配的内容

```js
function solution(num) {
  let _num = String(num)
  let reg = /\d{1,3}(?=(\d{3})+$)/g

  return _num.replace(reg, '$&,')
}

console.log(solution(12345678))
```

js实现

```js
function solution(num) {
  let _num = String(num)
  let str = ''

  for (let i = _num.length - 1, j = 1; i >= 0; i--, j++) {
    if (i !== 0 && j % 3 === 0) {
      //每隔三位加逗号，过滤正好在第一个数字的情况
      str += _num[i] + ','
      continue
    }
    str += _num[i] //倒着累加数字
  }

  return str.split('').reverse().join('')
}

console.log(solution(1234567810))
```

### JSON.stringify()实现

```js
function jsonStringify(data) {
  let type = typeof data;

  if(type !== 'object') {
    let result = data;
    //data 可能是基础数据类型的情况在这里处理
    if (Number.isNaN(data) || data === Infinity) {
       //NaN 和 Infinity 序列化返回 "null"
       result = "null";
    } else if (type === 'function' || type === 'undefined' || type === 'symbol') {
      // 由于 function 序列化返回 undefined，因此和 undefined、symbol 一起处理
       return undefined;
    } else if (type === 'string') {
       result = '"' + data + '"';
    }
    return String(result);
  } else if (type === 'object') {
     if (data === null) {
        return "null"  // 第01讲有讲过 typeof null 为'object'的特殊情况
     } else if (data.toJSON && typeof data.toJSON === 'function') {
        return jsonStringify(data.toJSON());
     } else if (data instanceof Array) {
        let result = [];
        //如果是数组，那么数组里面的每一项类型又有可能是多样的
        data.forEach((item, index) => {
        if (typeof item === 'undefined' || typeof item === 'function' || typeof item === 'symbol') {
               result[index] = "null";
           } else {
               result[index] = jsonStringify(item);
           }
         });
         result = "[" + result + "]";
         return result.replace(/'/g, '"');
      } else {
         // 处理普通对象
         let result = [];
         Object.keys(data).forEach((item, index) => {
            if (typeof item !== 'symbol') {
              //key 如果是 symbol 对象，忽略
              if (data[item] !== undefined && typeof data[item] !== 'function' && typeof data[item] !== 'symbol') {
                //键值如果是 undefined、function、symbol 为属性值，忽略
                result.push('"' + item + '"' + ":" + jsonStringify(data[item]));
              }
            }
         });
         return ("{" + result + "}").replace(/'/g, '"');
        }
    }
}
```

### 正则表达式字符串转驼峰

```js
'get-element-by-id'.replace(/-(\w)/g, ($,$1)=> $1.toUpperCase())
//"getElementById"
```

### 去除字符串内空格的方式

```javascript
str.replace(/\s*/g,""); //去除字符串内所有的空格
str.replace(/^\s*|\s*$/g,""); //去除字符串内两头的空格
str.replace(/^\s*/,""); //去除字符串内左侧的空格
str.replace(/(\s*$)/g,""); //去除字符串内右侧的空格
```

### 用递归算法实现，数组长度为5且元素的随机数在2-32间不重复的值

```js
const arr = new Array(5)
function randomNum(n) {
  if (n < 0) return
  const num = Math.floor(Math.random() * 31 + 2)
  if (arr.includes(num)) {
    return randomNum(n)
  }
  arr[n] = num
  return randomNum(n - 1)
}

randomNum(arr.length - 1)
```

### 比较两个版本号

如果 version1 > version2 返回 1，如果 version1 < version2 返回 -1， 除此之外返回 0。

```js
function version(v1, v2) {
  let arr1 = v1.split('.'), arr2 = v2.split('.')
  let len = Math.max(arr1.length, arr2.length)

  while (arr1.length < len) {
    arr1.push('0')
  }
    
  while (arr2.length < len) {
    arr2.push('0')
  }

  for (let i = 0; i < len; i++) {
    let num1 = Number(arr1[i]), num2 = Number(arr2[i])
    if (num1 > num2) {
      return 1
    }
    if (num1 < num2) {
      return -1
    }
  }
  return 0
}

console.log(version("0.1", "1.1"));
console.log(version("1.0.1", "1"));
console.log(version("7.5.2.4", "7.5.3"));
console.log(version("1.01", "1.001"));
console.log(version("1.0", "1.0.0"));
```

### 串行执行Promise请求

```js
async function solution(promises) {
  for (let p of promises) {
    await p
  }
}

function sleep(time, str) {
  return new Promise(resolve => {
    setTimeout(() => {
      console.log(str)
      resolve()
    }, time);
  })
}

const promises = [sleep(1000, 1), sleep(2000, 2), sleep(3000, 3)]
console.log(solution(promises))
```

### 实现一个lodash的get方法

```js
function deepGet(object, path, defaultValue = undefined) {
  return (!Array.isArray(path) ? path.replace(/\[/g, '.').replace(/\]/g, '').split('.') : path)
    .reduce((obj, key) => (obj || {})[key], object) || defaultValue
}


console.log(deepGet(obj, 'a[0].b.c'))  //哈哈哈
console.log(deepGet(obj, ['a', '0', 'b', 'c'])) //哈哈哈
```

### ip地址判断

ip地址格式：(1~255).(0~255).(0~255).(0~255)

```js
function ipCheck(ip) {
  if (!ip || typeof ip !== 'string') return false
  if (ip.length < 7 || ip.length > 15) {
    return false
  }
  if (ip.charAt(0) === '.' || ip.charAt(ip.length - 1) === '.') {
    return false
  }

  const arr = ip.split('.')
  if (arr.length !== 4) {
    return false
  }

  for (let i = 0; i < arr.length; i++) {
    if (arr[i].length > 1 && arr[i].charAt(0) === '0') {
      return false
    }
    for (let j = 0; j < arr[i].length; j++) {
      if (arr[i].charAt(j) < '0' || arr[i].charAt(j) > '9') {
        return false
      }
    }

    let num = parseInt(arr[i])
    if (i === 0) {
      if (num < 1 || num > 255) {
        return false
      }
    } else {
      if (num < 0 || num > 255) {
        return false
      }
    }
  }

  return true
}

console.log(ipCheck('255.255.255.255'))
console.log(ipCheck('1.02.003.014'))
console.log(ipCheck('1a.2.3.4'))
console.log(ipCheck('-1.1.2.3'))
console.log(ipCheck('123.123.123.123'))
console.log(ipCheck('0.0.0.0'))
console.log(ipCheck('123456789012345678 '))
```

### 手写Object.create

```js
Object.prototype.myCreate = function (obj) {
  function Fn() { }
  Fn.prototype = obj
  return new Fn()
}
```

### 手写bind

```js
Function.prototype.myBind = function (thisArg, ...args) {
  let self = this

  const bindFn = function (..._args) {
    self.apply(this instanceof self ? this : thisArg, args.concat(_args))
  }

  bindFn.prototype = Object.create(self.prototype)

  return bindFn
}
```

### 手写深拷贝

```js
function deepClone(source, cache = new Map()) {
  if (cache.has(source)) {
    return cache.get(source)
  }
  if (source instanceof Object) {
    let target
    if (source instanceof Array) {
      target = []
    } else if (source instanceof Function) {
      target = function (...args) {
        return source.apply(this, args)
      }
    } else if (source instanceof RegExp) {
      target = new RegExp(source)
    } else if (source instanceof Date) {
      target = new Date(source)
    } else {
      target = {}
    }

    cache.set(source, target)
    for (let key in source) {
      if (source.hasOwnProperty(key)) {
        target[key] = deepClone(source[key], cache)
      }
    }

    return target
  } else {
    return source
  }
}
```

