# action

使い方

```
action(fn)
action(name, fn)
@action classMethod() {}
@action(name) classMethod () {}
@action boundClassMethod = (args) => { body }
@action(name) boundClassMethod = (args) => { body }
@action.bound classMethod() {}
@action.bound(function() {})
```

アプリケーションはアクションを持っていて、アクションは状態を変更するもの  
Mobxでは、マークを付けることで、アクションがコード上のどこに存在できるかを明示的にすることができる  
アクションはコードの構造を改善するのに役立つ  
関数を受け取り、`untracked`、`transaction`、`allowStateChanges`でラップした後に関数を返す  
オブザーバブルな値を変更したり、副作用がある機能であれば、アクションを使用することをオススメする  
`action`はdevtoolsと組み合わせてデバッグに有用な情報も提供する  
`setter`に`@action`デコレータを付けることはサポートしていないが、計算されたプロパティの`setter`は自動的にアクションになる

※strictモードが有効な場合、`action`を使用することは必須

```javascript
class ContactList {
  @observable pendingRequestCount = 0;
  
  @action createRandomContact() {
    this.pendingRequestCount++;
    superagent
        .get('https://randomuser.me/api/')
        .set('Accept', 'application/json')
        .end(action("createRandomContact-callback", (error, results) => {
            if (error)
                console.error(error);
            else {
                const data = JSON.parse(results.text).results[0];
                const contact = new Contact(this, data.dob, data.name, data.login.username, data.picture)
                contact.addTag('random-user');
                this.contacts.push(contact);
                this.pendingRequestCount--;
            }
        }));
  }
}
```

## async actions and runInAction

actionは現在実行中の関数にのみ影響し、現在の関数でスケジュールされている（呼び出されていない）関数には影響しない  
これは、`setTimeout`、Promiseの`.then`、`async`、コールバックの中などで状態を変える場合、これらのコールバックも同様にラップする必要があることを意味する  
上記の`createRandomContact-callback`アクションでも実証されている

async/awaitを使用する場合、`action`で非同期関数をラップするだけでは不十分で、このような状況では、`runInAction`で状態を更新する予定の場所をラップする必要がある  
この`runInAction`ブロックを`await`で待つ必要はない

```javascript
@action /*optional*/ updateDocument = async () => {
    const data = await fetchDataFromUrl();
    /* required in strict mode to be allowed to update state: */
    runInAction("update state after fetching data", () => {
        this.data.replace(data);
        this.isSaving = true;
    })
}
```

`runInAction`の使用方法は`runInAction(name?, fn, scope?)`

もし、Babelを使用している場合、[mobx-deep-action](https://github.com/mobxjs/babel-plugin-mobx-deep-action)が役立つ

## Bound actions

`action`デコレータ/関数はJavaScriptの通常のバインディングルールに従う  
MobX 3では`action.bound`が導入され、アクションにターゲットオブジェクトが自動的にバインドされる  
`action`関数とは違って`@action.bound`はnameパラメータを受け取らないので、名前は常にアクションがバインドされているプロパティ名に基づく

```javascript
class Ticker {
    @observable this.tick = 0

    @action.bound
    increment() {
        this.tick++ // 'this' will always be correct
    }
}

const ticker = new Ticker()
setInterval(ticker.increment, 1000)
```

もしくは

```javascript
const ticker = observable({
    tick: 1,
    increment: action.bound(function() {
        this.tick++ // bound 'this'
    })
})

setInterval(ticker.increment, 1000)
```

※`action.bound`関数を使用する場合は、アロー関数を使用しない  
アロー関数はすでにバインディングされているので