# State and Lifecycle

以下のコードを例にして、コンポーネントを再利用可能にしてカプセル化する方法を学習する

```javascript
function tick() {
  const element = (
    <div>
      <h1>Hello, world!</h1>
      <h2>It is {new Date().toLocaleTimeString()}.</h2>
    </div>
  );
  ReactDOM.render(
    element,
    document.getElementById('root')
  );
}

setInterval(tick, 1000);
```

まずは見た目の部分からカプセル化する

```javascript
function Clock(props) {
  return (
    <div>
      <h1>Hello, world!</h1>
      <h2>It is {props.date.toLocaleTimeString()}.</h2>
    </div>
  );
}

ReactDOM.render(
  <Clock date={new Date()} />,
  document.getElementById('root')
);

setInterval(tick, 1000);
```

上記は、重要な事が欠けている  
それは、`Clock`がタイマーをセットし、毎秒UIを更新することは`Clock`が実行すること

理想的には以下のようにして`Clock`自体を更新させたい

```javascript
ReactDOM.render(
  <Clock />,
  document.getElementById('root')
);
```

これを実装するには`Clock`コンポーネントに`state`を追加する必要がある  
`state`は`props`に似ているが、これはプライベートで、コンポーネントによって完全にコントロールされる  
`state`はclassだけが利用できる機能です

## Converting a Function to a Class

クラスに変換するには以下の5つのステップが必要

1. `React.Component`を継承した同じ名前のES6 classを作成
2. `render()`関数を追加
3. `render()`関数の中にfunctionalの中身を移動
4. `render()`関数の中の`props`を`this.props`に置換
5. 空になったfunctionalの定義を削除

```javascript
class Clock extends React.Component {
  render() {
    return (
      <div>
        <h1>Hello, world!</h1>
        <h2>It is {this.props.date.toLocaleTimeString()}.</h2>
      </div>
    );
  }
}
```

これによって、`state`やライフサイクルフックなどの追加機能が使えるようになる

## Adding Local State to a Class

以下の3ステップで`props`から`state`に`date`を移す

1. `this.props.date`を`this.state.date`に置換

```javascript
class Clock extends React.Component {
  render() {
    return (
      <div>
        <h1>Hello, world!</h1>
        <h2>It is {this.state.date.toLocaleTimeString()}.</h2>
      </div>
    );
  }
}
```

2. `this.state`の初期値を代入するクラスコンストラクタを追加する

```javascript
class Clock extends React.Component {
  constructor (props) {
    super(props);
    this.state = { date: new Date() };
  }
  
  render() {
    return (
      <div>
        <h1>Hello, world!</h1>
        <h2>It is {this.state.date.toLocaleTimeString()}.</h2>
      </div>
    );
  }
}
```

classコンポーネントは常に、ベースコンストラクタを`props`で呼び出す必要がある

3. `<Clock />`要素から`date`属性を削除

```javascript
ReactDOM.render(
  <Clock />,
  document.getElementById('root')
);
```

## Adding Lifecycle Methods to a Class

多くのコンポーネントを持つアプリケーションではコンポーネントが破棄された時にリソースを解放することが重要

上記の`Clock`は最初にDOMにレンダリングされる時にタイマーがセットされる必要がある  
Reactではこれを`mounting`と呼ぶ

また、DOMから`Clock`が削除される時はタイマーをクリアする必要がある  
Reactではこれを`unmounting`と呼ぶ

コンポーネントがマウントされたりアンマウントされる時にコードを実行するための特別なメソッドを宣言できる

```javascript
class Clock extends React.Component {
  constructor (props) {
    super(props);
    this.state = { date: new Date() };
  }
  
  componentDidMount() {
  }
  
  componentWillUnmount() {
  }
  
  render() {
    return (
      <div>
        <h1>Hello, world!</h1>
        <h2>It is {this.state.date.toLocaleTimeString()}.</h2>
      </div>
    );
  }
}
```

これらの関数を「ライフサイクルフック」と呼ぶ

`componentDidMount()`はコンポーネントがDOMにレンダリングされた後に実行される  
`componentWillUnmount()`はコンポーネントがDOMから削除された後に呼ばれる

