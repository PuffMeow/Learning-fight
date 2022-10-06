## Docker 简介

简单一句话就是一个应用打包、分发、部署的工具，可以把它理解为一个轻量的虚拟机，但是是以容器的方式运行的。

支持各种系统 Linux，MacOS，Windows等。可以使用容器化部署以降低项目在不同的平台之间进行部署的成本。

再也不会出现 “怎么在我的电脑能运行，到了服务器就运行不了” 这种情况。

### 安装

安装地址在这儿 `https://docs.docker.com/get-docker/`，找到和自己电脑对应的操作系统安装就行

安装完成之后命令行输入 `docker -v` 可以看到版本号信息。

### 使用国内镜像加速

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

### Hello world

Docker 允许你在容器内运行应用程序， 使用 **docker run** 命令来在容器内运行一个应用程序

```
docker run ubuntu:15.10 /bin/echo "Hello world"
```

安装一个 ubuntu 15.10版本的容器，并在容器中输出hello world，如果本地不存在该容器就会从远程仓库下载，相当于你在 Docker 容器内安装了一个 ubuntu 系统的虚拟环境，可以在里面执行各种Linux的指令。

### 查看 docker 当前运行信息

```bash
docker info
```

### 交互式容器

相当于可以在容器虚拟环境中打开控制台。

放在镜像名后的是命令，这里我们希望有个交互式 Shell，因此用的是 /bin/bash

```
docker run -i -t ubuntu:15.10 /bin/bash
# 或者
docker run -it ubuntu:15.10 /bin/bash
```

- **-t:** 在新容器内指定一个伪终端或终端。
- **-i:** 允许你对容器内的标准输入 (STDIN) 进行交互。
- **-d：**让容器在后台运行，输入后不进入交互模式 
- **-p**：表示暴露端口
- 在控制台中输入exit退出

如果输入了-d参数，会让容器后台运行，那么怎么进入到容器中呢？

一个是`docker attach 容器ID`命令，如果从这个命令进入到容器中后，再输入`exit`会把整个容器也退出，不再维持后台运行的状态。

另一个是`docker exec -it 容器ID /bin/bash`命令回到容器中，执行这个命令在容器中输入`exit`不会把整个容器也退出，容器仍将维持后台运行状态。

## 镜像



### 查看镜像

```
docker images
```

### 启动一个镜像

```
docker run --name test-mysql -d -p 3306:3306 -e  MYSQL_ROOT_PASSWORD=123456 -v /root/data:/var/lib/mysql mysql
```

### 删除镜像

```
docker rmi 镜像仓库源
# 比如说要删除上面的test v0.0.1版本
docker rmi test:v0.0.1
```

### 获取镜像

当使用`docker run `命令来运行本地不存在的镜像时会自动下载镜像，但也可以使用`docker pull`命令来提前下载

```
docker pull 镜像名:版本号
# 例如
docker pull ubuntu:15.10
```

### 查找镜像

直接去官网找吧 **https://hub.docker.com/**

### 安装软件

比如在上面的官网中查找一个 redis 镜像，然后安装最新版的

```bash
docker run -d -p 6379:6379 --name redis redis:latest
```

-p 命令后面接的端口号指， 宿主机端口号:容器内端口号，也就是说把容器内开启的端口号挂载到宿主机的端口号上。

## 容器

### 删除一个容器

```
docker rm -f 容器ID
```

### 清理列表中所有终止状态的容器

```
docker container prune
```

### 查看当前运行的所有容器

```
docker container ls
```

### 查看当前存在的所有容器

```
docker ps -a
```

### 将宿主机目录指向容器内目录

使用 -v 指令

```bash
docker run -p 3000:3000 --name my-server -v 代码位置的绝对路径:/app -d server:v1
```

- bind mount: -v 绝对路径
- volumn: -v 随便起一个名字

