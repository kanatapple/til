# Rooms and Namespaces

## Rooms

各ネームスペース内で、参加/退出ができる任意のチャンネルを定義することができる

### Joining and leaving

`.join`を呼ぶことで指定されたチャンネルを購読することができる

```javascript
io.on('connection', function(socket){
  socket.join('some room');
});
```

ブロードキャストや送信をしたい場合は、`.to`か`.in`(これらはどちらも同じ)を使うだけ

```javascript
io.to('some room').emit('some event');
```

チャンネルから退出する場合は、`.join`と同じように`.leave`を呼ぶ