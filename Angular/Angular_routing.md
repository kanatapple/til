# ルーティング

## ngOnInit が走る条件？

パスの構造が変わった時に ngOnInit が実行される

http://example.com/hoge  
 ↓  
http://example.com/hoge/fuga

こういう時

http://example.com/hoge  
 ↓  
http://example.com/hoge?fuga=1

こういう時は実行されない

