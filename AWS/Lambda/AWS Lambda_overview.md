# AWS Lambdaとは

AWS Lambda とはサーバをプロビジョニング(必要に応じてネットワークやコンピューターの設備などのリソースを提供できるよう予測し、準備しておくこと)したり管理しなくてもコードを実行できるコンピューティングサービス  
AWS Lambda は必要に応じてコードを実行し、1日あたり数個のリクエストから1秒あたり数千のリクエストまで自動的にスケーリングする  
使用したコンピューティング時間に対しての支払い  
AWS Lambda を使うのに必要な操作はサポートする言語を指定するだけ

## AWS Lambdaでできること
- Amazon S3、Amazon DynamoDBテーブル内のデータ変更などのイベントに応答してコードを実行
- Amazon API Gateway を使用して HTTP リクエストに応答してコードを実行
- AWS SDK を使用して作成された API コールを使用してコードを呼び出せる

## AWS Lambdaが適している用途

AWS Lambda は、AWS Lambda でサポートされる言語 (Node.js、Java、C# および Python) でアプリケーションコードを記述できる場合、多くのアプリケーションシナリオにとって最適なコンピューティングプラットフォームとなり、Lambda によって提供される AWS Lambda スタンダードランタイム環境とリソース内で実行される
