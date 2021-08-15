### 屏幕参数获取

#### 获取整块屏幕的大小(桌面宽高的大小)

```js
const W  =  window.screen.width 
const H  =  window.screen.height
```

#### 获取浏览器宽高

```js
const W = window.outerWidth;
const H = window.outerHeight;
```

#### 获取当前窗口宽高(浏览器视口宽高，不包括控制台导航栏等）

```js
const W = window.innerWidth;
const H = window.innerHeight;
```

#### 获取元素布局宽高(包括border，不包括margin)

```js
const W = element.offsetWidth;
const H = element.offsetHeight;
```

#### 获取元素内容宽高(溢出滚动可见范围的也算)

```js
const W = element.scrollWidth;
const H = element.scrollHeight;
```

#### 获取滚动后被隐藏页面的宽高

```js
const H = document.documentElement.scrollTop;
const W = document.documentElement.scrollLeft
```

#### 获取元素距离顶部和左边距离

```js
const top = element.offsetTop;
const left = element.offsetLeft
```

