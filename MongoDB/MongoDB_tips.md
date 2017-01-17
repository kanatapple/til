# Tips

## 各 Collection のサイズ容量算出

```
db.getCollectionNames().forEach(function(n){print(n + ": " + db[n].stats().storageSize/1024/1024 + "MB")})
```