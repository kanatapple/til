# MobX

Simple, scalable state management

## インストール

```
npm install mobx
```

Reactでバインディングするためには`mobx-react`もインストールする

```
npm install mobx-react
```

ESNextのデコレータを使用できる

## Introduction

MobXはいろいろなプロダクションコードで使われているライブラリで、透過的にリアクティブプログラミングを適用することにより  
状態管理をシンプルかつスケーラビリティにする

```
Anything that can be derived from the application state, should be derived. Automatically.
```

![](https://mobx.js.org/docs/flow.png)

ReactとMobXは強力な組み合わせ  
Reactは、レンダリング可能なコンポーネントのツリーに変換するメカニズムを提供することによって、アプリケーションの状態をレンダリングする  
MobXはReactが使用するアプリケーションの状態を格納、更新するメカニズムを提供する

ReactとMobXはどちらも、アプリケーション開発の一般的な問題に対する非常に最適でユニークなソリューションを提供する  
Reactは高コストなDOM変更を減らすために仮想DOMを使用してUIを最適にレンダリングを提供する  
MobXは、必要なときのみ更新され、失効することのないリアクティブ仮想依存状態グラフを使用して、アプリケーションの状態をReactコンポーネントと最適に同期させるメカニズムを提供する

## Core concepts

MobXはいくつかのコンセプトを持っている

### Observable state

MobXは、オブジェクト、配列、クラスインスタンスなどの既存のデータ構造にobservableな機能を追加する  
ES.Nextの`@observable`デコレータでプロパティを注釈することで可能になる

```javascript
class Todo {
    id = Math.random();
    @observable title = "";
    @observable finished = false;
}
```

`observable`を使用するということは、オブジェクトのプロパティをスプレッドシートのセルに変換するということ  
しかし、スプレッドシートと異なり、プリミティブ値だけでなく、参照、オブジェクト、配列もobservableになりえる  
独自のデータソースもobservableに定義することもできる

### Intermezzo: Using MobX in ES5, ES6 and ES.next environments

気持ち悪く見える`@`はES.Nextのデコレータ  
MobXではデコレータを使用することは任意  
MobXはES5でも動くが、デコレータなどES.Nextを活用することはMobXを使う上で有用なこと

```javascript
function Todo() {
    this.id = Math.random()
    extendObservable(this, {
        title: "",
        finished: false
    })
}
```

### Computed values

MobXでは、`@computed`デコレータと`getter`/`setter`関数を使うことで、関連するデータが変更されたときに自動的に導出される値を定義することができます

```javascript
class TodoList {
    @observable todos = [];
    @computed get unfinishedTodoCount() {
        return this.todos.filter(todo => !todo.finished).length;
    }
}
```

MobXは`todo`が追加されたり`finished`プロパティが変更された時、自動的に`unfinishedTodoCount`が更新されるようにする  
このような計算はMS Excelなどのスプレッドシートの数式と非常によく似ていて、必要な時に自動的に計算される

### Reactions

`Reaction`は`Computed value`と似ているが、新しい値を生成する代わりに、コンソールへの描画、ネットワークリクエスト、DOMへパッチを当てるためにReactコンポーネントを段階的に更新するなどの副作用を与える  
要するに、`Reaction`は反応的で命令的なプログラミングを橋渡しする

#### React components

もしReactを使っている場合、`mobx-react`パッケージから`observer`関数/デコレータを追加するだけで、コンポーネント(or functionalコンポーネント)をリアクティブコンポーネントに変換できる

```javascript
import React, {Component} from 'react';
import ReactDOM from 'react-dom';
import {observer} from "mobx-react";

@observer
class TodoListView extends Component {
    render() {
        return <div>
            <ul>
                {this.props.todoList.todos.map(todo =>
                    <TodoView todo={todo} key={todo.id} />
                )}
            </ul>
            Tasks left: {this.props.todoList.unfinishedTodoCount}
        </div>
    }
}

const TodoView = observer(({todo}) =>
    <li>
        <input
            type="checkbox"
            checked={todo.finished}
            onClick={() => todo.finished = !todo.finished}
        />{todo.title}
    </li>
)

const store = new TodoList();
ReactDOM.render(<TodoListView todoList={store} />, document.getElementById('mount'));
```

`observer`はコンポーネントをレンダリングするデータの派生物へ変換する

#### Custom reactions

`Custom reaction`は、`autorun`、`autorunAsync`、または特定の状況に合わせた関数を使用して簡単に作成できる

例えば、以下の`autorun`では、`unfinishedTodoCount`の値が変わるたびにログメッセージが出力される

```javascript
autorun(() => {
  console.log('Task left: ' + todos.unfinishedTodoCount);
});
```

### What will MobX react to?

`unfinishedTodoCount`が変更するたびに値が表示されるのはなぜか？  
その答えは、「MobXは追跡されている関数の実行中に読み取られる既存のobservableなプロパティに反応する」から  

### Actions

多くのflux frameworkと異なり、MobXはユーザーのイベントをどのように扱うかは考慮しない
- Fluxのような方法で行える
- RxJSを使用してイベントを処理できる
- 上記のonClickハンドラで説明したように、イベントを最も簡単に処理することで可能

状態を更新した後、MobXは効率的で不具合のない方法で残りの部分を処理する  
そのため、以下のような簡単なステートメントだけで、ユーザーインターフェイスを自動的に更新できる

イベント発生させたり、`dispatcher`を呼び出すなどの技術的な必要はない  
Reactコンポーネントは最終的に状態を表現したものにすぎない

```javascript
store.todos.push(
    new Todo("Get Coffee"),
    new Todo("Write simpler code")
);
store.todos[0].finished = true;
```

## MobX: Simple and scalable

MobXは状態管理ライブラリの中で最もobtrusive(目立つ、目障り)  
MobXのアプローチはシンプルなだけでなく、スケーラビリティにも優れている

### Using classes and real references

MobXではデータの正規化が必要ない  
これにより、非常に複雑なドメインモデルに適したライブラリになる

### Referential integrity is guaranteed

データを正規化する必要ないため、MobXは状態と結果の関係を自動的に追跡し、参照整合性を無理なく得られる  
MobXはそれらを追跡し、参照のいずれかが変更されるたびに再レンダリングする  
稀に、あるデータの変更が一見無関係のコンポーネントに影響するかもしれないことを忘れるかもしれないが、MobXは忘れることはない

### Simpler actions are easier to maintain

上で示したように、MobXを使用した時に状態を変更することは非常に簡単  
単に変更を記述すれば、あとはMobXが処理してくれる

### Fine grained observability is efficient

MobXは、紛失を防ぐためにアプリケーションの全ての派生物のグラフを作成して、再計算の回数を最小限に抑える  
「Derive everything」は高負荷になるかもしれないが、MobXは状態に同期して結果を維持し、必要な再計算の数を最小限に抑えるために仮想導出グラフを作成する

### Easy interoperability

MobXはプレーンなJavaScript構造で動作する  
MobX特有の記述を必要としないので、ほとんどのJavaScriptライブラリと一緒に動作する  
したがって、既存のルーター、`react-router`、`director`、`superagent`、`lodash`などのデータフェッチ、ユーティリティライブラリをそのまま使用できる  
同じ理由から、サーバー側、クライアント側、isomorphicアプリケーション、`react-native`でも使用できる  

