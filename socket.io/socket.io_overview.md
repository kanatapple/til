# 概要

Socket.IOはリアルタイム双方向イベントベースの通信を可能にする  
これは、すべてのプラットフォーム、ブラウザ、またはデバイスで動作し、信頼性とスピードに注力する

# 使い方

## インストール方法

```
npm install socket.io
```

## イベントの送受信

Socket.IOはカスタムイベントの送受信を許可する  
接続、メッセージ、切断以外にもカスタムイベントを発行できる

```javascript
// note, io(<port>) will create a http server for you
var io = require('socket.io')(80);

io.on('connection', function (socket) {
  io.emit('this', { will: 'be received by everyone'});

  socket.on('private message', function (from, msg) {
    console.log('I received a private message by ', from, ' saying ', msg);
  });

  socket.on('disconnect', function () {
    io.emit('user disconnected');
  });
});
```

## ネームスペースによる制限

もし特定のアプリケーションに発行される全てのメッセージやイベントの制御が出来るのであれば、デフォルトの / のネームスペースで問題なく動作する  
もし、サードパーティのコードを取り入れたり、他のコードと共有したい場合のために、 Socket.IO はソケットに対するネームスペースを提供している

これは単一コネクション上での `multiplexing(多重通信)` の恩恵であり、二つの `WebSocket` 通信ではなく、一つだけを使って行っている

Server
```javascript
var io = require('socket.io')(80);
var chat = io
  .of('/chat')
  .on('connection', function (socket) {
    socket.emit('a message', {
        that: 'only'
      , '/chat': 'will get'
    });
    chat.emit('a message', {
        everyone: 'in'
      , '/chat': 'will get'
    });
  });

var news = io
  .of('/news')
  .on('connection', function (socket) {
    socket.emit('item', { news: 'item' });
  });
```

Client
```javascript
<script>
  var chat = io.connect('http://localhost/chat')
    , news = io.connect('http://localhost/news');
  
  chat.on('connect', function () {
    chat.emit('hi!');
  });
  
  news.on('news', function () {
    news.emit('woot');
  });
</script>
```

## 揮発性メッセージの送信

メッセージが欠落してもいい場合、揮発性メッセージを送ることができる

```javascript
var io = require('socket.io')(80);

io.on('connection', function (socket) {
  var tweets = setInterval(function () {
    getBieberTweet(function (tweet) {
      socket.volatile.emit('bieber tweet', tweet);
    });
  }, 100);

  socket.on('disconnect', function () {
    clearInterval(tweets);
  });
});
```

## 送達確認

クライアントがメッセージを受信したことを確認したい場合がある  
その場合は、単純に`.send`や`.emit`の最後の引数にコールバック関数を渡せばよい  
また`.emit`では、コールバックにデータを渡して実行することもできる

Server
```javascript
var io = require('socket.io')(80);

io.on('connection', function (socket) {
  socket.on('ferret', function (name, fn) {
    fn('woot');
  });
});
```

Client
```html
<script>
  var socket = io(); // TIP: io() with no args does auto-discovery
  socket.on('connect', function () { // TIP: you can avoid listening on `connect` and listen on events directly too!
    socket.emit('ferret', 'tobi', function (data) {
      console.log(data); // data will be 'woot'
    });
  });
</script>
```

## メッセージのブロードキャスト

ブロードキャストするには、`broadcast`フラグを追加して`.emit`やP`.send`を呼び出すだけ  
ブロードキャストとは、それを初めた人(そのメッセージを送って来た人)以外全員に送ること

```javascript
var io = require('socket.io')(80);

io.on('connection', function (socket) {
  socket.broadcast.emit('user connected');
});
```

## クロスブラウザWebSocketとして使用

もし本来のWebSocketの構文と同じように使いたい場合、`send`でメッセージを送り、`message`イベントを検出すればよい

Server
```javascript
var io = require('socket.io').listen(80);

io.sockets.on('connection', function (socket) {
  socket.on('message', function () { });
  socket.on('disconnect', function () { });
});
```

Client
```html
<script>
  var socket = io.connect('http://localhost/');
  socket.on('connect', function () {
    socket.send('hi');

    socket.on('message', function (msg) {
      // my msg
    });
  });
</script>
```

http://jxck.github.io/socket.io/