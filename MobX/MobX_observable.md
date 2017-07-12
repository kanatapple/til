# Observable

## 使い方

```
observable(value)
@observable classProperty = value
```

Observableな値にはJavaScriptの基本型、参照、オブジェクト、クラスインスタンス、配列、マップなどがある  
次の変換ルールが適用されるが、modifierを使うことで微調整できる
- valueがES6のMapの場合：`Observable`なMapが返される。`Observable Map`は、特定のエントリの変更だけでなく、エントリの追加や削除に反応させたくない場合に非常に便利
- valueが配列の場合：`Observable`な配列が返される
- valueがprototypeを持たないオブジェクトの場合：プロパティが全てオブザーバブルな値になる
- valueがprototypeを持つオブジェクト、JavaScriptの基本型、関数の場合：`Boxed Observable`が返される  
MobXは自動的にprototypeがオブザーバブルになるオブジェクトを生成しない、それはそのコンストラクタの責任だから  
クラス定義で`@observable`を使う代わりに、コンストラクタで`extendObservable`を使う

これらのルールは一見複雑に思えるかもしれませんが、実際には直感的に作業することができる

- 動的にキーを設定するオブジェクトを作る場合は`Observable Map`を使う
オブジェクトの静的なプロパティのみ観測可能だが、動的なものには`extendObservable`を使う
- `@observable`デコレータを使うには、デコレータがトランスパイラで有効になってることを確認する
- デフォルトで、データ構造をオブザーバブル化することは伝染的になる。これは、データ構造に含まれているか、または将来データ構造に含まれる任意の値が自動的にオブザーバブル化することを意味する

```javascript
const map = observable.map({ key: "value"});
map.set("key", "new value");

const list = observable([1, 2, 4]);
list[2] = 3;

const person = observable({
    firstName: "Clive Staples",
    lastName: "Lewis"
});
person.firstName = "C.S.";

const temperature = observable(20);
temperature.set(25);
```

# @observable

ES7、TypeScriptで使えるデコレータはクラスのプロパティをオブザーバブル化することができる  
`@observable`はインスタンスフィールドと`getter`プロパティに使える  
これによって、オブジェクトのどの部分がオブザーバブルかをきめ細かく制御することができる

```javascript
import {observable} from "mobx";

class OrderLine {
    @observable price = 0;
    @observable amount = 1;

    @computed get total() {
        return this.price * this.amount;
    }
}
```

もしデコレータやフィールド初期化をサポートしてないなら、`@observable key = value;`を`extendObservable(this, { key: value })`のように書ける

※全てのプロパティはアクセスされると遅延的に定義されていく

```javascript
const line = new OrderLine();
console.log("price" in line); // true
console.log(line.hasOwnProperty("price")); // false, the price _property_ is defined on the class, although the value will be stored per instance.
line.amount = 2;
console.log(line.hasOwnProperty("price")); // true, now all the properties are defined on the instance
```

`@observable`デコレータは`asStructure`のような修飾子と組み合わせることができる

```javascript
@observable position = asStructure({ x: 0, y: 0})
```

## Enabling decorators in your transpiler

デコレータはES標準になるまでは、TypeScriptやBabelでデフォルトではサポートしていない
- TypeScriptでは、`--experimentalDecorators`フラグを有効にするか、`tsconfig.json`で`experimentalDecorators`を`true`にセットする
- Babel5では、Babel CLIに`--stage 0`を渡す
- Babel6では、[issue](https://github.com/mobxjs/mobx/issues/105)で指示されている設定例を参考
