# 看完这篇文章你能了解到什么?

- **了解常见的 Docker 知识**

- **利用 Docker 快速跨平台部署后端( Node.js + MongoDB + Redis + Nginx ) 项目**
- 一些常见的 Linux 系统操作
- 编写 Dockerfile 文件
- 编写 docker-compose 文件
- 编写 一些常见的 nginx 配置文件

# PS

这里主要讲的是利用容器化的方式去部署项目，容器化部署的好处有很多哈，比如容器可以很方便从一台电脑迁移到另一台电脑。

如果想要了解传统的实机部署方式，可以看看我的另一篇文章 [手摸手教你如何从零一步一步部署一个前端项目到远程服务器](https://juejin.cn/post/6976114620897427464#heading-0)

# Docker 是什么？

简单一句话就是一个应用打包、分发、部署的工具，可以把它理解为一个轻量的虚拟机，但是是以容器的方式运行的。

支持各种系统 Linux，MacOS，Windows等。可以使用容器化部署以降低项目在不同的平台之间进行部署的成本。

再也不会出现 **“怎么在我的电脑能运行，到了服务器就运行不了”** 这种情况。

# Docker 基本概念

使用 Docker 前需要先了解这几个基本概念

- 镜像（image）
- 容器（container）
- 仓库（repository）

获取镜像的方式可以通过 Dockerfile 文件创建，也可以通过 dockerHub 仓库下载

- Docker 中**镜像**和**容器**的关系就像 **类** 与 **实例** 的关系
- 镜像可以通过 Dockerfile 文件来生成，容器通过镜像来创建

# Docker 使用国内镜像加速

Linux 系统

```
vim /etc/docker/daemon.json
```

Windows 系统，找到 daemon.json 文件并打开修改

`C:\Users\<你的用户名>\.docker\daemon.json` 文件

然后修改里面的 `registry-mirrors` 字段，可以添加多个源地址。可以在下载镜像的时候加速

```json
{
  "builder": {
    "gc": {
      "defaultKeepStorage": "20GB",
      "enabled": true
    }
  },
  "experimental": false,
  "features": {
    "buildkit": true
  },
  "registry-mirrors": [
    "https://registry.docker-cn.com"
  ]
}
```

- Docker中国官方：https://registry.docker-cn.com

- 中科大：https://docker.mirrors.ustc.edu.cn

- 网易：http://hub-mirror.c.163.com

# Hello world

Docker 允许你在容器内运行应用程序， 使用 **docker run** 命令来在容器内运行一个应用程序

```
docker run ubuntu:15.10 /bin/echo "Hello world"
```

安装一个 ubuntu 15.10版本的容器，并在容器中输出hello world，如果本地不存在该容器就会从远程仓库下载，相当于你在 Docker 容器内安装了一个 ubuntu 系统的虚拟环境，可以在里面执行各种Linux的指令。

## 交互式容器

相当于可以在容器虚拟环境中打开控制台。

放在镜像名后的是命令，这里我们希望有个交互式 Shell，因此用的是 /bin/bash

```bash
docker run -i -t ubuntu:15.10 /bin/bash
# 或者
docker run -it ubuntu:15.10 /bin/bash
```

- **-t:** 在新容器内指定一个伪终端或终端。
- **-i:** 允许你对容器内的标准输入 (STDIN) 进行交互。
- **-d：**让容器在后台运行，输入后不进入交互模式 
- **-p**：表示暴露端口
- 在控制台中输入exit退出

如果输入了-d 参数，会让容器后台运行，那么怎么进入到容器中呢？

一个是`docker attach 容器ID `命令，如果从这个命令进入到容器中后，再输入`exit`会把整个容器也退出，不再维持后台运行的状态。

另一个是`docker exec -it 容器ID /bin/bash`命令回到容器中，执行这个命令在容器中输入`exit`不会把整个容器也退出，容器仍将维持后台运行状态。

## Docker状态

输入指令`docker ps -a`可以查看所有的容器。

![image-20211014233025791](C:\Users\JqWang\AppData\Roaming\Typora\typora-user-images\image-20211014233025791.png)

如果要恢复一个已经停止的容器可以输入`docker start 容器ID`，同样的，想要停止一个容器可以输入`docker stop 容器ID`。

另外还有`docker restart 容器ID` 命令用于重启容器

| CONTAINER ID | IMAGE      | COMMAND              | CREATED        | STATUS   |
| ------------ | ---------- | -------------------- | -------------- | -------- |
| 容器 ID      | 使用的镜像 | 启动容器时运行的命令 | 容器的创建时间 | 容器状态 |

容器的状态有7种：

- created（已创建）
- restarting（重启中）
- running 或 Up（运行中）
- removing（迁移中）
- paused（暂停）
- exited（停止）
- dead（死亡）

## 删除一个容器

```
docker rm -f 容器ID
```

## 清理列表中所有终止状态的容器

```
docker container prune
```

## 查看镜像

导入之后可以输入`docker images`来查看所有的容器镜像列表

![image-20211015233846053](C:\Users\JqWang\AppData\Roaming\Typora\typora-user-images\image-20211015233846053.png)

| **REPOSITORY** | **TAG**    | **IMAGE ID** | **CREATED**  | **SIZE** |
| -------------- | ---------- | ------------ | ------------ | -------- |
| 镜像的仓库源   | 镜像的标签 | 镜像ID       | 镜像创建时间 | 镜像大小 |

## 删除镜像

```
docker rmi 镜像仓库源
# 比如说要删除上面的test v0.0.1版本
docker rmi test:v0.0.1
```

## 获取镜像

当使用`docker run `命令来运行本地不存在的镜像时会自动下载镜像，但也可以使用`docker pull`命令来提前下载

```
docker pull 镜像名:版本号
# 例如
docker pull ubuntu:15.10
```

## 查找镜像

直接去官网找吧 **https://hub.docker.com/**

## 安装软件

比如在上面的官网中查找一个 redis 镜像，然后安装最新版的

```bash
docker run -d -p 6379:6379 --name redis redis:latest
```

-p 命令后面接的端口号指， 宿主机端口号:容器内端口号，也就是说把容器内开启的端口号挂载到宿主机的端口号上。

## 将宿主机目录指向容器内目录

使用 -v 指令

```bash
docker run -p 3000:3000 --name my-server -v 代码位置的绝对路径:/app -d server:v1
```

- bind mount: -v 绝对路径
- volumn: -v 随便起一个名字

上面的命令表示将宿主机某个绝对路径上的代码挂载到容器里的 app 目录下，后台运行，容器名字为 server，并且版本号为 v1

## 容器间通信

创建一个虚拟网络进行通信

```bash
docker network create my-net
```

创建完成网络之后就可以在一个容器内指定网络，比如在 my-net 网络中启动 redis 容器，并且用 --network-alias 起了一个别名

```bash
docker run -d --name redis --network my-net --network-alias redis redis:latest
```

# docker-compose

可以使用 docker 组合将多个容器进行组合到一起，然后可以一键运行多个容器

比如 windows 的桌面图形版 docker 就不需要单独安装，如果是 MacOS 或者 Linux 系统就需要单独安装 

命令行输入 `docker-compose -v` 检测是否安装成功

需要单独编写 docker-compose 脚本，然后在其中编写命令使用

在同一个 docker-compose 里的容器都默认使用相同的网络，就不需要单独再写网络了

在编写 docker-compose.yml 的目录执行 `docker-compose up` 就能跑起来了，如需后台运行可以加 -d 参数

具体关于 docker-compose 的使用可以看下面实战篇

## docker-compose 常用命令

查看容器运行状态

```bash
docker-compose ps -a
```

启动容器并构建

```bash
docker-compose up --build -d
```

不使用缓存构建容器

```bash
docker-compose build --no-cache
```

删除容器

```bash
docker-compose down
```

重启容器

```bash
docker-compose restart
```

停止容器

```bash
docker-compose stop
```

单个服务重启

```bash
docker-compose restart service-name
```

进入某个容器服务的命令行

```bash
docker-compose exec service-name sh
```

查看某个容器服务运行日志

```bash
docker-compose logs service-name
```

# 实战篇

## 基础部署

这里我们来拿一个前端应用创建一个镜像，我之前写了一个后台管理系统，拿来举个例子

```bash
docker run -it -d --name admin --privileged -p 8080:8080 -v ${PWD}/:/admin node:16.14.2 /bin/bash -c "cd /admin && npm install -g pnpm && pnpm install && pnpm run start"
```

这句话的意思是 创建一个 docker 容器并在后台运行，--privileged 命令是授予容器 root 权限，然后把容器的 8080 端口暴露到宿主机的8080 端口， 然后把宿主机内的代码目录路径指向容器内的 `/admin` 路径( `${PWD}` 命令是获取当前目录的绝对路径，当前目录则为代码所在的根目录)， 然后使用 node 16.14.2版本的镜像，在控制台依次运行下列命令：

>cd /admin  进入到容器内的 /admin 目录
>
>node install -g pnpm  全局安装 pnpm 包管理器(我项目中用到了pnpm)
>
>pnpm install 安装依赖
>
>pnpm run start 启动项目，运行在8080端口

如果想要修改容器内的文件，则需要使用 vim，但是可能会遇到容器没有 vim 命令的问题，解决方式如下

```bash
# 先运行
apt-get update
# 再安装 vim
apt-get install vim
```

## 利用 Docker 进行后端项目部署

首先需要准备一个云服务器，前提就是需要有服务器~~~这里我用的是腾讯云的 CentOS7系统，这也是 Linux 系统的。具体需要云服务器的可以自己到腾讯云或者阿里云购买，如果是学生身份的话买一个服务器就一两百块一年。买服务器这一步就不具体详述了。

下面我们就主要讲一下如何利用 docker-compose 部署一个在本地开发好的 nginx + node.js + redis + mongodb 的项目到云服务器上。

## Docker 安装

首先登录到云服务器上，可以通过腾讯云或者阿里云的官网进行网页登录，也可以利用 ssh 在本地控制台登录。登录成功后先安装 Docker 。不知道怎么在本地连接远程服务器的可以看我另一篇文章 [手摸手教你如何从零一步一步部署一个前端项目到远程服务器~~](https://juejin.cn/post/6976114620897427464)

```bash
sudo yum install docker-ce docker-ce-cli containerd.io
```

安装完成之后控制台会显示 Complete!，然后在控制台输入 `docker -v` 可以看到 docker 的版本号信息：

```bash
Docker version 20.10.13, build a224086
```

如果不是 CentOS 系统可以自己去官网安装。安装地址在这儿 `https://docs.docker.com/get-docker/`，找到和自己电脑对应的操作系统安装就行

## 设置开机自启

```bash
sudo systemctl enable docker 
```

## 启动 Docker

```bash
sudo systemctl start docker 
```

启动成功之后，控制台输入 `docker info` 可以看到 Server 一栏有相关信息：

```json
Server:
  Containers: 0
  Running: 0
  Paused: 0
  Stopped: 0
  Images: 0
  ......
```

## 测试安装结果

输入下面命令会去 dockerHub 拉取 hello-world 这个镜像然后启动

```bash
sudo docker run hello-world
```

最终控制台显示了 `Hello from Docker!` 那就证明安装 Docker 这一步就已经完成啦。

## 服务器安装 docker-compose

docker-compose 是一个用于定义和运行多容器 Docker 应用程序的工具，可以设置好多个容器，然后可以使用一个命令启动所有容器。

Linux 安装，控制台输入如下命令：

```bash
sudo curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
```

安装完成之后需要申请执行权限，输入以下命令：

```bash
sudo chmod +x /usr/local/bin/docker-compose
```

最后控制台输入下面命令查看是否已经安装完成：

```bash
docker-compose -v
# 控制台显示：docker-compose version 1.29.2, build 5becea4c
```

## 服务器安装 Git

```bash
yum install -y git
```

安装完成之后控制台输入

```bash
git --version
# git version 1.8.3.1
```

这样就代表 Git 安装成功啦

## 初始化git

安装好git之后就要进行初始化操作。第一次使用git的时候我们需要给git配置用户名和邮箱，用户和邮箱可以使用 github 的，也可以使用gitlab 仓库的账号

配置用户名

```bash
git config --global user.name "用户名"
```

配置邮箱

```bash
git config --global user.email "邮箱地址"
```

配置好这个以后我们输入便可以看到我们所有的配置信息了，然后可以看到 user.name 和 user.email 配置得对不对

```bash
git config -l
```



## 配置 ssh 密钥

配置完密钥之后在 git 上推拉代码的时候就不需要再重复输入密码确认了，比较方便。

```bash
ssh-keygen -t rsa -C "邮箱地址"

# Generating public/private rsa key pair.
# 接下来会弹出三个命令会问你存放位置，以及输入两次密码，依次操作即可
# Enter file in which to save the key (/root/.ssh/id_rsa): 
# 输入密码
# Enter passphrase (empty for no passphrase): 
# 确认密码
# Enter same passphrase again: 
```

配置成功后会显示，就是说你的密钥存放在了 /root/.ssh/id_rsa 中

```bash
Your identification has been saved in /root/.ssh/id_rsa.
Your public key has been saved in /root/.ssh/id_rsa.pub.
```

接下来将私钥添加到本机，输入命令

```bash
ssh-add ~/.ssh/id_rsa
# 接下来会让你输入密码，就是你前面输入的密码
# 成功后会显示 Identity added: /root/.ssh/id_rsa (/root/.ssh/id_rsa)
```

然后就查看一下公钥，这个公钥需要复制到 git 里的 setting。

```bash
# 查看公钥
cat ~/.ssh/id_rsa.pub
# 显示一大堆字符串，然后复制这堆字符串，按下面的操作进行
```

- 点击 github 头像，然后倒数第二个是 setting
- 左侧的一堆选项栏中，找到一个钥匙图标的 `SSH and GPG keys`
- 然后在 SSH keys 这一个面板，点击右边绿色的 `New SSH key `  按钮
- 随便起个备注的名字，然后将刚刚复制的一大堆字符串密钥，粘贴到这儿，点击确定就完成了

接下来在服务器控制台输入下面命令来验证是否配置成功

```bash
ssh -T git@github.com
```

如果显示下面的命令就配置成功了，好了，git 的安装就可以告一段落了~~~

```bash
The authenticity of host 'github.com (20.205.243.166)' can't be established.
ECDSA key fingerprint is xxxxxxxxxxxxxxxxxx
ECDSA key fingerprint is xxxxxxxxxxxxxxxxxx
Are you sure you want to continue connecting (yes/no)? yes
Warning: Permanently added 'github.com,20.205.243.166' (ECDSA) to the list of known hosts.
Hi xxxxxxx(你的git名字)! You've successfully authenticated, but GitHub does not provide shell access.
```

## 准备好开发完毕的项目

我这里用的项目主要是 Node.js 编写的服务器，其中里面引用了 mongodb 和 redis，然后使用 nginx 作为网关然后配置反向代理。

我们的项目结构是这样子的，我们下面就拿这样的目录结构来做示范~~~

```text
|-- epidemic-compose
   |-- docker-compose.yml # 编写 docker-compose 编排逻辑的
   |-- epidemic-server # node 服务器
   |-- mongo # 存放 mongo 初始化脚本和作为容器中 mongodb 数据的挂载目录
   |-- nginx # nginx 的配置
```

我们再来看看二级目录长啥样

```text
|-- epidemic-compose
    |-- docker-compose.yml
    |-- epidemic-server
    |   |-- commitlint.config.js
    |   |-- Dockerfile       # 编写容器的配置
    |   |-- nest-cli.json
    |   |-- package.json
    |   |-- .env            # 放环境变量的地方
    |   |-- .dockerignore   # 里面忽略 node_modules
    |   |-- pnpm-lock.yaml
    |   |-- README.md
    |   |-- src              # 存放源码的地方
    |   |-- tsconfig.build.json
    |   `-- tsconfig.json
    |-- mongo
    |--  -- mongo-volume   # 用来挂载 mongodb 容器中的数据库数据
    |   `-- init-mongo.js  # 用来创建 mongodb 初始账户的
    |-- nginx
    |   `-- nginx.conf     # 编写 nginx 的配置
```

## 编写 Docker 配置文件

我们先来看看 epidemic-server 目录里的 Dockerfile 如何编写，这个 Dockerfile 最终会将其打包成一个服务端容器

```bash
# 安装 Node 精简版
FROM node:16.14.2-alpine

# 设置维护者信息 
LABEL maintainer="Dachui"

# 防止中文打印信息显示乱码
ENV LANG="C.UTF-8"

# 拷贝项目文件进行构建，拷贝到容器内的 app/server 目录下
WORKDIR /app/server
# 将项目中的 package.json 文件拷贝到容器中的 app/server 
COPY ./package.json /app/server
# 拷贝 pnpm 的依赖锁文件
COPY pnpm-lock.yaml /app/server

# 项目中用到了 pnpm 包管理器
RUN npm install -g pnpm --registry=https://registry.npm.taobao.org
# 然后安装 pm2 用来做服务器的进程守护
RUN pnpm install -g pm2
# 安装项目依赖
RUN pnpm install

# 将当前目录代码复制到容器中
COPY . /app/server

# 打包代码
RUN pnpm run build

# 对外暴露3000端口
EXPOSE 3000

# 运行 pm2 启动打包之后的项目, pm2在容器中运行需要用 pm2-runtime 命令
CMD [ "pm2-runtime", "dist/main.js" ]
```

接下来我们看看 ` docker-compose.yml ` 文件，docker compose 可以将多个容器进行编排，以在最终实现一键启动所有容器

```bash
version: '3'

# 自定义网络
networks:
  # 网络名字
  mynet:
    # 由网关驱动
    driver: bridge

# 容器服务
services:
  # 服务名称
  mongo:
    # 安装镜像
    image: mongo:latest
    # 容器名称
    container_name: mongo
    # 挂掉之后重新自启
    restart: always
    # 将容器内的对应目录挂载到宿主机上
    # 比如容器中的 /data/db 里面的东西都会放到我们服务器中的 ~mongo/mongo-volume 目录
    volumes:
      - ./mongo/mongo-volume:/data/db
      # init-mongo.js文件会在 mongodb 容器初始化完成之后执行，给数据库创建默认的角色
      - ./mongo/init-mongo.js:/docker-entrypoint-initdb.d/mongo-init.js:ro
    environment:
      # 时区，设置为上海，就是东八区
      TZ: Asia/Shanghai
      # 初始化 mongodb 的账户，这个账户会创建在 admin 下，就是超管权限
      MONGO_INITDB_ROOT_USERNAME: root
      MONGO_INITDB_ROOT_PASSWORD: password
      MONGO_INITDB_DATABASE: my-database
    ports:
      # 将容器的27017端口映射到宿主机的27017端口
      - 27017:27017
    networks:
      # 设置网络
      - mynet

  redis:
    image: redis:latest
    container_name: redis
    restart: always
    environment:
      - TZ=Asia/Shanghai
    ports:
      - 6379:6379
    # 这里的命令用来给 redis 创建默认的密码，在 node 里面我们用 ioredis 这个包和 redis 进行连接
    command:
      - /bin/bash
      - -c
      - redis-server --appendonly yes --requirepass "redispassword"
    networks:
      - mynet

  # 服务器
  server:
    # 使用 ./epidemic-server 目录下的 Dockerfile 文件进行构建容器，然后启动
    build:
      context: ./epidemic-server
    container_name: server
    ports:
      - 3000:3000
    restart: always
    environment:
      - TZ=Asia/Shanghai
    # 这里表示需要先等 mongo 和 redis 两个容器服务启动好才会启动 server
    depends_on:
      - mongo
      - redis
    networks:
      - mynet

  nginx:
    image: nginx:alpine
    container_name: nginx
    volumes:
      # 容器里的 nginx 配置将当前路径下的 nginx 目录里的 nginx.conf
      - ./nginx/nginx.conf:/etc/nginx/nginx.conf
    ports:
      - 80:80
      - 443:443
    restart: always
    environment:
      - TZ=Asia/Shanghai
    networks:
      - mynet
    depends_on:
      - server
```

接下来我们来看看 mongo 目录下的 `init-mongo.js` 文件

这个文件主要是在 mongo 容器生成的时候给 epidemic- server 数据库设置初始化密码用的

```js
// 需要和上面 docker-compose.yml 里的MONGO_INITDB_ROOT_USERNAME和 MONGO_INITDB_ROOT_PASSWORD 对应上
db.auth("root", "password");

// 需要和上面的 MONGO_INITDB_DATABASE 对应上
db = db.getSiblingDB("my-database");

db.createUser({
  user: "user1",
  pwd: "password1",
  roles: [
    {
     // 赋予这个用户读写 my-databse 数据库的权限
      role: "readWrite",
      db: "my-database",
    },
  ],
});
```

我们在 Node.js 里用 mongoose 是这样子连接数据库的

```bash
# @ 后面本身是填域名的，但是我们这里用的是 docker 容器间通信，所以要填服务名称，就是上面docker-compose里的services->mongo
mongodb://user1:password1@mongo/my-database
```

最后我们再来看看 nginx.conf 文件，这个文件是用来编写 nginx 配置服务的

```nginx
# 运行用户，默认即是nginx，可以不进行设置
user nginx;
# nginx进程数，建议设置为等于CPU总核心数。
worker_processes 1;

events {
  # 使用epoll的I/O模型(如果你不知道Nginx该使用哪种轮询方法，会自动选择一个最适合你操作系统的)
  use epoll;
  # 每个进程允许最大并发数
  worker_connections 1024;
}

http {
  # 开启高效传输模式
  sendfile on;
  # 减少网络报文段的数量
  tcp_nopush on;
  tcp_nodelay on;
  # 保持连接的时间，也叫超时时间，单位秒
  keepalive_timeout 30;
  types_hash_max_size 2048;

  # 文件扩展名与类型映射表
  include /etc/nginx/mime.types;
  # 加载其它的子配置文件
  include /etc/nginx/conf.d/*.conf;
  # 默认文件类型
  default_type application/octet-stream;

  # 默认off，是否开启 gzip 压缩
  gzip on;
  # 设置启用 gzip 的类型
  gzip_types text/plain text/css application/json application/x-javascript text/javascript;
  # gzip 压缩比，压缩级别是 1-9，1 压缩级别最低，9 最高，级别越高压缩率越大，压缩时间越长，建议 4-6
  gzip_comp_level 4;

  # 服务器地址，这里可以配置多个服务器地址，实现负载均衡
  upstream my_server {
    server 服务器地址(可以是ip也可以填域名):3000;
  }

  server {
    # 监听启动80端口
    listen 80;
    # 服务器地址，也可以填域名或ip地址
    server_name 服务器地址;

    #location / {
    #  如果你有前端项目的话，这里也可以找你打包后前端项目
    #  root 前端打包后文件的地址;
    #  index index.html index.htm;
    #  try_files $uri $uri/ /index.html;
    #}

    # 我编写 node服务的时候，把所有接口的前缀都加上了 /api
    # 当 nginx 匹配到请求 /api后缀的路径时就会把请求代理到 3000 端口的 node 服务
    location /api/ {
      # 代理本机启动的 node 服务器，服务器启动在 3000 端口
      proxy_pass http://my_server/api/;

      # 获取用户真实 ip，否则 Node 服务中拿不到用户的 ip 地址
      proxy_set_header Host $host;
      proxy_set_header X-Real-IP $remote_addr;
      proxy_set_header X-Real-Port $remote_port;
      proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
  }
}
```

## 项目部署

正常来说将本地代码部署到远程服务器上有几种方式：

1. 在本地连接到远程服务器后直接将本地的代码通过文件传输命令行发送到服务器上，然后在服务器上进行所有操作
2. 在本地把所有容器 build 好之后上传到 DockerHub 中（一般要设置为私有仓库），然后在服务器上拉取 DockerHub 的镜像
3. 本地开发完之后上传代码到Git，然后在服务器上通过 Git 拉取代码，然后进行部署



我们这里使用的是第三种方法来进行代码管理 ：

本地代码开发完成之后，上传到git，这一步不用说了吧，默认你会上传代码哈~~~然后在服务器上拉取 git 仓库中的代码，上传代码到仓库之后，要去克隆 ssh 的链接，而不是 https 的链接，类似下面这样的

```bash
git clone git@github.com:用户名/仓库名.git
```

接下来在服务器中的 root 目录下新建一个 project 目录并进入到该目录，然后去拉取 git 中的代码

```bash
cd ~root && mkdir project && cd myproject

git clone git@github.com:用户名/仓库名.git
```

输入下面命令可以查看当前文件夹下所有的文件和目录

```bash
ls -a
```

然后进入到仓库中，我们的仓库就叫做  `epidemic-compose`，然后就要 `cd epidemic-compose` 进入项目文件夹内，准备构建项目

然后运行下面的这个命令，等待安装完成~~~ 到这里其实项目就已经是可以正常访问了的~

```bash
docker-compose up -d --build
```

最后的最后，我们的 node.js 服务是部署在 3000 端口的，然后我们使用了反向代理 proxy_pass，所以我们可以直接通过80端口去访问到我们的服务器，下面我们就看看效果


![](https://files.mdnice.com/user/6550/5101b525-372a-4fa6-adb9-c48ff19b1715.png)


大功告成了哈~

看吧，我们只要把项目开发完毕之后，再编写一下 docker 的配置文件，就可以快速从一个机器上把项目部署到另一个机器上
，我这里开发的时候用的是 Windows 系统，然后用 docker 快速把项目部署到 Linux 系统的服务器上。不难吧~~~

# 结语

其实在最后一步的时候还可以加上自动化，比如每次我们从本地上传最新代码到 git 的时候就触发一个钩子函数，服务器上从仓库拉取最新代码然后重新启动容器，这样的话我们就能做到一套完整的自动化部署流程了。
