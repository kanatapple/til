# Testing React Apps

FacebookではReactApplicationのテストにJestを使用している

## Setup

### Setup with Create React App

Reactを使い始めたばかりの場合は、Jestを使う準備ができているので`Create React App`を使うことを勧める  
セットアップのための余計な手順を行う必要がなく、すぐに次のセクションに進める

### Setup without Create React App

すでにアプリケーションがある場合には、うまく機能するためにいくつかのパッケージをインストールする必要がある  
テスト環境のコードに変換するために`babel-jest`とReactのBabelプリセットを使う(babel-jestはjestをインストールしてあればOK)

### Snapshot Testing

省略  
[Jest_SnapshotTesting.md](./Jest_SnapshotTesting.md)を参照

### DOM Testing

アサーションしたり、レンダリングされたコンポーネントを操作したい場合は、`Enzyme`かReactの`TestUtils`を使うことができる

Enzymeを使う場合は`npm install enzyme`を実行する  
もし、Reactのバージョンが15.5.0未満の場合は、`react-addons-test-utils`も追加でインストールする

2つのラベルを入れ替えるチェックボックを実装してみる

```javascript
import React from 'react';

export default class CheckboxWithLabel extends React.Component {

  constructor(props) {
    super(props);
    this.state = {isChecked: false};

    // bind manually because React class components don't auto-bind
    // http://facebook.github.io/react/blog/2015/01/27/react-v0.13.0-beta-1.html#autobinding
    this.onChange = this.onChange.bind(this);
  }

  onChange() {
    this.setState({isChecked: !this.state.isChecked});
  }

  render() {
    return (
      <label>
        <input
          type="checkbox"
          checked={this.state.isChecked}
          onChange={this.onChange}
        />
        {this.state.isChecked ? this.props.labelOn : this.props.labelOff}
      </label>
    );
  }
}
```

この例ではEnzymeの`shallow`レンダーを使う

```javascript

import React from 'react';
import {shallow} from 'enzyme';
import CheckboxWithLabel from '../CheckboxWithLabel';

test('CheckboxWithLabel changes the text after click', () => {
  // Render a checkbox with label in the document
  const checkbox = shallow(
    <CheckboxWithLabel labelOn="On" labelOff="Off" />
  );

  expect(checkbox.text()).toEqual('Off');

  checkbox.find('input').simulate('change');

  expect(checkbox.text()).toEqual('On');
});
```

### Custom Transformers

より高度な機能が必要な場合は、独自のトランスフォーマーを構築できる  
例えば、`babel-jest`を使う代わりに`babel`を使用する

```javascript
// custom-transformer.js

'use strict';

const babel = require('babel-core');
const jestPreset = require('babel-preset-jest');

module.exports = {
  process(src, filename) {
    if (babel.util.canCompile(filename)) {
      return babel.transform(src, {
        filename,
        presets: [jestPreset],
        retainLines: true,
      }).code;
    }
    return src;
  },
};
```

これをJestで動作させるには、Jestの設定を`"transform": {"\\.js$": "path/to/custom-transformer.js"}`に変更する必要がある  
