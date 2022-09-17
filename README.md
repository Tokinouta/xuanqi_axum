拟采用技术选型：

- 数据库：MongoDB
- 服务器：actix

中间需不需要加一层redis？

原玄奇系统中，一个仓库对应一个python类，内部的条目采用嵌套的列表结构。

如果要支持共享编辑，需要考虑竞争条件。能不能同时读写？万一读到一半有人写东西了怎么办？

创建存储用数据库：

```bash
docker run -d --network  mysql-net --name some-mongo -e MONGO_INITDB_ROOT_USERNAME=mongoadmin -e MONGO_INITDB_ROOT_PASSWORD=secret -p 27017:27017 mongo
```

创建访问用mongodb终端：

```bash
docker run -it --rm --network mysql-net mongo mongo --host some-mongo -u mongoadmin -p secret --authenticationDatabase admin some-db
```