上面的命令表示将宿主机某个绝对路径上的代码挂载到容器里的 app 目录下，后台运行，容器名字为 server，并且版本号为 v1

### 容器间通信

创建一个虚拟网络进行通信

```bash
docker network create my-net
```

创建完成网络之后就可以在一个容器内指定网络，比如在 my-net 网络中启动 redis 容器，并且用 --network-alias 起了一个别名

```bash
docker run -d --name redis --network my-net --network-alias redis redis:latest
```

## docker-compose

可以使用 docker 组合将多个容器进行组合到一起，然后可以一键运行多个容器

比如 windows 的桌面图形版 docker 就不需要单独安装，如果是 MacOS 或者 Linux 系统就需要单独安装 

命令行输入 `docker-compose -v` 检测是否安装成功

需要单独编写 docker-compose 脚本，然后在其中编写命令使用

在同一个 docker-compose 里的容器都默认使用相同的网络，就不需要单独再写网络了

#### 运行

在编写 docker-compose.yml 的目录执行 `docker-compose up` 就能跑起来了，如需后台运行可以加 -d 参数

- 查看运行状态：`docker-compose ps`
- 停止运行：`docker-compose stop`
- 重启：`docker-compose restart`
- 重启单个服务：`docker-compose restart service-name`
- 进入容器命令行：`docker-compose exec service-name sh`
- 查看容器运行日志：`docker-compose logs service-name`

### 容器发布和部署

可以把 build 出来的容器镜像上传到 docker 提供的镜像库中，方便开源，也可以搭建自己私有的镜像仓库然后放到上面

注册一个 docker 账号之后就可以 -> [到这里创建自己的镜像仓库](https://hub.docker.com/repository/create)

命令行登录 docker 账户，然后输入密码，登录成功后显示 `Login Succeeded`

```bash
docker login -u username 
```

新建一个 tag，名字要跟注册账号一样

```bash
docker tag my-server:v1 username/my-server:v1
```

推送到仓库

```bash
docker push username/my-server:v1
```

部署

```bash
docker run -dp 3000:3000 username/my-server:v1
```

### 容器数据备份

容器内的数据如果没有挂载到宿主机目录，在删除镜像后容器内的数据就会丢失

如果是用 `bind mount`直接把宿主机的目录挂进去容器，那迁移数据很方便，直接复制目录就好了
如果是用`volume` 方式挂载的，由于数据是由容器创建和管理的，则需要用特殊的方式把数据弄出来。

linux命令中有几个压缩包所用的命令，这里先了解一下，下面会用到：

- tar cvf xxx.tar   打包一个tar
- tar xvf xxx.tar   解压一个tar

#### 备份 MongoDB 数据演示

- 运行一个 mongodb，创建一个名叫`mongo-data`的 volume 指向容器的 /data 目录
  `docker run -p 27017:27017 --name mongo -v mongo-data:/data -d mongo:4.4`
- 运行一个 Ubuntu 系统容器，挂载 `mongo` 容器的所有 volume，映射宿主机的 backup 目录到容器里面的 /backup 目录，然后运行 tar 命令把数据压缩打包， --rm 命令会把已有的同名容器删除再启动
  `docker run --rm --volumes-from mongo -v 宿主机绝对路径/backup:/backup ubuntu tar cvf /backup/backup.tar /data/`

最后你就可以拿着这个 backup.tar 文件去其他地方导入了。

#### 恢复 Volume 数据演示

- 运行一个 ubuntu 容器，挂载 `mongo` 容器的所有 volumes，然后读取 /backup 目录中的备份文件，解压到 /data/ 目录
  `docker run --rm --volumes-from mongo -v 宿主机绝对路径/backup:/backup ubuntu bash -c "cd /data/ && tar xvf /backup/backup.tar --strip 1"`

> volumes-from 指定的是容器名字
> strip 1 表示解压时去掉前面1层目录，因为压缩时包含了绝对路径
