# Router v4

## Install

React Routerは3つのパッケージ(react-router、react-router-dom、react-router-native)に分割された  
`react-router`はコアルーティングコンポーネントと機能を提供し、他の2つは環境固有のコンポーネントを提供する

```
npm install react-router-dom
```

## The Router

ブラウザベースのプロジェクトの場合、`<BrowserRouter>`と`<HashRouter>`コンポーネントがある  
動的なWebサイトでは`<BrowserRouter>`、静的なWebサイトでは`<HashRouter>`を使う必要がある

通常は`<BrowserRouter>`を使うことをオススメするが、静的なファイルだけを扱うサーバーでWebサイトをホストする場合は  
`<HashRouter>`が良い解決策

### history

それぞれのルータは`history`オブジェクトを作成する  
これは現在の場所を追跡したり、変更するたびにウェブサイトを再レンダリングするために使われる

### Rendering a <Router>

ルーターコンポーネントはただ１つの子要素のみ受け付ける  
この制限内で動かすために、`<App>`という残りのアプリケーションをレンダリングするコンポーネントを作成するのが一般的

```javascript
import { BrowserRouter } from 'react-router-dom'
ReactDOM.render((
  <BrowserRouter>
    <App />
  </BrowserRouter>
), document.getElementById('root'));
```

## The <App>

アプリケーションを`<App>`コンポーネントに定義することにする  
簡略化のため、2つの部分に分割する  
1つ目は`<Header>`コンポーネントで、Webサイト全体をナビゲートするリンクが含まれる  
2つ目は`<Main>`コンポーネントで、残りのコンテンツがレンダリングされる場所

```javascript
const App = () => (
  <div>
    <Header />
    <Main />
  </div>
);
```

好きなようにアプリケーションをレイアウトできるが、ルートとナビゲーションを分けるといい

## Routes

`<Route>`コンポーネントはReact Routerの主要なビルディングブロック  
URLのパス名と一致するものだけレンダリングしたい場合は、`<Route>`要素を作成する必要がある

### Path

`<Route>`は`path`を受け取り、`path`にはルートが一致するパス名を文字列で指定する  
現在の場所のパス名が`path`と一致すると、ルートはReact Elementをレンダリングする  
パスが一致しない場合は、ルートは何もレンダリングしない

```javascript
<Route path='/roster' />
// when the pathname is '/', the path does not match
// when the pathname is '/roster' or '/roster/2', the path matches

// If you only want to match '/roster', then you need to use
// the "exact" prop. The following will match '/roster', but not
// '/roster/2'
<Route exact path='/roster' />
```

### Matching paths

`path-to-regexp`パッケージはルート要素の`path`属性が現在の場所と一致するかどうかを判定するのに使われる  
パスの文字列を正規表現にコンパイルし、現在の場所のパス名と照合される  
パスの文字列には高度な書式設定オプションがある([path-to-regexp documentation](https://github.com/pillarjs/path-to-regexp)参照)

ルートのパスが一致すると以下のプロパティを持つ`match`オブジェクトが作成される
- url － 現在の場所のパス名の一致部分
- path － ルートのパス
- isExact － path == pathname
- params － path-to-regexpによって取得されたpathnameの値を含むオブジェクト

## Creating our routes

`<Route>`はルーター内部であればどこにでも作成できるが、同じ場所にレンダリングすることには意味がある  
`<Switch>`コンポーネントを使用して、`<Route>`をグループ化することができる  
`<Switch>`は子要素を繰り返し処理し、現在のパス名に一致する最初の要素のみレンダリングする

## What does the <Route> render?

`<Route>`には3つの属性があり、ルートのパスが一致した時に何をレンダリングすべきかを定義するのに使用できる  
`<Route>`要素に提供されるのは1つだけ

1. component － Reactコンポーネント。`React.createElement`によって作成されたReactコンポーネントである新しい要素を返す
2. render － React要素を返す関数。コンポーネントと似ているが、インラインレンダリングや、要素に`props`を渡すのに便利
3. children － React要素を返す関数。上2つとは異なり、パスが一致するかどうかに関係なく、常にレンダリングされる

```javascript
<Route path='/page' component={Page} />

const extraProps = { color: 'red' }
<Route path='/page' render={(props) => (
  <Page {...props} data={extraProps}/>
)}/>

<Route path='/page' children={(props) => (
  props.match
    ? <Page {...props}/>
    : <EmptyPage {...props}/>
)}/>
```

通常、`component`か`render`属性のいずれかを使用する  
`children`は時々役に立つかもしれないが、通常、パスが一致しない時は何も描画しないことは望ましい

`<Route>`でレンダリングされた要素には、いくつかの`props`が渡される
`match`オブジェクト、`location`オブジェクト、`history`オブジェクトの3つ

## Nested Routes

```javascript
import { Switch, Route } from 'react-router-dom'
const Main = () => (
  <main>
    <Switch>
      <Route exact path='/' component={Home}/>
      <Route path='/roster' component={Roster}/>
      <Route path='/schedule' component={Schedule}/>
    </Switch>
  </main>
)
```

上記の例では、`/roster/:number`のルートが含まれていない  
代わりに、パス名が`/roster`で始まる時に`<Roster>`コンポーネントがレンダリングされる

`<Roster>`コンポーネント内では2つのパスのルートがレンダリングされる

1. `/roster` － パス名が`/roster`の場合のみに一致させる必要があるので、`exact`属性を与える必要がある
2. `/roster/:number` － `/roster`の後に来るパス名の部分を取得してパスパラメータに使用される

```javascript
const Roster = () => (
  <Switch>
    <Route exact path='/roster' component={FullRoster}/>
    <Route path='/roster/:number' component={Player}/>
  </Switch>
)
```

上記のように、共通のプレフィックスを持つルートを同じコンポーネントにグループ化すると便利  

## Path Params

パス名の中の値をキャプチャしたい時がある  
これを行うには、ルートのパス文字列にパスパラメータを追加することでできる

`/roster/:number`の`:number`の部分は、`/roster/`に続くパス名の部分がキャプチャされ、`match.params.number`として格納されることを意味する

例えば、`/roster/6`の場合、以下の`params`オブジェクトが作成される

```javascript
{ number: '6' } // キャプチャされた値は文字列になることに注意
```

## Links

アンカーを使うとページがリロードしてしまうが、React Routerはそれを防ぐために`<Link>`コンポーネントを提供する  
`<Link>`をクリックするとページをリロードせずに、URLを更新し、コンテンツをレンダリングする

```javascript
import { Link } from 'react-router-dom'
const Header = () => (
  <header>
    <nav>
      <ul>
        <li><Link to='/'>Home</Link></li>
        <li><Link to='/roster'>Roster</Link></li>
        <li><Link to='/schedule'>Schedule</Link></li>
      </ul>
    </nav>
  </header>
)
```

`<Link>`は`to`属性を使用して、移動する場所を記述する  
これは、文字列またはロケーションオブジェクト(pathname、search、hash、`state`属性の組み合わせを含む)です  
文字列の場合は、ロケーションオブジェクトに変換される