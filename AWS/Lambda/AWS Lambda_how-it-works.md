# AWS Lambdaの仕組み

Lambda 関数を作成する際に、Lambda 関数に許可するメモリ容量や最大実行時間などの設定情報を指定し、Lambda 関数が呼び出されると、AWS Lambda は指定された構成設定に基づいてコンテナ (実行環境) を起動する  
コンテナの設定には時間がかかる  
ブートストラップが必要なため、Lambda 関数を呼び出すために若干の遅延が発生するが、通常、初めて呼び出した時、または更新された時にのみ発生する  
Lambda 関数を連続して呼び出す場合、AWS Lambda はできる限りコンテナを再利用する

Lambda 関数が実行されると、AWS Lambda は別の Lambda 関数呼び出しに備えて、コンテナを一定期間維持  
実際には、サービスは Lambda 関数の完了後コンテナをフリーズさせ、再び Lambda 関数が呼び出された際に AWS Lambda がコンテナを再利用する場合は、コンテナを解凍して再利用する  
コンテナを再利用するこのアプローチでは、次のような影響がある

- Lambda 関数内の宣言 (handler コード以外、「[プログラミングモデル](https://docs.aws.amazon.com/ja_jp/lambda/latest/dg/programming-model-v2.html)」を参照) は、関数が再度呼び出された際に追加で最適化されない限り、初期化状態を維持する  
たとえば、Lambda 関数がデータベース接続を確立する場合、連続した呼び出しでは接続を再確立する代わりに元の接続が使用される  
接続を作成する前にすでに接続が存在するかどうかを確認するロジックをコードに追加できる

- 各コンテナには、/tmp ディレクトリに多少のディスク領域がある  
ディレクトリのコンテンツは、コンテナが停止された際に維持され、複数の呼び出しに使用できる一時的なキャッシュを提供する  
キャッシュに保存したデータが存在するかどうかを確認するための追加コードを追加できる  
ディスク領域のサイズについては「[AWS Lambda の制限](https://docs.aws.amazon.com/ja_jp/lambda/latest/dg/limits.html)」を参照

- Lambda 関数で開始され、関数の終了時に完了しなかったバックグラウンド処理やコールバックは、AWS Lambda がコンテナを再利用する場合に再開される  
コードのバックグラウンド処理またはコールバック (Node.js の場合) はコード終了までに完了させる

注意  
Lambda 関数を記述する場合、AWS Lambda が常にコンテナを再利用することを前提にしない  
さまざまな要因によって、AWS Lambda が既存コンテナを再利用せずに、単純に新しいコンテナを作成する場合もある

## [Lambda 関数の同時実行](https://docs.aws.amazon.com/ja_jp/lambda/latest/dg/concurrent-executions.html)

ストリームベースのイベントソースとストリームベースではないイベントソースでは同時実行数が異なる

### 安全制限

デフォルトで、合計の同時実行数が100に制限されている  
上限を大きくするには「[同時実行数の制限の引き上げをリクエストするには](https://docs.aws.amazon.com/ja_jp/lambda/latest/dg/concurrent-executions.html#increase-concurrent-executions-limit)」の手順に従う

安全制限を超えるような場合は呼び出しがスロットルされ関数が実行されない  
呼び出しがスロットルされるごとに、その関数の CloudWatch Throttles メトリクスが増加  
スロットルされた呼び出しの処理は関数の呼び出し方法によって異なる

## [エラー時の再試行](https://docs.aws.amazon.com/ja_jp/lambda/latest/dg/retries-on-errors.html)

Lambda 関数は以下の理由で失敗する場合がある
- 関数がエンドポイントに到達する前にタイムアウト
- 関数が入力データを正しく解析できなかった
- 関数にメモリ不足エラーまたは他タイムアウトなどのリソース制約が発生した

このようなエラーが発生した場合、関数は例外をスローする  
例外の処理方法は Lambda 関数の呼び出し方法によって異なる

## [AWS Lambda アクセス権限モデル](https://docs.aws.amazon.com/ja_jp/lambda/latest/dg/intro-permission-model.html)

## [Lambda 実行環境と利用できるライブラリ](https://docs.aws.amazon.com/ja_jp/lambda/latest/dg/current-supported-versions.html)

AWS Lambda の実行環境は以下
- パブリックAmazon Linux AMIバージョン - amzn-ami-hvm-2016.03.3.x86_64-gp2
- Linuxカーネルバージョン - 4.4.35-33.55.amzn1.x86_64

コードにネイティブバイナリを使用している場合は、必ずこの環境でコンパイルする  
AWS Lambda は64ビットのバイナリのみサポート

AWS Lambda は次のランタイムバージョンをサポート
- Node.js – v0.10.36, v4.3.2 (推奨)
- Java – Java 8
- Python – Python 2.7
- .NET Core – .NET Core 1.0.1 (C#)

使用しているランタイムに限らず以下のライブラリが用意されているのでライブラリを含める必要はない
- AWS SDK - AWS SDK for JavaScript バージョン 2.7.10

以下のライブラリは AWS Lambda 実行環境にあるので含める必要はない
- AWS SDK for Python (Boto 3) バージョン 1.4.2、Botocore バージョン 1.4.81
- Java 用 java-1.8.0-openjdk の Amazon Linux ビルド

### [Lambda 関数で使用できる環境変数](https://docs.aws.amazon.com/ja_jp/lambda/latest/dg/current-supported-versions.html#lambda-environment-variables)