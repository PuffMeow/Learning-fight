### Hello world

Docker 允许你在容器内运行应用程序， 使用 **docker run** 命令来在容器内运行一个应用程序

```
runoob@runoob:~$ docker run ubuntu:15.10 /bin/echo "Hello world"
```

安装一个ubuntu 15.10版本的容器，并在容器中输出hello world，如果本地不存在该容器就会从远程仓库下载，相当于你在Docker容器内安装了一个ubuntu的虚拟环境，可以在里面执行各种Linux的指令。

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

- 在控制台中输入exit退出

如果输入了-d参数，会让容器后台运行，那么怎么进入到容器中呢？

一个是`docker attach 容器ID`命令，如果从这个命令进入到容器中后，再输入`exit`会把整个容器也退出，不再维持后台运行的状态。

另一个是`docker exec -it 容器ID /bin/bash`命令回到容器中，执行这个命令在容器中输入`exit`不会把整个容器也退出，容器仍将维持后台运行状态。

### Docker状态

输入指令`docker ps -a`可以查看所有的容器。

![image-20211014233025791](C:\Users\JqWang\AppData\Roaming\Typora\typora-user-images\image-20211014233025791.png)

如果要恢复一个已经停止的容器可以输入`docker start 容器ID`，同样的，想要停止一个容器可以输入`docker stop 容器ID`。

另外还有`docker restart 容器ID`命令用于重启容器

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