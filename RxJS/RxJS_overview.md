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