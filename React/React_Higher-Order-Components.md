# Higher-Order Components

`Higher-Order Components(HOC)`はコンポーネントロジックを再利用するための高度な技術  
HOCはそれ自体、React APIの一部ではない

具体的にはHOCはコンポーネントを受け取って新しいコンポーネントを返す関数

```javascript
const EnhancedComponent = higherOrderComponent(WrappedComponent);
```

コンポーネントは`props`をUIに変換するのに対して、HOCはコンポーネントを別のコンポーネントに変換する  
HOCはReduxの`connect`、Relayの`createContainer`などのReactのサードパーティーライブラリでよく使われている

## Use HOCs For Cross-Cutting Concerns

コンポーネントはReactでコード再利用の主要な単位  
しかし、いくつかのパターンはコンポーネントにあまり適していないことがわかる

例えば、以下のようなコード(外部データを購読してコメントをリスト表示するコンポーネント)

```javascript
class CommentList extends React.Component {
  constructor() {
    super();
    this.handleChange = this.handleChange.bind(this);
    this.state = {
      // "DataSource" is some global data source
      comments: DataSource.getComments()
    };
  }

  componentDidMount() {
    // Subscribe to changes
    DataSource.addChangeListener(this.handleChange);
  }

  componentWillUnmount() {
    // Clean up listener
    DataSource.removeChangeListener(this.handleChange);
  }

  handleChange() {
    // Update component state whenever the data source changes
    this.setState({
      comments: DataSource.getComments()
    });
  }

  render() {
    return (
      <div>
        {this.state.comments.map((comment) => (
          <Comment comment={comment} key={comment.id} />
        ))}
      </div>
    );
  }
}
```

次に、ブログを購読するコンポーネント。このコンポーネントは同様のパターンに従っている

```javascript
class BlogPost extends React.Component {
  constructor(props) {
    super(props);
    this.handleChange = this.handleChange.bind(this);
    this.state = {
      blogPost: DataSource.getBlogPost(props.id)
    };
  }

  componentDidMount() {
    DataSource.addChangeListener(this.handleChange);
  }

  componentWillUnmount() {
    DataSource.removeChangeListener(this.handleChange);
  }

  handleChange() {
    this.setState({
      blogPost: DataSource.getBlogPost(this.props.id)
    });
  }

  render() {
    return <TextBlock text={this.state.blogPost} />;
  }
}
```

`CommentList`と`BlogPost`は同一ではない(DataSourceの異なるメソッドを呼び出し、異なる出力をレンダリングしている)  
しかし、それらの実装の多くは同じ
- マウント時に`DataSource`の変更イベントリスナーを追加
- リスナーの内部で、`DataSource`が変更されるたびに`setState`を呼び出す
- アンマウント時に`DataSource`の変更イベントリスナーを削除

大規模なアプリケーションでは、`DataSource`を購読して`setState`を呼び出すという同じパターンが何度も繰り返されることが想像できる  
このロジックを単一の場所に定義し、それらを多くのコンポーネントにわたって共有できるようにする抽象化が必要  
これを実現するのがHOC

`CommentList`は`BlogPost`のような`DataSource`を購読するコンポーネントを作成する関数を書くことができる  
この関数は、購読されたデータを`props`として受け取る子コンポーネントを引数の1つとして受け取る

```javascript
const CommentListWithSubscription = withSubscription(
  CommentList,
  (DataSource) => DataSource.getComments()
);

const BlogPostWithSubscription = withSubscription(
  BlogPost,
  (DataSource, props) => DataSource.getBlogPost(props.id)
);
```

第1引数はラップされたコンポーネント  
第2引数は与えられた`DataSource`と現在の`props`のデータを受け取る(???)

`CommentListWithSubscription`と`BlogPostWithSubscription`がレンダリングされると、`CommentList`と`BlogPost`には`DataSource`から取得された最新のデータが`data`として渡される

```javascript
// This function takes a component...
function withSubscription(WrappedComponent, selectData) {
  // ...and returns another component...
  return class extends React.Component {
    constructor(props) {
      super(props);
      this.handleChange = this.handleChange.bind(this);
      this.state = {
        data: selectData(DataSource, props)
      };
    }

    componentDidMount() {
      // ... that takes care of the subscription...
      DataSource.addChangeListener(this.handleChange);
    }

    componentWillUnmount() {
      DataSource.removeChangeListener(this.handleChange);
    }

    handleChange() {
      this.setState({
        data: selectData(DataSource, this.props)
      });
    }

    render() {
      // ... and renders the wrapped component with the fresh data!
      // Notice that we pass through any additional props
      return <WrappedComponent data={this.state.data} {...this.props} />;
    }
  };
}
```

HOCは渡されたコンポーネントを変更したり、継承を使用してその動作をコピーしない  
HOCはコンテナーコンポーネントでラッピングされたオリジナルのコンポーネントを作成する  
HOCは副作用のないpure関数

