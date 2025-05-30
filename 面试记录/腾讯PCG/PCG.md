---
# 主题列表：juejin, github, smartblue, cyanosis, channing-cyan, fancy, hydrogen, condensed-night-purple, greenwillow, v-green, vue-pro, healer-readable
# 贡献主题：https://github.com/xitu/juejin-markdown-themes
theme: smartblue
highlight:
---

### 开头
这是一场线下面试，所以是面对面交流的，全程都是口述，没写代码（前一天被五道手写（真手写）算法题给虐的不要不要的），面试官好像是个工作了五年的🐧高级前端，不过人看着还是很友善滴。

接下来就是开场了👇

#### **先来一个自我介绍**
 这里我跟面试官巴拉巴拉了一些，然后说到我平时喜欢看各种程序员网站，比如说掘金，CSDN，牛客网。。。然后面试官好像对这一块特别感兴趣？就问我掘金ID是啥，然后我就当场打开了手机掘金，给他看了看我的账号，然后他说：看来你平时都是看别人的文章比较多，我说我也有自己写一点点文章（虽然都是近期才开始写）。他随手打开了我的一篇博客看了看，应该看的是这个：[前端面试题总结](https://juejin.cn/post/6893856813247266823)，看了几眼之后就问下一个问题了。

 #### **你学前端多久了？ 为什么要学前端?**
 俺说：我学前端到现在一年多一点吧，至于为啥要学前端，主要还是因为身边同学的影响，以及自己的一些觉悟。因为大一暑假的时候去美国了，然后在美国做一些底层的工作（学校的某个赴美实习项目），做完这个工作之后我就感觉不学一门技术可能真的就要out了(自己不甘心于做底层工作)，那时候因为大一的时候学过C++，我就去学了一下虚幻4引擎，做了一个很简单的游戏Demo，后面实在因为上手门槛太高，我就被劝退了。再到后面因为身边刚好有个好朋友在学前端，他一直给我灌输前端的思想。十月份从美国回来之后，我就开始了前端的学习直到现在了，然后他点点头说了声了解，就开始了下一个问题。

 #### **关于git、代码规范、代码测试**
 我说对git只是了解，仅限于简单的提交，代码规范我只知道eslint，代码测试的话就是只了解Jest和Mocha（因为没接触到过大型多人合作项目，所以这些都是仅限于了解到过），然后问我eslint是怎么检查代码的？ 我回答说在编译阶段的时候检查的，还有静态检查的时候代码不规范会出现红线警告（关于eslint我只是知道在使用脚手架的时候会自动安装）。。。菜了菜了，有没有各位靓仔靓女知道这一块该怎么回答？欢迎在评论区讨论。  

 #### **说一下浏览器的渲染过程**
 一开始我还想回答从输入URL地址到渲染的过程，但是面试官说让我只详细解释从数据返回后浏览器如何渲染页面。我只说出了一个解析DOM和CSSOM还有提了一手回流和重绘。。。然后面试官问我DOM树结构和CSSOM的结构是怎么样的(那时候我回答CSSOM的结构是扁平化的？？？？？？？？我现在想想人都傻了)，这一块我承认我真的很不熟悉！！！这里再做一下总结：

 我们先来看看DOM结构和CSSOM结构大概的模样，可见它们都是树形结构

![](https://p1-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/b6172f6d8a5a49db91e96ab500d150a1~tplv-k3u1fbpfcp-watermark.image)

而关于浏览器的渲染过程大概是这样子的

__1、__ 浏览器会将HTML解析成一个DOM树，DOM树的构建过程是一个深度遍历过程，当前节点的所有子节点都构建好后才会去构建当前节点的下一个兄弟节点

__2、__ 将CSS解析成CSS规则树。CSS中各节点之间同样拥有层级关系，如父子关系、兄弟关系等，彼此相连，构成CSSOM树。

__3、__ 根据DOM树和CSS来构造渲染树，渲染树不等于DOM树，像header和display：none；这种没有具体内容的东西就不在渲染树中。DOM树、CSSOM树和渲染树这三者并行构建，它们是一边加载，一边解析，一边渲染的。

__4、__ 根据render树，浏览器可以计算出网页中有哪些节点，各节点的CSS以及从属关系，然后可以计算出每个节点在屏幕中的位置

__5、__ 遍历render树进行绘制页面中的各元素

__6、__ 这其中可能还会发生回流和重绘，这会影响到页面的渲染性能

大概过程就是这样的
![](https://p3-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/ec68fc11ff214ef78ecef4a9b65c346c~tplv-k3u1fbpfcp-watermark.image)

 #### **关于localStorage**
 现在有A站点和B站点，在B站点存储了localStorage，A站点能不能访问？ 我真的人傻了，那时候一直坚持的说可以访问得到，还和面试官扯了好几分钟，然后面试官说比如百度也可以访问得到你自己网站里的localStorage吗？我说应该可以。。。然后就开始跟我说关于安全这一块的问题，你是怎么做的，，，然后扯着扯着我自己都扯不下去了，最后说了句不太清楚了。平时还真的没了解过这个。回来查了才知道，为了保证安全，浏览器是不允许localStorage跨域访问的。。。。。。我真的。。。菜了菜了。

 #### **说三个ES6的特性**
 这里我说了`let`、 `const` 、和 `Promise`

 关于let和const最主要的就是拥有了块级作用域，而且它们不像var一样会变量提升，提了一下暂时性死区，还有不能重复声明变量。另外const就是声明一个常量，赋值后就不能重新修改，然后面试官就问，比如用const声明一个数组，数组里的值能不能修改？ 因为数组是引用数据类型，所以他里面的内容还是可以修改的。接下来就问 `[0] === [0]` 这个是true 还是 false？ 那肯定是false咯，还是引用数据类型的问题，他们指向的是不同的内存地址。接下来就是说Promise解决了什么问题：`大概就是回调地狱，让代码更加扁平化`，然后就说Promise也有缺点，就是一旦回调完成，它的内部状态就没法再改变，就是由`pending变为fullfilled和rejected的问题`。接下来就问我如何实现`Promise的串行`，我就说了个大概是用async await实现，比如说await第一个请求，得到结果后拿这个结果去await第二个请求，再继续...，然后面试官说，如果我给你的是一个数组，让你去串行执行数组里的url，你怎么实现？ 刚好我在自己做的项目中遇到过这问题，我就说了我当时是用reduce来实现的，基本思路大概就是这样的。
 ```javascript
urlArr.reduce(async (prev, ele) => {
	await prev;
    return ajax(ele)
}, Promise.resolve())
 ```

#### **聊聊计算机网络吧，http和https的区别**
大概就是这些http是80端口，https是443端口，还有http明文传输，https密文传输，https在传输层和应用层之间多了一层SSL层。然后面试官就问我https是对称加密还是非对称加密？我说两个都有用到，让我具体解释一番。。。然后我之前有看到过这个，但是到这里卡壳了，支支吾吾随便说了一些，感觉没回答好，总的来说我还是菜了啊啊啊。然后就过了到下一题。
其实当初自己有做过这一块的总结的，但自己可能对计网还不太熟悉，总是记不住。。这里我再做一下总结：
**HTTPS 在内容传输的加密上使用的是对称加密，非对称加密只作用在证书验证阶段。使用对称加密是为了在传输过程中传输效率更高，而非对称加密每次都要加密解密就会消耗大量的时间**

关于计网更加具体的内容可以看看我写的这篇文章[计网之从传输层到应用层](https://juejin.cn/post/6901610715527217159)

#### **说一下http的状态码**
就把常见那几个从200到500的状态码说了一下,我回答的时候好像把301和302说反了，😄这就有点尴尬了

👇这里再做下总结:

**1XX：** 信息，服务器收到请求，需要请求者继续执行操作（不常用）

**2XX：** 成功，操作被成功接收并处理 。常见：**200-OK（正常返回）**、**201-Created（成功创建）**、**204 - No Content（网页未更新时，服务器正常处理但没返回内容）**

**3XX：** 重定向，需要进一步的操作以完成请求 。常见：**301 -  Moved Permanently （永久重定向， 请求的资源已被永久的移动到新URI，返回信息会包括新的URI，浏览器会自动定向到新URI ）**、**302 -  Found（临时重定向，资源临时被移动，客户端应继续使用原有URI ）**、**304 -  Not Modified（ 未修改。请求的资源未修改，不返回任何资源。客户端通常会缓存访问过的资源）**

**4XX：** 客户端错误状态 ，表示服务器无法处理请求。常见：**400 -  Bad Request （客户端请求语法错误，服务器无法理解 ）**、**401 - Unauthorized（未授权，一般是需要先登录授权）**、**403 -  Forbidden （ 服务器理解请求，但是拒绝执行 ，一般用于某些页面权限的设置）**、**404 - Not Found（资源访问不到）**

**5XX：** 服务器错误，处理请求的过程中发生了错误 。常见：**500（ 服务器内部错误，无法完成请求 ）**、 **503（超载或者在维护，暂时无法处理请求）**



#### **接下来就是说一下快速排序**
我就说了一下使用数组的方式和使用下标的方式实现快排，然后就说了一下快排的时间复杂度和空间复杂度。
平均时间复杂度是O(nlogn)，使用数组的时候空间复杂度是O(n)，使用下标的时候空间复杂度是O(1)，然后面试官问我最坏情况:我就说了一下最坏情况时间复杂度是O(n^2),然后问我啥时候会出现最坏情况，然后我就懵逼了？？？这个确实没了解过（大二的时候学过数据结构，但是现在已经忘得差不多了，最近正在狂补算法中。。。）

这里再次总结：

**1、** 已排好序的时候

**2、** 在分解时每次选取的基准值为**最小值**

**3、** 在分解时每次选取的基准值为**最大值**

所以为了尽量避免不会出现最差情况，我们就得求数组的中值，然后选取序列的中值作为基准值。

这里把两种实现思路的代码给出
```javascript
//这种方法消耗额外的空间，而且效率上比下面那种要差，但是写起来更方便，易于理解
function quickSort1(arr) {
  if (arr.length < 2) return arr

  let leftArr = []
  let rightArr = []
  let pivot = arr[0]

  for (let i = 1; i < arr.length; i++) {
    if (arr[i] < pivot) {
      leftArr.push(arr[i])
    } else {
      rightArr.push(arr[i])
    }
  }

  return quickSort1(leftArr).concat([pivot], quickSort1(rightArr))
}
```

我们再看看第二种快排的实现方式
```javascript
function quickSort2(arr, n) {
  //对arr[l...r]这部分进行快排
  function _quickSort(arr, l, r) {
    if (l >= r) return

    //使得arr[l...p-1] < arr[p]； arr[p+1...r] > arr[p]
    //返回p，这个p就是作为基准值。
    function _partition(arr, l, r) {
      //使用数组第一个元素作为基准值的标志点
      let v = arr[l]
      let j = l
      //arr[l+1...r] < v; arr[j+1...i) > v
      for (let i = l + 1; i <= r; i++) {
        if (arr[i] < v) {
          [arr[j + 1], arr[i]] = [arr[i], arr[j + 1]]
          j++
        }
      }
      //遍历完之后j的位置和开始标志点的位置交换，交换完j的位置就是标志点的位置
      [arr[l], arr[j]] = [arr[j], arr[l]]

      //将标志点的位置返回
      return j
    }

    let p = _partition(arr, l, r)
    //对下一轮基准值左边的和右边的数据继续使用快排递归
    _quickSort(arr, l, p - 1)
    _quickSort(arr, p + 1, r)
  }

  _quickSort(arr, 0, n - 1)
  return arr
}
```

接下来我们再看看这俩种方法**对随机生成的一千万个数字进行快速排序**

生成随机数组的方法

```javascript
//生成n个元素随机数组，随即范围在[rangeL,rangeR]
function gennerateRandomArr(n, rangeL, rangeR) {
  if (rangeR < rangeL) {
    throw new Error('右边不能小于左边')
  }
  let arr = new Array(n)
  //生成范围内的随机数
  for (let i = 0; i < n; i++) {
    arr[i] = Math.floor(Math.random() * (rangeR - rangeL + 1) + rangeL)
  }
  return arr
}
```

下面的结果已经执行多次，生成的数组都是随机的，多次测试出来的结果都是大概在这个值。

第一种用额外数组的方式：
![](https://p9-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/a9b954251840495d816b8606243985e4~tplv-k3u1fbpfcp-watermark.image)

第二种用下标的方式：
![](https://p9-juejin.byteimg.com/tos-cn-i-k3u1fbpfcp/fc567cabf057497f8e18634238237bd0~tplv-k3u1fbpfcp-watermark.image)

嗯。。。这题就差不多到这了

#### **100层楼用两个杯子找杯子摔碎的临界点**
面试官说，用最少的次数来找出这个临界点，杯子可以复用。我炸一听，这不就是二分查找法吗？？？然后就跟面试官说了用二分查找应该可以吧，然后面试官说只有两个杯子，从50层摔碎，然后到25层摔碎杯子就没了。。。然后问问我有没有其它的方法，，我又懵逼了！最后默默说了句暂时想不出来了。。。然后就结束了

这里放个参考链接，最好的情况应该就是14次找出这个临界点，具体可以看看下面的文章，讲的很详细

[Google经典面试题《100层楼测试两个鸡蛋》剖析](https://zhuanlan.zhihu.com/p/39816314)


#### **你有没有啥要问我的？**
随便问了一下它们部门有在使用什么框架吗，然后面试官回答我之后就没了，说了声谢谢静静离场。。。


### 总结
这次面试可以说是我自己人生中第一场正式的面试，问的还是比较有广度和深度的，这次面试应该大概率已经没了，但是我们还是要学会及时总结。经过这次面试之后发现自己很多地方知识点还不够牢固，所以回来做个复盘，希望在日后的面试中继续加油吧，**勇往直前，越战越勇！！！** **大家对题目中的这些题有什么自己的见解也欢迎在评论中讨论讨论，共勉！！！**  