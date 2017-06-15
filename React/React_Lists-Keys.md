# Lists and Keys

Reactのリストは、JavaScriptの配列を要素のリストに変換することとほぼ一緒

## Rendering Multiple Components

要素のコレクションを作成し、それを中括弧を使うことでJSXに含めることができる

```javascript
const numbers = [1, 2, 3, 4, 5];
const listItems = numbers.map((number) => 
  <li>{number}</li>
);

ReactDOM.render(
  <ul>listItems</ul>,
  document.getelementById('root')
);
```

## Basic List Component

大抵、コンポーネントの中でリストをレンダリングする  
上記の例はこのようにリファクタリングできる

```javascript
function NumberList(props) {
  const numbers = props.numbers;
  const listItems = numbers.map((number) => 
    <li>{number}</li>
  );
  
  return (
    <ul>{listItems}</ul>
  );
}
```

このコードを実行すると、リストアイテムに`key`を設定する必要があるという警告が表示される  
`key`はリストを作成する時に設定する必要がある特別な属性

## Keys

キーはどのアイテムが変更されたか、追加されたか、削除されたかをReactが認識するのに役立つ  
要素にStableな一意性を与えるために要素にキーを与える必要がある

```javascript
const numbers = [1, 2, 3, 4, 5];
const listItems = numbers.map((number) =>
  <li key={number.toString()}>
    {number}
  </li>
);
```

キーを決定する最も良い方法は、リストの兄弟間で一意に識別する文字列を使用すること  
ほとんどの場合、データのIDをキーとして使用する

```javascript
const todoItems = todos.map((todo) => 
  <li key={todo.id}>{todo.text}</li>
);
```

StableなIDを設定できない場合は、最後の手段として配列のインデックスをキーとして使用できる

```javascript
const todoItems = todos.map((todo, index) => 
  <li key={index}>{todo.text}</li>
);
```

キーの重要性については[こちら](https://facebook.github.io/react/docs/reconciliation.html#recursing-on-children)を参照

## Extracting Components with Keys

キーは配列の周囲のコンテキストでのみ意味がある  
例えば、`<ListItem>`コンポーネントに分割した場合、`<ListItem>`内の`<li>`要素ではなく、`<ListItem>`自体にキーを与える必要がある

### Example: Incorrect Key Usage

```javascript
function ListItem(props) {
  const value = props.value;
  return (
    // Wrong! There is no need to specify the key here:
    <li key={value.toString()}>{value}</li>
  );
}

function NumberList(props) {
  const numbers = props.numbers;
  const listItems = numbers.map((number) =>
    // Wrong! The key should have been specified here:
    <ListItem value={number} />
  );
  return (
    <ul>{listItems}</ul>
  );
}
```

### Example: Correct Key Usage

```javascript
function ListItem(props) {
  // Correct! There is no need to specify the key here:
  return <li>{props.value}</li>;
}

function NumberList(props) {
  const numbers = props.numbers;
  const listItems = numbers.map((number) =>
    // Correct! Key should be specified inside the array.
    <ListItem key={number.toString()} value={number} />
  );
  return (
    <ul>{listItems}</ul>
  );
}
```

## Keys Must Only Be Unique Among Siblings

配列内で使用されるキーは、グローバルで一意である必要はないが、兄弟間で一意でなければならない  
2つの異なる配列を生成する場合、同じキーを使用することができる

```javascript
function Blog(props) {
  const sidebar = (
    <ul>
      {props.posts.map((post) =>
        <li key={post.id}>
          {post.title}
        </li>
      )}
    </ul>
  );
  const content = props.posts.map((post) =>
    <div key={post.id}>
      <h3>{post.title}</h3>
      <p>{post.content}</p>
    </div>
  );
  return (
    <div>
      {sidebar}
      <hr />
      {content}
    </div>
  );
}

const posts = [
  {id: 1, title: 'Hello World', content: 'Welcome to learning React!'},
  {id: 2, title: 'Installation', content: 'You can install React from npm.'}
];
ReactDOM.render(
  <Blog posts={posts} />,
  document.getElementById('root')
);
```

キーはReactへのヒントとして機能しますが、コンポーネントに渡されることはありません  
コンポーネントに同じ値が必要な場合は、別の名前で`props`に明示的に渡す

```javascript
const content = posts.map((post) =>
  <Post
    key={post.id}
    id={post.id}
    title={post.title} />
);
```

上記の例では、Postコンポーネントはprops.idを読み取ることができますが、props.keyは読み取ることができない

## Embedding map() in JSX

JSXでは、任意の式を中括弧で埋め込むことができるので、`map()`の結果をインライン展開できる

```javascript
function NumberList (props) {
  const numbers = props.numbers;
  return (
    <ul>
      {numbers.map((number) => 
        <ListItem key={number.toString()} value={number} />
      )}
    </ul>
  );
}
```

可読性が悪くなる時は変数に入れるのは問題ない