ラップされたコンポーネントはコンテナの全ての`props`とその出力のレンダリングに使われる新しい`props`を受け取る  
HOCはデータがどのようにどんな感じで使われるかを気にしないし、ラップされたコンポーネントはどこからデータが来たかを気にしない

`withSubscription`関数は通常の関数なので、好きなだけ多くの引数を追加できる  
例えば、ラップされたコンポーネントからHOCを分離するために、変更可能な`data`という名前にすることができる  
あるいは、`shouldComponentUpdate`を構成する引数、データソースを構成する引数を受け取ることもできる  
これら全て、コンポーネントがどのように定義されているかをHOCが完全にコントールができるため可能

コンポーネントと同様に、`withSubscription`とラップされたコンポーネントとの関係は、完全に`props`ベース  
これは、ラップされたコンポーネントに同じ`props`を提供している限り、HOCを別のものに交換するもの容易  
例えば、データフェッチライブラリを変更する場合に便利

## Don't Mutate the Original Component. Use Composition.

HOCの中ではコンポーネントのプロトタイプを変更してはいけない

以下のコードはいくつか問題がある

```javascript
function logProps(InputComponent) {
  InputComponent.prototype.componentWillReceiveProps(nextProps) {
    console.log('Current props: ', this.props);
    console.log('Next props: ', nextProps);
  }
  // The fact that we're returning the original input is a hint that it has
  // been mutated.
  return InputComponent;
}

// EnhancedComponent will log whenever props are received
const EnhancedComponent = logProps(InputComponent);
```

1つは、入力コンポーネントが拡張コンポーネントから分離して再利用できない  
さらに、`componentWillReceiveProps`が変更された拡張コンポーネントを別のHOCに適用すると、最初のHOCの機能がオーバーライドされる  
このHOCはライフサイクルメソッドを持たないfunctionalコンポーネントでも機能しない

Mutating HOCs are a leaky abstraction—the consumer must know how they are implemented in order to avoid conflicts with other HOCs.

HOCは変更ではなく、入力コンポーネントをコンテナコンポーネントでラッピングすることで合成すべき

```javascript
function logProps(WrappedComponent) {
  return class extends React.Component {
    componentWillReceiveProps(nextProps) {
      console.log('Current props: ', this.props);
      console.log('Next props: ', nextProps);
    }
    render() {
      // Wraps the input component in a container, without mutating it. Good!
      return <WrappedComponent {...this.props} />;
    }
  }
}
```

このHOCは衝突の可能性を回避しつつ、前述のバージョン(変更してるHOC)と同じ機能を持っている  
これはクラスおよびfunctionalコンポーネントでも同様に機能する

## Convention: Pass Unrelated Props Through to the Wrapped Component

HOCはコンポーネントに機能を追加するが、大きく変えるべきではない  
HOCから返されるコンポーネントはラップされたコンポーネントと同様のインターフェースを持つことが期待される

HOCは特定のconcernとは無関係な`props`を渡すべき  
大抵のHOCは以下のような`render`メソッドを含んでいる

```javascript
render() {
  // Filter out extra props that are specific to this HOC and shouldn't be
  // passed through
  const { extraProp, ...passThroughProps } = this.props;

  // Inject props into the wrapped component. These are usually state values or
  // instance methods.
  const injectedProp = someStateOrInstanceMethod;

  // Pass props to wrapped component
  return (
    <WrappedComponent
      injectedProp={injectedProp}
      {...passThroughProps}
    />
  );
}
```

この制約は、HOCができるだけ柔軟かつ再利用可能であることを保証するのに役立つ

## Convention: Maximizing Composability

全てのHOCが同じように見えるわけではない  
ラップされたコンポーネントを1つだけ引数として受け取ることもある

```javascript
const NavbarWithRouter = withRouter(Navbar);
```

通常、HOCは追加の引数を受け付ける  
Relayのこの例では、`config`オブジェクトを使用してコンポーネントのデータ依存関係を指定している

```javascript
const CommentWithRelay = Relay.createContainer(Comment, config);
```

HOCの最も一般的なシグネチャは以下のような感じ

```javascript
const ConnectedComment = connect(commentSelector, commentActions)(Comment);
```

分割すると何が起きているかわかりやすい

```javascript
// connect is a function that returns another function
const enhance = connect(commentListSelector, commentListActions);
// The returned function is an HOC, which returns a component that is connected
// to the Redux store
const ConnectedComment = enhance(CommentList);
```

つまり、`connect`はHOCを返す関数

この形式は混乱したり不必要に思えるかもしれないが、有用な特性を持っている  
`connect`関数によって返された引数を受け取るHOCは、`Component => Component`というシグネチャを持つ  
出力タイプが入力タイプと同じ関数は一緒に組み立てるのが容易

```javascript
// Instead of doing this...
const EnhancedComponent = connect(commentSelector)(withRouter(WrappedComponent))

// ... you can use a function composition utility
// compose(f, g, h) is the same as (...args) => f(g(h(...args)))
const enhance = compose(
  // These are both single-argument HOCs
  connect(commentSelector),
  withRouter
)
const EnhancedComponent = enhance(WrappedComponent)
```

