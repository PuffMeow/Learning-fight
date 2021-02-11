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

- gzip
- compress
- deflate(zlib)
- identity(不进行编码)

发送多种数据的多部份对象集合（图片或文本上传时使用)

Content-Type:

- multipart/form-data  //Web表单上传时使用
- multipart/byteranges //206状态码，响应报文包含了多个范围内的内容

## 第四章 状态码

