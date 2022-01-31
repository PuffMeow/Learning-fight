// http://test.qq.com/admin?query=123&page=index.html
function test(UrlStr) {
  let reg = /(http|https):\/\/(.*?)\.qq\.com\??(.*?)/
  return reg.test(UrlStr)
  // if (UrlStr.startsWith('http://') || UrlStr.startsWith('https://')) {
  //   if (!UrlStr.includes('qq.com')) return false
  //   let arr = UrlStr.split('.')
  //   console.log(arr)
  //   let indexQ = arr.indexOf('qq')
  //   let indexC = arr.indexOf('com')
  //   if (indexQ + 1 !== indexC) return false
  // }
  // return true
}
console.log(test('http://test.qq.com/admin?query=123&page=index.html'))