## Convention: Wrap the Display Name for Easy Debugging

HOCによって作成されたコンテナコンポーネントは他のコンポーネントと同様にReact Developer Toolsに表示される  
デバッグを容易にするために、HOCの結果であることが伝わるような表示名を選ぶ

最も一般的な方法は、ラップされたコンポーネントの表示名をラップすること
例えば、HOCの名前が`withSubscription`で、ラップされたコンポーネントの名前が`CommentList`の場合、`WithSubscription(CommentList)`という名前を付ける

```javascript
function withSubscription(WrappedComponent) {
  class WithSubscription extends React.Component {/* ... */}
  WithSubscription.displayName = `WithSubscription(${getDisplayName(WrappedComponent)})`;
  return WithSubscription;
}

function getDisplayName(WrappedComponent) {
  return WrappedComponent.displayName || WrappedComponent.name || 'Component';
}
```

## Caveats

HOCにはいくつか注意点がある

### Don't Use HOCs Inside the render Method

Reactのdiffingアルゴリズム(reconciliationと呼ばれる)は、コンポーネントの一意性を使用して、既存のサブツリーを更新するか、それを破棄して新しいものをマウントするか決定する  
もし`render`メソッドから返されたコンポーネントが前回のレンダリングのコンポーネントと同じ場合、Reactはサブツリーを新しいものと比較して再帰的に更新する  
それらが等しくない場合、前回のサブツリーは完全にアンマウントされる

通常、これについて考える必要はない  
しかし、HOCにとっては重要で、コンポーネントの`render`メソッド内でコンポーネントにHOCを適用してはいけない

```javascript
render() {
  // A new version of EnhancedComponent is created on every render
  // EnhancedComponent1 !== EnhancedComponent2
  const EnhancedComponent = enhance(MyComponent);
  // That causes the entire subtree to unmount/remount each time!
  return <EnhancedComponent />;
}
```

ここでの問題はパフォーマンスだけでなく、コンポーネントを再マウントすることで、そのコンポーネントと子コンポーネントの全ての状態が失われる

代わりに、ラップされたコンポーネントが1回だけ作られるように、コンポーネント定義の外でHOCを適用する  
そうすることで、レンダリング全体で一意性が保たれる

HOCを動的に適用する必要があるまれなケースでは、コンポーネントのライフサイクルメソッドまたはそのコンストラクタ内でHOCを実行することもできる

### Static Methods Must Be Copied Over

Reactコンポーネントに静的メソッドを定義すると便利なことがあります  
例えば、Relayコンテナは静的メソッド`getFragment`を公開して、GraphQLフラグメントの構成を容易にする

コンポーネントにHOCを適用すると、元のコンポーネントはコンテナコンポーネントにラップされるが、静的メソッドはコピーされない

```javascript
// Define a static method
WrappedComponent.staticMethod = function() {/*...*/}
// Now apply an HOC
const EnhancedComponent = enhance(WrappedComponent);

// The enhanced component has no static method
typeof EnhancedComponent.staticMethod === 'undefined' // true
```

これを解決するために、返却する前にメソッドをコンテナにコピーする

```javascript
function enhance(WrappedComponent) {
  class Enhance extends React.Component {/*...*/}
  // Must know exactly which method(s) to copy :(
  Enhance.staticMethod = WrappedComponent.staticMethod;
  return Enhance;
}
```

ただし、コピーする必要のあるメソッドを正確に把握する必要がある  
`hoist-non-react-statics`を使用すると非React静的メソッドを自動的にコピーできる

```javascript
import hoistNonReactStatic from 'hoist-non-react-statics';
function enhance(WrappedComponent) {
  class Enhance extends React.Component {/*...*/}
  hoistNonReactStatic(Enhance, WrappedComponent);
  return Enhance;
}
```

### Refs Aren't Passed Through

HOCの制約は、全ての`props`をラップされたコンポーネントに渡すことだが、参照は渡されない  
これは、参照はキーのような`props`と違い、Reactによって特別に処理されるから
HOCにラップされたコンポーネントの要素に`ref`を追加すると、`ref`はラップされたコンポーネントではなく、最も外側のコンポーネントのインスタンスを参照する

このような問題に直面した場合、理想的な解決策は`ref`の使用を避けること  
Reactを使って間もないユーザーは`props`がうまく動作している状況では`ref`に頼っている

```javascript
function Field({ inputRef, ...rest }) {
  return <input ref={inputRef} {...rest} />;
}

// Wrap Field in a higher-order component
const EnhancedField = enhance(Field);

// Inside a class component's render method...
<EnhancedField
  inputRef={(inputEl) => {
    // This callback gets passed through as a regular prop
    this.inputEl = inputEl
  }}
/>

// Now you can call imperative methods
this.inputEl.focus();
```