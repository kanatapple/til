# Components and Props

コンポーネントを使用すると、UIを独立した再利用可能な部分に分割し、各部分について個別に考えることができる

概念的にはコンポーネントはJavaScriptの関数のようなもの  
これらは任意の入力(propsと呼ぶ)を受け取り、画面に何が表示されるかが記述されているReact Elementsを返す

## Functional and Class Components

コンポーネントを定義する一番シンプルな方法はJavaScript関数を記述すること

```javascript
function Welcome(props) {
  return <h1>Hello, {props.name}</h1>
}
```

この関数は、単一の引数(propsオブジェクト)を受け取り、React Elementsを返すので、有効なReactコンポーネント  
このようなコンポーネントは文字通りJavaScript関数なので「functional」と呼ばれる

また、ES6のclassを使ってコンポーネントを定義することもできる

```javascript
class Welcome extends React.Component {
  render() {
    return <h1>Hello, {this.props.name}</h1>;
  }
}
```

上記2つのコンポーネントはReactの観点からは同じもの  
classにはいくつかの追加機能がある

## Rendering a Component

以前は、以下のようにDOMタグを表すReact Elementsしか使えなかった

```javascript
const element = <div />;
```

現在ではユーザー定義のコンポーネントも使うことができる

```javascript
const element = <Welcome name="Alice" />;
```

Reactはユーザー定義のコンポーネントを見つけると、JSXの属性を単一のオブジェクトとしてコンポーネントに渡す  
このオブジェクトを「props」と呼ぶ

```javascript
function Welcome(props) {
  return <h1>Hello, {props.name}</h1>;
}

const element = <Welcome name="Sara" />;
ReactDOM.render(
  element,
  document.getElementById('root')
);
```

この例では以下のことが起きている
1. `<Welcome name="Sara" />`という要素で`ReactDOM.render()`を呼ぶ
2. Reactは`{name: 'Sara'}`という`props`オブジェクトとともに`Welcome`コンポーネントを呼ぶ
3. `Welcome`コンポーネントは結果として`<h1>Hello, Sara</h1>`を返す
4. React DOMは`<h1>Hello, Sara</h1>`とマッチするようにDOMを更新する

※注意  
コンポーネントは常に大文字から始めること  
例えば、小文字で始まるものはDOMのタグを表し、大文字で始まるものはコンポーネントを表す

## Composing Components

コンポーネントは出力する際に他のコンポーネントを参照できる

例えば以下のように`Welcome`コンポーネントをたくさんレンダリングする`App`コンポーネントを作れる

```javascript
function Welcome(props) {
  return <h1>Hello, {props.name}</h1>;
}

function App() {
  return (
    <div>
      <Welcome name="Sara" />
      <Welcome name="Cahal" />
      <Welcome name="Edite" />
    </div>
  );
}

ReactDOM.render(
  <App />,
  document.getElementById('root')
);
```

通常、新規のReactアプリケーションでは一番上にAppコンポーネントを持つ

## Extracting Components

コンポーネントをより小さなコンポーネントへ分割することを恐れないこと

```javascript
function Comment(props) {
  return (
    <div className="Comment">
      <div className="UserInfo">
        <img className="Avatar"
          src={props.author.avatarUrl}
          alt={props.author.name}
        />
        <div className="UserInfo-name">
          {props.author.name}
        </div>
      </div>
      <div className="Comment-text">
        {props.text}
      </div>
      <div className="Comment-date">
        {formatDate(props.date)}
      </div>
    </div>
  );
}
```

例えば上記のようなコンポーネントの場合、以下のように分割し

```javascript
function Avatar(props) {
  return (
    <img className="Avatar"
      src={props.user.avatarUrl}
      alt={props.user.name}
    />
  );
}

function UserInfo(props) {
  return (
    <div className="UserInfo">
      <Avatar user={props.user} />
      <div className="UserInfo-name">
        {props.user.name}
      </div>
    </div>
  );
}
```

上記コンポーネントを使うことでシンプルにすることができる

```javascript
function Comment(props) {
  return (
    <div className="Comment">
      <UserInfo user={props.author} />
      <div className="Comment-text">
        {props.text}
      </div>
      <div className="Comment-date">
        {formatDate(props.date)}
      </div>
    </div>
  );
}
```

複数回使い回されているものや、コンポーネント自体が複雑なものはコンポーネントを抽出する

## Props are Read-Only

コンポーネントを関数またはclassのどちらで定義したとしても、`props`を変更してはならない

例えば、以下の関数は純関数(pure function)と呼ばれる

```javascript
function sum(a, b) {
  return a + b;
}
```

Reactコンポーネントは純関数(pure function)のように`props`を変更しないようにする