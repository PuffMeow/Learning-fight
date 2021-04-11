### CVTE

#### 笔试算法

```js
//第一道算法题
//比如s1->ABCD有如下四种情况
// 判断 s1:ABCD  DABC  CDAB BCDA 
// s2:DAB
// 判断s2是否可以和s1中某一种情况完全重合
function isTrue(s1, s2) {
  let start = 1
  let count = s1.length
  while (s1.indexOf(s2) === -1) {
    let tempArr = s1.split('')
    tempArr.unshift(tempArr.pop())
    s1 = tempArr.join('')
    start += 1
    if (start === count) {
      return false
    }
  }
  return true
}
console.log(isTrue('WAB', 'BW'))


// 第二道算法题
// 实现一个函数handleRgba将rgba转化成十六进制颜色
// 比如:rgba(255, 255, 255, 0.4)
// 转化成16进制， 返回['#ffffff', 0.4]
function handleRgba(rgbaStr) {
  let res = rgbaStr.replace(/[rgba|RGBA\(\)]/ig, '')
  let tempArr = res.split(',')
  let opacity = tempArr[3]
  let color = '#'
  for (let i = 0; i < 3; i++) {
    let hex = Number(tempArr[i]).toString(16)
    if (hex.length < 2) {
      hex = '0' + hex
    }
    color += hex
  }
  return [color, opacity]
}
// let res = handleRgba('rgba(2, 1, 255, 0.4)')
// console.log(res)
```

#### 面试算法

Leetcode跳跃游戏原题:

```
给定一个非负整数数组 nums ，你最初位于数组的 第一个下标 。
数组中的每个元素代表你在该位置可以跳跃的最大长度。
判断你是否能够到达最后一个下标。

输入：nums = [2,3,1,1,4]
输出：true
解释：可以先跳 1 步，从下标 0 到达下标 1, 然后再从下标 1 跳 3 步到达最后一个下标。

输入：nums = [3,2,1,0,4]
输出：false
解释：无论怎样，总会到达下标为 3 的位置。但该下标的最大跳跃长度是 0 ， 所以永远不可能到达最后一个下标。
```

```js
/**
 * @param {number[]} nums
 * @return {boolean}
 */
var canJump = function (nums) {
    const length = nums.length
    let maxStep = 0
    for (let i = 0; i < nums.length; i++) {
        if (maxStep < i) return false
        maxStep = Math.max(maxStep, i + nums[i])
        if (maxStep >= length - 1) return true
    }
};
```

