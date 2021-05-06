# lab7



## 编程内容

修改了 `sys_open` 并实现了`sys_linkat、sys_unlinkat、sys_stat` 三个系统调用。首先需要对 `syscall` 的参数个数进行一下修改，然后修改文件系统，编写链接相关的方法，具体地说就是用一个目录项表示硬链接，新目录项的  `inode_number` 与其链接的目标一致。同时我在 `Inode` 中加入 `inode_id` 以方便 `stat` 调用

还需要对前六章练习中的代码做一些修改，主要在于第七章开始从文件读入程序内容，而不是从内存读入了



## 问答作业

### 1

目前的文件系统只有单级目录，假设想要支持多级文件目录，请描述你设想的实现方式，描述合理即可。

首先对 `easy-fs` 改造，在 `DiskInode` 中加入上一级目录的索引，同时在 `Inode` 中加入创建子目录的接口。子目录本身也是一种文件，而且 `DiskInode` 已经支持目录这种文件类型，所以实现简单的多级目录功能修改不是很多

其次在 OS 以及用户库中也要加入相应的进入、创建子目录等系统调用。OS 应该像维护 `ROOT_INODE` 那样维护当前所在的目录，当然 `ROOT_INODE` 也要继续维护，以支持 `cd /` 这种绝对寻址

### 2

在有了多级目录之后，我们就也可以为一个目录增加硬链接了。在这种情况下，文件树中是否可能出现环路(软硬链接都可以，鼓励多尝试)？你认为应该如何解决？请在你喜欢的系统上实现一个环路，描述你的实现方式以及系统提示、实际测试结果。

可能实现环路，为目录中的文件添加硬链接连接到其祖先目录即可

应该禁止给目录创建硬链接来解决环路问题

尝试在 Ubuntu 下创建硬链接环路：

```bash
yuntong@Yuntong-Ubuntu:~/test$ ln ~/test test
ln: /home/yuntong/test: hard link not allowed for directory
```

可见 Ubuntu 禁止了为目录创建硬链接

尝试创建软连接环路：

```bash
yuntong@Yuntong-Ubuntu:~/test$ ln -s ~/test test
yuntong@Yuntong-Ubuntu:~/test$ cd test/test/test/test/test/test/test/test/test/test/test
yuntong@Yuntong-Ubuntu:~/test/test/test/test/test/test/test/test/test/test/test/test$ 
```

可见创建时没有提示，也可以正常工作