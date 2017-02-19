# RxJS とは

The Reactive Extensions for JavaScript  
Reactive Extensions の JavaScript 向けの実装

## RxJS = Observables + Operators + Schedulers

RxJS は下記の要素からなる
- Observable ・・・ 非同期データストリームを表現するもの
- Operator ・・・ Observable に対して map や filter をはじめとするクエリーを適用するもの
- Scheduler ・・・ 並行性を制御するもの

## Observable は複数個に対応できる Promise

Observable は複数値を受け取れる Promise で、非同期に対応した Iterable

イベントは配列に似ている  
イベントは時間軸上に点在する離散的な「点」

```
--- ○ --- ○ - ○ ------- ○ -- ○ ----->  
   [○,    ○,  ○,        ○,   ○]
```

配列のようにイベントを操作できるということ

## Stream

厳密には配列ではない
「Stream」とは「水の流れ・小川」という意味だから、「タイムライン」という「川」が流れていて、「イベント」が流れてくるイメージ

Rx では Observable と呼ぶことがあるが、「ストリーム」の方がわかりやすい

ストリームを作成するメソッドがたくさん用意されている

- Create
- Empty
- From
- Interval
- Range

他にもたくさん

## オペレータ

ストリームに対して何らかの操作を行うメソッドのこと

ストリーム(Observable)を作成して「オペレータ」をかけて、出来上がったストリームを購読(subscribe)するという流れ

オペレータの戻り値が Observable になっているのでチェーンすることができる

```js
Rx.Observable.from([1, 2, 3, 4, 5, 6, 7, 8]) // 配列をストリーム(Observable)に変換
    .filter(function (num) { //戻り値：Observable
        return num % 2;
    }).map(function (num) { // 戻り値：Observable
        return num * num;
    }).forEach(function (num) {  // `forEach`は`subscribe`のエイリアスです。戻り値：Diposable
        return console.log(num);
    });
```
