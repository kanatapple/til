# React Native

## Build native mobile apps using JavaScript and React

`React Native`はJavaScriptのみを使用してモバイルアプリを構築することができる  
Reactと同じデザインが使用でき、コンポーネントを宣言しリッチなモバイルUIを作成できる

```javascript
import React, { Component } from 'react';
import { Text, View } from 'react-native';

class WhyReactNativeIsSoGreat extends Component {
  render() {
    return (
      <View>
        <Text>
          If you like React on the web, you'll like React Native.
        </Text>
        <Text>
          You just use native components like 'View' and 'Text',
          instead of web components like 'div' and 'span'.
        </Text>
      </View>
    );
  }
}
```

## A React Native app is a real mobile app

React Nativeでは"モバイルWebアプリ"、"HTML5アプリ"、"ハイブリッドアプリ"をビルドしない  
Objective-CまたはJavaで作成されたアプリと区別がつかない本物のモバイルアプリをビルドする  
React Nativeは通常のiOSアプリ、Androidアプリと同じ基本的なUIブロックを使用する  
JavaScriptとReactを使ってこれらのUIブロックを置くだけである

```javascript
import React, { Component } from 'react';
import { Image, ScrollView, Text } from 'react-native';

class AwkwardScrollingImageWithText extends Component {
  render() {
    return (
      <ScrollView>
        <Image
          source={{uri: 'https://i.chzbgr.com/full/7345954048/h7E2C65F9/'}}
          style={{width: 320, height:180}}
        />
        <Text>
          On iOS, a React Native ScrollView uses a native UIScrollView.
          On Android, it uses a native ScrollView.

          On iOS, a React Native Image uses a native UIImageView.
          On Android, it uses a native ImageView.

          React Native wraps the fundamental native components, giving you
          the performance of a native app, plus the clean design of React.
        </Text>
      </ScrollView>
    );
  }
}
```

## Don't waste time recompiling

React Nativeは高速にビルドする  
再コンパイルする代わりに即座にリロードする  
`Hot Reloading`を使用すると、アプリの状態を維持しながら新しいコードを実行することもできる

## Use native code when you need to

React NativeはObjective-C、Java、Swiftで書かれたコンポーネントとスムーズに結合する  
アプリケーションを最適化する必要がある時は、ネイティブコードに簡単にドロップできる  
React Nativeでアプリの一部を構築したり、ネイティブコードを直接使用してアプリの一部を構築することも簡単

```javascript
import React, { Component } from 'react';
import { Text, View } from 'react-native';
import { TheGreatestComponentInTheWorld } from './your-native-code';

class SomethingFast extends Component {
  render() {
    return (
      <View>
        <TheGreatestComponentInTheWorld />
        <Text>
          TheGreatestComponentInTheWorld could use native Objective-C,
          Java, or Swift - the product development process is the same.
        </Text>
      </View>
    );
  }
}
```
