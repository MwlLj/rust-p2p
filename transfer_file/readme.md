## 使用

### 参数说明
序号 | 参数名 | 所属 | 参数含义
:--: | :--: | :--: | :--:
1 | -server | 公有 | 中转服务地址, 如: 180.167.205.74:51110
2 | -self | 公有 | 通信一方自身id
3 | -peer | 公有 | 通信另一方id
4 | -mode | 公有 | send: 发送方; recv: 接收方
5 | -extra-data | 发送端 | 发送的文件路径
6 | -write-file-mode | 接收端 | cover: 表示覆盖现有的文件; create: 表示创建新的文件
7 | -once-max | 发送端 | 一次发送的字节数, 如: 1M -> 1024000
8 | -download-root | 接收端 | 文件下载的存储路径 (如果不存在, 将自动创建)
9 | -conn-timeouts | 公有 | 连接中转服务的超时时间
10 | -obj | 发送端 | 对象id, 如果为空, 将默认为文件名

### 接收端使用示例
```
transfer_file.exe -self 654321 -peer 123456 -mode recv -write-file-mode create -server 180.167.205.74:51110
```

### 发送端使用示例
```
transfer_file.exe -self 123456 -peer 654321 -mode send -extra-data test.exe -once-max 1024000 -server 180.167.205.74:51110
```

