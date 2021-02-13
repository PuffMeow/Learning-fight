## 第三章 报文信息

请求行常见头部：

- Accept
- Accept-Language
- Accept-Encoding
- Connection
- Cache-Control

状态行常见头部:

- Last-modified

- Etag
- Content-Length
- Connection
- Content-Type

常见编码传输方式：

- gzip:  采用Lempel-Ziv算法及32位循环冗余检测(CRC)
- compress: 由UNIX文件压缩程序compress生成的编码格式，采用Lempel-Ziv-Welch算法
- deflate(zlib)：组合使用zlib及deflate压缩算法生成的
- identity(不进行编码)

发送多种数据的多部份对象集合（图片或文本上传时使用)

Content-Type:

- multipart/form-data  //Web表单上传时使用
- multipart/byteranges //206状态码，响应报文包含了多个范围内的内容

## 第四章 状态码

### 类别：

|      | 类别             | 原因短语               |
| ---- | ---------------- | ---------------------- |
| 1xx  | 信息性状态码     | 接收的请求正在处理     |
| 2xx  | 成功状态码       | 请求正常处理完毕       |
| 3xx  | 重定向状态码     | 需要附加操作以完成请求 |
| 4xx  | 客户端错误状态码 | 服务器无法处理请求     |
| 5xx  | 服务端错误状态码 | 服务器处理请求出错     |

### 常见状态码：

#### 2XX成功：

200 OK：客户端发来的请求在服务器被正常处理

204 No Content：服务器接收的请求正常处理，但返回的响应报文中不含实体的主体部分。比如浏览器发出请求处理后，返回204，浏览器的显示页面不刷新。

206 Partial Content：客户端进行了范围请求。响应报文包含有Content-Range指定范围的实体内容

#### 3XX重定向：

301 Moved Permanently：永久重定向，表示请求的资源被分配了新的URI，以后请求资源应使用新的URI

302 Found：临时重定向，表示请求的资源被分配了新的URI，但是只希望用户本次使用新的URI访问

303 See Other：表示请求对应的资源存在另一个URI，应使用GET请求定向获取请求的资源。该状态码和302状态码有相同功能，不同的是它规定客户端应当使用GET请求去获取资源

304 Not Modified：该状态码和协商缓存有关，客户端请求服务器询问资源是否改变，如果服务器资源未改变，返回304状态码，客户端可以直接使用未过期的缓存

307 Temporary Redirect：临时重定向，和302有着相同的含义。302标准会禁止POST变成GET，但是使用时大家并不遵守，而307会遵照浏览器的标准，不会从POST变成GET。

#### 4XX客户端错误：

400 Bad Request：表示请求报文中存在语法错误，需要修改请求的内容后再次发送请求

401 Unauthorized：表示发送的请求需要通过有HTTP认证的认证信息

403 Forbidden：该状态码表示请求资源的访问被服务器拒绝了，没有访问权限。

404 Not Found：表示服务器找不到请求的资源，也可以在服务器拒绝请求且不想说明理由时使用

417 Expectation failed：服务器无法理解客户端的期望做出回应而发生错误

#### 5XX服务器错误：

500 Internal Server Error：表示服务器在执行请求时发生了错误

503 Service Unavailable：服务器暂时处于超负载或进行停机维护，暂时无法处理请求

## 第六章 HTTP首部

### 4种HTTP首部字段类型

- 通用首部字段：请求报文和响应报文都会用到
- 请求首部字段：客户端向服务端发送请求报文时使用的首部
- 响应首部字段：服务端向客户端返回响应报文时使用的首部
- 实体首部字段：针对请求报文和响应报文的实体部分使用的首部

### 通用首部字段

