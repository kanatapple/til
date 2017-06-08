# Rendering Elements

ElementsはReactアプリケーションの最小ビルディングブロック  
Elementsは画面上に表示させたいものを記述する

```javascript
const element = <h1>Hello, workd</h1>;
```

BrowserのDOM要素とは異なり、React Elementsは単純なオブジェクトで作成するのが容易  
React DOMはReact Elementsと一致するようにDOMを更新する

## Rendering an Element into the DOM

以下のように、HTMLの中に`<div>`があるとします

```html
<div id="root"></div>
```

React DOMによってこのタグの内部の全てが管理されるので、これをルートDOMと呼びます  
Reactだけで構築されたアプリケーションは通常単一のルートDOMを持つが、既存のアプリケーションにReactを導入する場合  
複数の独立したルートDOMを持つことができる

React ElementsをルートDOMにレンダリングするには、`ReactDOM.render()`に両方渡す

```javascript
const element = <h1>Hello, world</h1>;
ReactDOM.render(
  element,
  document.getElementById('root')
);
```

## Updating the Rendered Element

React Elementsは不変  
一度要素を作成したら子要素や属性を変更することはできない  
要素はムービーの1つのフレームのようなもので、ある時点のUIを表す

これまでの知識では、UIを更新する唯一の方法は新しい要素を作成し、それを`ReactDOM.render()`に渡すこと

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

## React Only Updates What's Necessary

React DOMは要素と子要素を以前のものと比較し、DOMを望ましい状態にするために、必要な部分のみ更新を適用する  
UIツリー全体を記述した要素を作成しても、内容が変更された部分だけがReact DOMによって更新される