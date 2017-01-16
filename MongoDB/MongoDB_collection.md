# Collection

## update

```
update(selector, document[, options][, callback])
```

### field 名を変更する

```
db.getCollection('Collection名').update({}, {$rename: {"beforeName": "afterName"}}, {multi: true})
```

`multi: true` を指定すると全てのドキュメントに対して行われる