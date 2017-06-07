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
