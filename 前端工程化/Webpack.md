## Webpakc是啥?

webpack是一个静态的模块化打包工具，为现代的JavaScript应用程序 ，官方文档是[Webpack文档](https://webpack.docschina.org/ )

- 打包(bundler)：webpack可以将帮助我们进行打包，所以它是一个打包工具 
- 静态的(static)：这样表述的原因是我们最终可以将代码打包成最终的静态资源（部署到静态服务器） 
- 模块化(module)：webpack默认支持各种模块化开发，ES Module、CommonJS、AMD等； 
- 现代化(modern)：现代前端开发面临各种各样的问题，催生了webpack的出现和发展 

## Webpack能干啥？

在开发vue、react、angular等项目的过程中我们需要一些特殊的配置：比如给某些目录结构起别名，让我们的项目支持sass、less等预处理器，希望在项目中添加TypeScript的支持 。另外，Webpack支持定制化的配置，比如安装性能分析工具、使用gzip压缩代码、引用cdn的资源，公共代码抽取等，有些时候我们甚至也需要编写自己的Plugin和Loader 

## 环境配置

Webpack的运行依赖Node环境的 ，电脑必须先安装Node环境，webpack的安装，需要安装webpack和webpack-cli。

```
npm install webpack webpack-cli –g # 全局安装
npm install webpack webpack-cli –D # 局部安装
```

