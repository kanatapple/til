# Conditional Rendering

条件付きレンダリングはJavaScriptと同じように動作する  
現在の状態を表す要素を作成するために、JavaScriptのif演算子やその他の条件演算子を使い、ReactがUIを更新するようにする

```javascript
function UserGreeting(props) {
  return <h1>Welcome back!</h1>;
}

function GuestGreeting(props) {
  return <h1>Please sign up.</h1>;
}
```

上記のコンポーネントがあり、以下のようなユーザがログインしてるかどうかで、`UserGreeting`、`GuestGreeting`のいずれかのコンポーネントを表示する`Greeting`コンポーネントを作成します

```javascript
function Greeting(props) {
  const isLoggedIn = props.isLoggedIn;
  if (isLoggedIn) {
    return <UserGreeting />;
  }
  else {
    return <GuestGreeting />;
  }
}

React.render(
  <Greeting isLoggedIn={false} />,
  document.getElementById('root')
);
```

この例では、isLoggedInの値によって異なる挨拶をする

## Element Variables

要素を格納するために変数を使用することができる  
これにより、コンポーネントの一部を条件付きでレンダリングし、残りの部分は変更されない

以下は「ログアウト」ボタンと「ログイン」ボタンを表すコンポーネントです

```javascript
function LoginButton(props) {
  return (
    <button onClick={props.onClick}>
      Login
    </button>
  );
}

function LogoutButton(props) {
  return (
    <button onClick={props.onClick}>
      Logout
    </button>
  );
}
```

`LoginControl`というステートフルなコンポーネントを作成してみる  
現在の状態に応じて、`<LoginButton />`または`<Logout />`のいずれかをレンダリングする

```javascript
class LoginControl extends React.Component {
  constructor(props) {
    super(props);
    this.handleLoginClick = this.handleLoginClick.bind(this);
    this.handleLogoutClick = this.handleLogoutClick.bind(this);
    this.state = {isLoggedIn: false};
  }

  handleLoginClick() {
    this.setState({isLoggedIn: true});
  }

  handleLogoutClick() {
    this.setState({isLoggedIn: false});
  }

  render() {
    const isLoggedIn = this.state.isLoggedIn;

    let button = null;
    if (isLoggedIn) {
      button = <LogoutButton onClick={this.handleLogoutClick} />;
    } else {
      button = <LoginButton onClick={this.handleLoginClick} />;
    }

    return (
      <div>
        <Greeting isLoggedIn={isLoggedIn} />
        {button}
      </div>
    );
  }
}

ReactDOM.render(
  <LoginControl />,
  document.getElementById('root')
);
```

上記のように、変数を宣言してif文を使用すると、条件付きでコンポーネントをレンダリングできるが  
短い構文を使用することもできる

## Inline If with Logical && Operator

JSXでは中括弧に式や`&&`オペレータを含めることができる

```javascript
function Mailbox(props) {
  const unreadMessages = props.unreadMessages;
  return (
    <div>
      <h1>Hello!</h1>
      {unreadMessages.length > 0 &&
        <h2>
          You have {unreadMessages.length} unread messages.
        </h2>
      }
    </div>
  );
}

const messages = ['React', 'Re: React', 'Re:Re: React'];
ReactDOM.render(
  <Mailbox unreadMessages={messages} />,
  document.getElementById('root')
);
```

JavaScriptでは、`true && expression`の場合、`expression`も評価され  
`false && expression`の場合、`expression`が評価されないために機能する  
そのため、条件がtrueの場合、`&&`の後の要素が出力され、falseの場合、Reactは無視してスキップする

## Inline If-Else with Conditional Operator

インラインで条件付きレンダリングをするもう1つの方法は、JavaScriptの3項演算子を使うこと

```javascript
render () {
  const isLoggedIn = this.state.isLoggedIn;
  return (
    <div>
      The user is <b>{isLoggedIn ? 'currently' : 'not'}</b> logged in.
    </div>
  );
}
```

以下のように大きな表現にも使用できる

```javascript
render (props) {
  const isLoggedIn = props.state.isLoggedIn;
  return (
    <div>
      {isLoggedIn ? (
        <LogoutButton onClick={this.handleLogoutClick} />
      ) : (
        <LoginButton onClick={this.handleLoginClick} />
      )
    </div>
  );
}
```

条件が複雑になりすぎる場合は、コンポーネントを分割した方がいい

## Preventing Component from Rendering

稀に他のコンポーネントにレンダリングされたが、自身で非表示にしたい場合がある  
その場合は、`null`を返せばいい

以下の例では、`<WarningBanner />`は`warn`という`props`の値に応じてレンダリングされる  
その値が`false`の場合、コンポーネントはレンダリングしない

```javascript
function WarningBanner(props) {
  if (!props.warn) {
    return null;
  }

  return (
    <div className="warning">
      Warning!
    </div>
  );
}

class Page extends React.Component {
  constructor(props) {
    super(props);
    this.state = {showWarning: true}
    this.handleToggleClick = this.handleToggleClick.bind(this);
  }

  handleToggleClick() {
    this.setState(prevState => ({
      showWarning: !prevState.showWarning
    }));
  }

  render() {
    return (
      <div>
        <WarningBanner warn={this.state.showWarning} />
        <button onClick={this.handleToggleClick}>
          {this.state.showWarning ? 'Hide' : 'Show'}
        </button>
      </div>
    );
  }
}

ReactDOM.render(
  <Page />,
  document.getElementById('root')
);
```

コンポーネントのレンダリングメソッドでnullを返しても、ライフサイクルメソッドの起動には影響しない  
`componentWillUpdate`、`componentDidUpdate`は引き続き呼び出される
