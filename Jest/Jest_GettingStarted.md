# Getting Started

## Install

```
npm install jest
```

まず、仮で2つの数字を足す関数のテストを行う  
`sum.js`を作成する

```javascript
function sum(a, b) {
  return a + b;
}

module.exports = sum;
```

次に実際のテストが含まれる`sum.test.js`を作成する

```javascript
const sum = require('/sum');

test('adds 1 + 2 to equal 3', () => {
  expect(sum(1, 2)).toBe(3);
});
```

`package.json`の`scripts`セクションに以下を追加

```json
{
  "scripts": {
    "test": "jest"
  }
}
```

最後に`npm test`を実行すると、Jestはこのようなメッセージを出力する

```
PASS  ./sum.test.js
✓ adds 1 + 2 to equal 3 (5ms)
```

このテストでは`expect`と`toBe`を使用して、2つの値が同じであることをテストした  
Jestでできるその他のことは[Using Matches](https://facebook.github.io/jest/docs/using-matchers.html)を参照

## Running from command line

様々な便利なオプションを付けてCLIから直接Jestを実行できる  
以下は、`config.json`という設定ファイルを使い、`my-test`という名前に一致するファイルに対してJestを実行し、その後OS固有の通知を表示する

```
jest my-test --notify --config=config.json
```

JestのCLIはオプションについては[Jest CLI Options](https://facebook.github.io/jest/docs/cli.html)を参照

