# Snapshot Testing

スナップショットテストはUIが意図せず変更されないようにするための非常に便利なツール

モバイルアプリケーションの典型的なスナップショットテストは、UIをレンダリングし、スナップショットを取得し、保存されていたイメージと比較する  
2つの画像が一致しない場合、テストは失敗する。変更が予期せぬものか、スクリーンショットを新しいバージョンのUIコンポーネントのものに更新する必要があるかのいずれか

## Snapshot Testing with Jest

グラフィカルなUIをレンダリングする代わりに、テストレンダラーを使ってReactツリーのシリアライズ可能な値を生成することができる

```javascript
import React from 'react';
import Link from './Link';
import renderer from 'react-test-renderer';

it('renders correctly', () => {
  const tree = renderer.create(
    <Link page='http://www.facebook.com'>Facebook</Link>
  ).toJSON();
  expect(tree).toMatchSnapshot();
});
```

このテストを初めて実行した時、Jestは以下のようなスナップショットファイルを作成する

```
// Jest Snapshot v1, https://goo.gl/fbAQLP

exports[`renders correctly 1`] = `
<a
  className="normal"
  href="http://www.facebook.com"
  onMouseEnter={[Function]}
  onMouseLeave={[Function]}
>
  Facebook
</a>
`;
```

スナップショット成果物はコードの変更とともにコミットされ、コードレビュープロセスの一環としてレビューされる  
Jestはコードレビュー中に人間が読めるように`pretty-format`を使用してスナップショットを作成する  
その後のテストで、Jestはレンダリングされた以前のスナップショットと単純に比較する  
もし、マッチすればテストをパスし、もしマッチしなければ、テストランナーが見つけたバグを修正するべきだし、もしくは、実装が変更されスナップショットを更新する必要がある

## Updating Snapshots

バグが混入した時、スナップショットが失敗することで直感的に気づくことができる
テストが失敗したら、問題を解決して、スナップショットが再び合格することを確認する

以下のようにすると、Linkコンポーネントが指しているアドレスを変更するとテストが失敗する

```javascript
it('renders correctly', () => {
  const tree = renderer.create(
    <Link page="http://www.instagram.com">Instagram</Link>
  ).toJSON();
  expect(tree).toMatchSnapshot();
});
```

上記のケースでは、Jestはこのように出力する

```
FAIL  .\Link.test.js                                               
 ● renders correctly                                               
                                                                   
   expect(value).toMatchSnapshot()                                 
                                                                   
   Received value does not match stored snapshot 1.                
                                                                   
   - Snapshot                                                      
   + Received                                                      
                                                                   
    <a                                                             
      className="normal"                                           
   -  href="http://www.facebook.com"                               
   +  href="http://www.instagram.com"                              
      onMouseEnter={[Function]}                                    
      onMouseLeave={[Function]}                                    
    >                                                              
   -  Facebook                                                     
   +  Instagram                                                    
    </a>                                                           
                                                                   
     at Object.<anonymous> (Link.test.js:10:16)                    
         at Promise (<anonymous>)                                  
     at Promise.resolve.then.el (node_modules/p-map/index.js:42:16)
         at <anonymous>                                            
     at process._tickCallback (internal/process/next_tick.js:169:7)
```

別のアドレスを指すようにコンポーネントを更新したので、このコンポーネントのスナップショットの変更を期待するのは妥当  
更新されたコンポーネントのスナップショットがこのテストケースのスナップショット成果物と一致しないため、スナップショットテストケースが失敗する

これを解決するには、スナップショット成果物を更新する必要がある  
スナップショットを再生成することを知らせるフラグを付けてJestを実行することができる

```
jest --updateSnapshot
```

上記コマンドを実行して、変更を許可する  
好みで単一文字`-u`を使用してスナップショットを再生成することもできる  
これにより、失敗したスナップショットの成果物が再生成される  
意図しないバグが原因でさらに失敗したスナップショットテストがあった場合、スナップショットを再生成する前にバグを修正して、バグが含まれるスナップショットを記録しないようにする必要がある

スナップショットを再生成するテストケースを制限したい場合は、`--testNamePattern`フラグを追加してパターンにマッチするテストのスナップショットのみを再記録することができる

## Tests Should Be Deterministic

テストは、コンポーネントが変更されていない限り同じテストを複数回実行しても毎回同じ結果を返すべきである  
生成されたスナップショットにプラットフォーム固有のデータやその他の非確定的なデータが含まれていないことを確認する必要がある

例えば、`Clock`という`Date.now()`を使っているコンポーネントがある場合、このコンポーネントから生成されるスナップショットは毎回違う結果になる  
こういうケースでは毎回同じ値を返す`Date.now()`のモックを作るべき

```javascript
Date.now = jest.fn(() => 1482363367071);
```

こうすることで、テストの実行時に関係なく、このコンポーネントに対して同じスナップショットが生成される