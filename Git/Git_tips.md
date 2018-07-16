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
