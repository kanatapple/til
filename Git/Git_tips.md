# Tips

## 特定のブランチを指定してclone

```
$ git clone -b ブランチ名 リポジトリのURL
```

## コミット履歴を綺麗にする

```sh
git rebase -i HEAD~3
```
`i` は `interactive` の頭文字。インタラクティブに行うという意味

--autosquash オプションいるかも？
 
HEAD~3` で現在のブランチの先頭から3つのコミットを抽出する

上記コマンドを叩くとエディタが起動するので編集する

pick: 変更なし
squash: 1つ前のコミットに統合し、メッセージは残す
fixup: 1つ前のコミットに統合し、メッセージは破棄

編集が終わったら push する
```sh
git push -f origin branch
```

## ブランチの削除

### ローカル

```sh
git branch -D foo
```

### リモート

```sh
git push --delete origin foo
```

## fixup
```sh
git commit --fixup={コミット番号}
```

## ブランチの名前を変更
```sh
git branch -m <古いブランチ名> <新しいブランチ名>
```

今いるブランチをリネームする場合は
```sh
git branch -m <新しいブランチ名>
```

## 過去のcommitのAuthorとメールアドレスを変更する

```sh
git filter-branch -f --env-filter \
  "GIT_AUTHOR_NAME='<new name>'; \
   GIT_AUTHOR_EMAIL='<new email>'; \
   GIT_COMMITTER_NAME='<new name>'; \
   GIT_COMMITTER_EMAIL='<new email>';" \
  <対象コミットリスト>
```

## reset

reset前のコミットはORIG_HEADという名前で参照できる  
間違えてresetした場合、ORIG_HEADにresetすると元の状態に戻せる

```sh
git reset --hard ORIG_HEAD
```

