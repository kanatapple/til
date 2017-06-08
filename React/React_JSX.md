# JSX

```
const element = <h1>Hello, world!</h1>;
```

これを`JSX`と呼び、JavaScriptの拡張構文  
JSXはReactの`element`を生成する

## Embedding Expressions in JSX

JSXでは中括弧で囲むことで任意のJavaScript式に埋め込むことができる

```javascript
const element = (
  <h1>Hello, world!</h1>
);
```

必須ではないが、セミコロンの自動挿入による不具合を避けるために、中括弧で囲むことをオススメする

## JSX is an Expression Too

コンパイル後、JSX式は通常のJavaScriptオブジェクトに変換される  
これは、if文・for文の中で使うことや、変数への割り当て、関数の引数として渡せたり関数で返すことが可能なことを意味する

```javascript
function getGreeting(user) {
  if (user) {
    return <h1>Hello, {formatName(user)}!</h1>;
  }
  return <h1>Hello, Stranger.</h1>;
}
```

## Specifying Attributes with JSX

クォートを使うことで文字列リテラルとして属性を指定することができる

```javascript
const element = <div tabIndex="0"></div>;
```

中括弧を使うことでJavaScript式を属性に埋め込むことができる

```javascript
const element = <img src={user.avatarUrl}></img>;
```

JavaScript式を埋め込む時に中括弧の前後にクォートを入れてはいけない  
囲んだ場合はJSXは属性を式ではなく文字列リテラルとして扱う
クォートまたは中括弧（式の場合）のどちらかを使用する必要がありますが、両方を同じ属性で使用することはできない

## Specifying Children with JSX

タグの中が空の場合、XMLのようにすぐ閉じることができる

```javascript
const element = <img src={user.avatarUrl} />;
```

JSXタグは子要素を含むこともできる

```javascript
const element = (
  <div>
    <h1>Hello!</h1>
    <h2>Good to see you here.</h2>
  </div>
);
```

JSXはHTMLよりJavaScriptに近いので、React DOMはHTMLの属性名の代わりにキャメルケースのプロパティ名を使う

## JSX Prevents Injection Attacks

JSXではユーザーの入力を埋め込んでも安全

```javascript
const title = response.potentiallyMaliciousInput;
// This is safe:
const element = <h1>{title}</h1>;
```

デフォルトでReact DOMはレンダリングする前にJSXに埋め込まれた値をエスケープする  
したがって、アプリケーションに明示的に記述されていないものを挿入することは絶対にできない  
全てレンダリングされる前に文字列に変換される

## JSX Represents Objects

BabelはJSXを`React.createElement()`呼び出しにコンパイルする  
以下の2つの例は同じ結果になる

```javascript
const element = (
  <h1 className="greeting">
    Hello, world!
  </h1>
);
```

```javascript
const element = React.createElement(
  'h1',
  {className: 'greeting'},
  'Hello, world!'
);
```

`React.createElement()`はバグのないコードを書くのに役立ついくつかのチェックを行いますが  
基本的には以下のようなオブジェクトを作成する

```javascript
const element = {
  type: 'h1',
  props: {
    className: 'greeting',
    children: 'Hello, world'
  }
};
```

このようなオブジェクトは「React elements」と呼ばれる  
Reactはこれらのオブジェクトを読み取り、それらを使ってDOMを構築し、最新の状態に保つ