# Context

`v15.5以降、React.PropTypesは非推奨。その代わりにcontextTypesを定義するprop-typesライブラリを使う`

Reactでは、コンポーネントを通るデータの流れを追跡するのは容易  
場合によっては、`props`を手動で渡すことなく、コンポーネントツリーを通してデータを渡したくなる  
Reactでは`Context API`を使うことで直接これを行うことができる

## Why Not To Use Context

多くのアプリケーションではコンテキストを使用する必要はない  
Stableなアプリケーションにしたい場合は、Contextを使用しない方がいい  
これは実験的なAPIであり、今後のReactでは変更される可能性がある

ReduxやMobXのような状態管理ライブラリに精通していない場合は、コンテキストを使用しない方がいい  
多くの実用的なアプリケーションでは、これらのライブラリとReactのバインディングは、多くのコンポーネントに関する状態を管理するのに適している  
コンテキストが正しい解決策である以上に、Reduxは問題を解決する正しい解決策である可能性が高い  

経験豊富なReact開発者でない場合は、コンテキストを使用しない方がいい  
通常、`props`と`state`だけを使用して機能を実装する良い方法がある


それでもコンテキストを使用しない場合は、コンテキストの使用を小さな領域に分けて、できるだけコンテキストAPIを直接使用しないようにして、APIが変更されたときにアップグレードする方が簡単

## How To Use Context

次のような構造を持っているとする

```javascript
class Button extends React.Component {
  render() {
    return (
      <button style={{background: this.props.color}}>
        {this.props.children}
      </button>
    );
  }
}

class Message extends React.Component {
  render() {
    return (
      <div>
        {this.props.text} <Button color={this.props.color}>Delete</Button>
      </div>
    );
  }
}

class MessageList extends React.Component {
  render() {
    const color = "purple";
    const children = this.props.messages.map((message) =>
      <Message text={message.text} color={color} />
    );
    return <div>{children}</div>;
  }
}
```

この例では、ButtonとMessageコンポーネントのスタイルを適用するために、`props`を通して手動でカラーを渡している  
コンテキストを使用すると、自動的にこれをツリーに渡すことができます

```javascript
const PropTypes = require('prop-types');

class Button extends React.Component {
  render() {
    return (
      <button style={{background: this.context.color}}>
        {this.props.children}
      </button>
    );
  }
}

Button.contextTypes = {
  color: PropTypes.string
};

class Message extends React.Component {
  render() {
    return (
      <div>
        {this.props.text} <Button>Delete</Button>
      </div>
    );
  }
}

class MessageList extends React.Component {
  getChildContext() {
    return {color: "purple"};
  }

  render() {
    const children = this.props.messages.map((message) =>
      <Message text={message.text} />
    );
    return <div>{children}</div>;
  }
}

MessageList.childContextTypes = {
  color: PropTypes.string
};
```

childContextTypesとgetChildContextをMessageList（コンテキストプロバイダ）に追加することで、Reactは自動的に情報を渡し、サブツリー（この場合はButton）のどのコンポーネントもcontextTypesを定義することで情報にアクセスできる  
contextTypesが定義されていない場合、contextは空のオブジェクトになる

## Parent-Child Coupling

コンテキストでは、親と子が通信するAPIを構築することもできる  
例えば、React Router v4がこのように動作するライブラリの1つ

```javascript
import { BrowserRouter as Router, Route, Link } from 'react-router-dom';

const BasicExample = () => (
  <Router>
    <div>
      <ul>
        <li><Link to="/">Home</Link></li>
        <li><Link to="/about">About</Link></li>
        <li><Link to="/topics">Topics</Link></li>
      </ul>

      <hr />

      <Route exact path="/" component={Home} />
      <Route path="/about" component={About} />
      <Route path="/topics" component={Topics} />
    </div>
  </Router>
);
```

`Router`コンポーネントから渡された情報は、各`Link`と`Route`は`Router`に含まれている情報を受け渡しできる

## Referencing Context in Lifecycle Methods

`contextTypes`がコンポーネント内に定義されている場合、以下のライフサイクルメソッドは追加の引数`context`を受け取る

- constructor(props, context)
- componentWillReceiveProps(nextProps, nextContext)
- shouldComponentUpdate(nextProps, nextState, nextContext)
- componentWillUpdate(nextProps, nextState, nextContext)
- componentDidUpdate(prevProps, prevState, prevContext)

## Referencing Context in Stateless Functional Components

ステートレスなfunctionalコンポーネントも、関数のプロパティに`contextTypes`が定義されている場合、`context`を受け取ることができる  

```javascript
const PropTypes = require('prop-types');

const Button = ({children}, context) =>
  <button style={{background: context.color}}>
    {children}
  </button>;

Button.contextTypes = {color: PropTypes.string};
```

## Updating Context

してはいけない  
Reactはコンテキストを更新するAPIを持っているが、基本的に破壊的なので、使うべきではない  
`getChildContext`は`state`や`props`が変更された時に呼び出される  
コンテキスト内のデータを更新するには、`this.setState`を使用してローカル状態の更新がトリガとなる  
これは新しいコンテキストをトリガし、子コンポーネントも変化を受け取る

```javascript
const PropTypes = require('prop-types');

class MediaQuery extends React.Component {
  constructor(props) {
    super(props);
    this.state = {type:'desktop'};
  }

  getChildContext() {
    return {type: this.state.type};
  }

  componentDidMount() {
    const checkMediaQuery = () => {
      const type = window.matchMedia("(min-width: 1025px)").matches ? 'desktop' : 'mobile';
      if (type !== this.state.type) {
        this.setState({type});
      }
    };

    window.addEventListener('resize', checkMediaQuery);
    checkMediaQuery();
  }

  render() {
    return this.props.children;
  }
}

MediaQuery.childContextTypes = {
  type: PropTypes.string
};
```

問題は、コンポーネントによって提供されたコンテキスト値が変更された場合、途中のコンポーネントがshouldComponentUpdateからfalseを返す場合、その値を使用する子孫は更新されない  
これは、コンテキストを使用してコンポーネントを完全に制御できないため、コンテキストを確実に更新する方法ではない