```javascript
componentDidMount() {
  this.timerID = setInterval(
    () => this.tick(),
    1000
  );
}

componentWillUnmout() {
  clearInterval(this.timerID);
}
```

タイマーIDの設定の仕方に注意する  
`this.props`はReact自身によって設定され、`this.state`には特別な意味があるので、視覚的な出力に使用されていないものを保存する必要がある場合は、手動でクラスにフィールドを追加する

最後に毎秒実行される`tick()`を実行する  
`this.setState()`を使って`state`を更新させる

```javascript
class Clock extends React.Component {
  constructor(props) {
    super(props);
    this.state = {date: new Date()};
  }

  componentDidMount() {
    this.timerID = setInterval(
      () => this.tick(),
      1000
    );
  }

  componentWillUnmount() {
    clearInterval(this.timerID);
  }

  tick() {
    this.setState({
      date: new Date()
    });
  }

  render() {
    return (
      <div>
        <h1>Hello, world!</h1>
        <h2>It is {this.state.date.toLocaleTimeString()}.</h2>
      </div>
    );
  }
}

ReactDOM.render(
  <Clock />,
  document.getElementById('root')
);
```

## Using State Correctly

`setState()`について知っておくべきことが3つある

### Do Not Modify State Directly

`state`を直接編集しない

```javascript
// Wrong
this.state.comment = 'Hello';

// Correct
this.setState({comment: 'Hello'});
```

`this.state`に設定していいのは`constructor`でのみ

### State Updates May Be Asynchronous 

`state`は非同期に更新される  
Reactはパフォーマンスのために複数の`setState()`を呼び出しを一括にまとめて呼び出す  
`props`や`state`は非同期で更新される可能性があるので、計算するために使用すべきではない

```javascript
// Wrong
this.setState({
  counter: this.state.counter + this.props.increment,
});
```

これを解決する方法として、`setState()`にオブジェクトではなく関数を渡す  
この関数は第1引数に前回の状態を受け取り、第2引数に更新された`props`を受け取る

```javascript
// Correct
this.setState((prevState, props) => ({
  counter: prevState.counter + props.increment
}));

// Correct
this.setState(function(prevState, props) {
  return {
    counter: prevState.counter + props.increment
  };
});
```

### State Updates are Merged

`state`の更新はマージされる

`setState()`を呼ぶとReactは現在の状態にオブジェクトをマージする

例えば、以下のように`constructor`で`state`をセットし

```javascript
constructor(props) {
  super(props);
  this.state = {
    posts: [],
    comments: []
  };
}
```

それから別々に`setState()`を呼ぶと

```javascript
componentDidMount() {
  fetchPosts().then(response => {
    this.setState({
      posts: response.posts
    });
  });

  fetchComments().then(response => {
    this.setState({
      comments: response.comments
    });
  });
}
```

この場合、`this.setState({comments})`では`this.state.posts`は損なわれないが  
`this.state.comments`は完全に置き換えられる

## The Data Flows Down

親コンポーネントも子コンポーネントも特定のコンポーネントがステートフルかステートレスか知ることができないし  
関数、classで定義されているかどうかも気にしない

これが`state`はローカルまたはカプセル化と呼ばれる理由です  
`state`を所有しセットしたコンポーネント以外、その`state`にアクセスすることはできない

コンポーネントは子コンポーネントの`props`として`state`を渡すことができる

```javascript
<h2>It is {this.state.date.toLocaleTimeString()}.</h2>
```

ユーザー定義のコンポーネントでも機能する

```javascript
<FormattedDate date={this.state.date} />

// このコンポーネントはpropsのdateを受け取るが、それがどこから渡されたものかは知ることができない
function FormattedDate(props) {
  return <h2>It is {props.date.toLocaleTimeString()}.</h2>;
}
```

これは通常、「トップダウン」または「単方向」データフローと呼ばれる  
すべての`state`は常に特定のコンポーネントによって所有され、その`state`から派生したデータまたはUIはツリー内のコンポーネントの「下」にしか影響を与えない

