# アカウント

## 複数アカウントの切り替え

SSH の場合
 - サブアカウント用のSSH鍵を生成し、GitHub に登録する
 - ~/.ssh/config を編集し、サブアカウント用の Host 情報を定義する
 - 以下、各リポジトリで
   - git@[サブアカウントの Host]~ 形式の URL で clone や remote add する
   - git config user.name/user.email でサブアカウントのユーザ名・メールアドレスを設定

```
Host github.com
  HostName github.com
  IdentityFile ~/.ssh/id_rsa
  User git

Host github.com.company # サブアカウント用
  HostName github.com
  IdentityFile ~/.ssh/id_rsa_company
  User git
```

clone する時に、config に指定した Host で clone する

```bash
$ git clone git@github.com.company:yasunari-fujieda/test.git
```

その後、clone したリポジトリでユーザ名・メールアドレスを設定

```bash
$ git config --local user.name "name"
$ git config --local user.email "email"
```
