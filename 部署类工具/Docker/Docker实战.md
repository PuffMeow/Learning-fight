# 看完这篇文章你能了解到什么?

- **利用 Docker 快速跨平台部署后端( Node.js + MongoDB + Redis + Nginx ) 项目**
- 一些常见的 Linux 系统操作
- 编写 Dockerfile 文件
- 编写 docker-compose 文件
- 编写 nginx 配置文件

# PS

这里主要讲的是利用容器化的方式去部署项目，容器化部署的好处有很多哈，比如容器可以很方便从一台电脑迁移到另一台电脑。

如果想要了解传统的实机部署方式，可以看看我的另一篇文章

# 利用 Docker 进行项目部署

首先需要准备一个云服务器，前提就是需要有服务器~~~这里我用的是腾讯云的 CentOS7系统，这也是 Linux 系统的。具体需要云服务器的可以自己到腾讯云或者阿里云购买，如果是学生身份的话买一个服务器就一两百块一年。买服务器这一步就不具体详述了。

# Docker 安装

首先登录到云服务器上，可以通过腾讯云或者阿里云的官网进行网页登录，也可以利用 ssh 在本地控制台登录。登录成功后先安装 Docker 

```bash
sudo yum install docker-ce docker-ce-cli containerd.io
```

安装完成之后控制台会显示 Complete!，然后在控制台输入 `docker -v` 可以看到 docker 的版本号信息：

```bash
Docker version 20.10.13, build a224086
```

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

# Docker基本概念

使用 Docker 前需要先了解这几个基本概念

- 镜像（image）
- 容器（container）
- 仓库（repository）

获取镜像的方式可以通过 Dockerfile 文件创建，也可以通过 dockerHub 仓库下载

- Docker 中**镜像**和**容器**的关系就像**类**与**实例**的关系
- 镜像可以通过 Dockerfile 文件来生成，容器通过镜像来创建

# 安装 Docker-Compose

Docker-Compose 是一个用于定义和运行多容器 Docker 应用程序的工具，可以设置好多个容器，然后可以使用一个命令启动所有容器。

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

# CentOS 系统安装 Git

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

# 准备好开发完毕的项目

我这里用的项目主要是 Node.js 编写的服务器，其中里面引用了 mongodb 和 redis，然后使用 nginx 作为网关然后配置反向代理。



我们的项目结构是这样子的，我们下面就拿这样的目录结构来做示范~~~

```text
|-- epidemic-compose
   |-- docker-compose.yml # 编写 docker-compose 编排逻辑的
   |-- epidemic-server # node 服务器
   |-- mongo # 存放mongo初始化脚本和作为容器中mongodb数据的挂载目录
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

然后进入到仓库中，我们的仓库就叫做 `epidemic-compose`，然后就要 `cd epidemic-compose` 进入项目文件夹内，准备构建项目

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

后面有时间再写一下关于前端项目如何部署，其实也不难，可以利用 CDN 的方式，也可以使用 nginx 放到服务器上部署。如果你也想部署你的项目，看完教程还不会，可以私信我
