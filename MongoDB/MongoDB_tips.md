# Tips

## 各 Collection のサイズ容量算出

```
db.getCollectionNames().forEach(function(n){print(n + ": " + db[n].stats().storageSize/1024/1024 + "MB")})
```

## Docker を使って構築する方法

```
systemctrl enable docker
systemctrl start docker

docker pull mongo
docker run --restart=always --name zest-mongo -d -p 27017:27017 mongo
                                                      |     └コンテナのポート番号
                                                      └ホストのポート番号
```