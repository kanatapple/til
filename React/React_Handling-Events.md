# Handling Events

React ElementsのイベントハンドリングはDOM要素のイベントハンドリングに似ている  

いくつかの構文的な違いがある
- Reactのイベントは`lowercase`ではなく`camelCase`を使う
- JSXではイベントハンドラーとして文字列ではなく関数を渡す

例えばHTMLの場合

```html
<button onclick="activateLasers()">
  Active Lasers
</button>
```

Reactではわずかに異なって

```javascript
<button onClick={activateLasers}>
  Active Lasers
</button>
```

となる

その他の違いはReactではデフォルトの動作をキャンセルする場合、`false`を返すのではなく明確に`preventDefault`を呼ばなければならない  
例えば、HTMLではリンクのデフォルトの動作である新しいページを開くことをキャンセルさせる場合

```html
<a href="#" onclick="console.log('The link was clicked.'); return false">
  Click me
</a>
```

と記述するが、Reactの場合は以下のようになる

```javascript
function ActionLink() {
  function handleClick(e) {
    e.preventDefault();
    console.log('The link was clicked.');
  }
  
  return (
    <a href="#" onClick={handleClick}>
      Click me
    </a>
  );
}
```

ここで、`e`は合成されたイベントである  
Reactは、W3C仕様に従ってこれらの合成イベントを定義するので、ブラウザ間の互換性について心配する必要はない

Reactを使う場合、リスナーを登録するためにDOM要素が作られた後に`addEventListener`を呼ぶ必要はない  
代わりに要素が最初にレンダリングされる時にリスナーを渡すだけ

ES6 classを使ってコンポーネントを定義する場合、一般的なパターンとしてイベントハンドラーはクラスメソッドになる  
以下のコンポーネントは「ON」と「OFF」の状態を切り替えるボタンをレンダリングする

```javascript
class Toggle extends React.Component {
  constructor (props) {
    super(props);
    this.state = { isToggleOn: true };
    this.handleClick = this.handleClick.bind(this);
  }
  
  handleClick() {
    this.setState((prevState) => ({
      isToggleOn: !prevState.isToggleOn
    }));
  }
  
  render () {
    return (
      <button onClick={handleClick}>
        {this.state.isToggleOn ? 'ON' : 'OFF'}
      </button>
    );
  }
}
```

JSXコールバックでは`this`の扱いに注意する必要がある  
JavaScriptではクラスメソッドはデフォルトでバインドされない  
もし、`this.handleClick`にバインドするのを忘れると、関数が呼び出された時、`this`は`undefined`になる  
これはReact特有の挙動ではなくJavaScript関数の挙動の一部  
一般的に、`onClick={this.handleclick}`のように、後ろに()`を付けずにメソッドを参照する場合は、そのメソッドをバインドする必要がある

`bind`を呼ぶ以外の2つの解決策がある

プロパティ初期化子の構文が使える場合、これを使うことでコールバックを正しくバインドできる

```javascript
class LoggingButton extends React.Component {
  handleClick = () => {
    console.log('this is:', this);
  };

  render() {
    return (
      <button onClick={this.handleClick}>
        Click me
      </button>
    );
  }
}
```

この構文は`Create React App`ではデフォルトで有効になっている

プロパティ初期化子の構文が使えない場合、コールバックでアロー関数を使うことができる

```javascript
class LoggingButton extends React.Component {
  handleClick() {
    console.log('this is:', this);
  }

  render() {
    return (
      <button onClick={(e) => this.handleClick(e)}>
        Click me
      </button>
    );
  }
}
```

この構文の問題は、コンポーネントがレンダリングされるたびに異なるコールバックが作成されること  
大抵の場合問題ないが、このコールバックが下位コンポーネントの`prop`として渡された場合、これらのコンポーネントは余分な再レンダリングを行う可能性がある  
この種のパフォーマンス上の問題を避けるため、コンストラクタでバインドするか、プロパティ初期化子構文を使用することをお勧めする