| 首部字段名        | 说明                       |
| ----------------- | -------------------------- |
| Cache-Control     | 控制缓存行为               |
| Connection        | 逐跳首部、连接的管理       |
| Date              | 创建报文的日期             |
| Pragma            | 报文指令                   |
| Trailer           | 报文末端的首部一览         |
| Transfer-Encoding | 指定报文主体的传输编码方式 |
| Upgrade           | 升级为其它协议             |
| Via               | 代理服务器的相关信息       |
| Warning           | 错误通知                   |

### 请求首部字段

| 首部字段名        | 说明                               |
| ----------------- | ---------------------------------- |
| Accept            | 用户代理可处理的媒体类型           |
| Accept-Charset    | 优先字符集                         |
| Accept-Encoding   | 优先编码内容                       |
| Accept-Language   | 优先语言                           |
| Authorization     | Web认证信息                        |
| Expect            | 期待的服务器特定行为               |
| From              | 用户邮箱地址                       |
| Host              | 请求资源所在服务器                 |
| If-Match          | 比较实体标记（E-tag）              |
| If-Modified-Since | 比较资源更新时间                   |
| If-None-Match     | 比较实体标记(和If-Match)相反       |
| If-Range          | 资源未更新时发送实体Byte的范围请求 |
| Range             | 实体的字节范围请求                 |

### 响应首部字段

| 首部字段名         | 说明                         |
| ------------------ | ---------------------------- |
| Accept-Ranges      | 是否接受字节范围请求         |
| Age                | 推算资源创建经过时间         |
| Etag               | 资源匹配信息                 |
| Location           | 令客户端重定向至指定URI      |
| Proxy-Authenticate | 代理服务器对客户端的认证信息 |
| Retry-After        | 再次发起请求时机的要求       |
| Server             | HTTP服务器的安装信息         |
| Vary               | 代理服务器缓存的管理信息     |

### 实体首部字段

| 首部字段名       | 说明                   |
| ---------------- | ---------------------- |
| Allow            | 资源可支持的HTTP方法   |
| Content-Encoding | 实体主体适用的编码方式 |
| Content-Language | 实体主体的自然语言     |
| Content-Length   | 实体主体的大小         |
| Content-Location | 替代对应资源的URI      |
| Content-MD5      | 实体主体的报文摘要     |
| Content-Range    | 实体主体的位置信息     |
| Content-Type     | 实体主体的媒体信息     |
| Expires          | 实体主体过期的日期时间 |
| Last-Modified    | 资源的最后修改时间     |

### HTTP/1.1 通用首部字段

请求报文和响应报文都会用到的首部就叫通用首部

缓存控制：

```
Cache-Control: private, max-age=0, no-cache
```

缓存请求指令有

| 指令           | 参数   | 说明                                                 |
| -------------- | ------ | ---------------------------------------------------- |
| no-cache       | 无     | 强制向源服务器再次验证资源是否过期，不缓存过期的资源 |
| no-store       | 无     | 不缓存请求或响应的任何内容                           |
| max-age= 秒    | 必需   | 响应的最大age值                                      |
| max-stale = 秒 | 可省略 | 接收已过期的响应                                     |
| min-fresh = 秒 | 必需   | 期望在指定时间内的响应仍有效                         |

缓存响应指令有

| 指令            | 参数   | 说明                                                         |
| --------------- | ------ | ------------------------------------------------------------ |
| public          | 无     | 可向任意方提供响应的缓存                                     |
| private         | 可省略 | 仅向特定用户返回响应                                         |
| no-cache        | 可省略 | 缓存前必须先确认有效性，目的是为了防止从缓存中返回过期的资源。 |
| no-store        | 无     | 不缓存请求或响应的任何内容                                   |
| must-revalidate | 无     | 可缓存但必须再向源服务器进行确认                             |
| max-age = 秒    | 必需   | 响应的最大Age值，用了该字段会忽略Expires字段的处理           |
| s-maxage = 秒   | 必须   | 功能和max-age类似，但是它只用于公共缓存服务器。表示响应的最大Age值，用了该字段会忽略Expires首部字段和max-age指令的处理 |