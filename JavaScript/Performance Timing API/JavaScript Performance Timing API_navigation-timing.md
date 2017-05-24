# Navigation Timing

Navigation Timing APIはWeb Performance APIの出発点  
最初は`window.performance.navigation`でPerformanceNavigationTimingのインスタンスを取得していたが  
これは`performance.getEntries`に統一しようというゴールと合わなかった

Navigation Timing API Level2ではNavigation Timingの理想的な設計によってこの歴史的なバグを修正しようと試みている  
それはPerformance Timeline APIに参加し、PerformanceResourceTimingインターフェースから`iniatorType`と`workStart`を継承することである

## PerformanceNavigationTiming属性

以下は[NAVIGATION-TIMING-2](https://w3c.github.io/perf-timing-primer/#bib-NAVIGATION-TIMING-2)に定義されているページナビゲーションの重要なパフォーマンス特性のリスト
![Navigation Timing Attributes](navigation-timing-attributes.png)

[ブラウザ対応](http://caniuse.com/#feat=nav-timing)
