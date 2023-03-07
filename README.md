chatGPTをターミナルから利用する簡単なCLIツールです。

## 使い方

```shell
gptr "1+1は？"
```

利用できるフラグや説明は以下コマンドで表示できます。

```shell
gptr -h
```

複数行入力を行いたい場合は、説明を入力せずに実行し最後にCtrl+Dを押してください。
```shell
> gptr
1+1の
回答を教えて
Ctrl+dを入力して完了
```

```shell
## 設定

利用するのはopenAIのAPIキーが必要になります。[openAI](https://platform.openai.com/account/api-keys)から取得してください。  
翻訳機能を利用する場合は、別途deepLのAPIキーが必要になります。[deepL](https://www.deepl.com/pro#developer)から取得してください。  
gptr.jsonを作成し、以下のように設定してください。

```json
{
  "openai_api_key": "key from openai api",
  "translate": {
    "source": "ja",
    "target": "en"
  },
  "deepl_api_key": "key from deepl api"
}
```



## 質問の事前定義
chatGPTに質問を行う前に前提条件を指定したいことがあります。(例えば「英語に翻訳してください」、「Typescriptに関して質問です」等)  
その場合に定型文としてあらかじめファイルに指定しておき、オプションで簡単に呼び出すことができます。

事前定義を行うにはgptr_predefine.jsonを作成し、以下のような形で設定してください。
```json5
{
  "predefine": {
    // 事前定義の名前
    "ts": {
      // 事前定義の内容
      "content": "Typescriptに関する質問です。",
      // 翻訳するかどうか
      "translate":false
    },
    "en": {
      "content": "I want to translate to English.",
      "translate":false
    }
  }
}
```

事前定義を呼び出す場合は*-p*オプションの後に事前定義名を渡してください。  

**例.上記で設定したTypescriptの質問を行う場合**
```shell
gptr -p ts "「事前定義を行う」という関数名を3つ考えてください。"
```

### 設定ファイルの読み込み場所
以下の順序で設定ファイルの読み込みを行います。
1. ホームディレクトリ内
2. ホームディレクトリの中の.config/gptrディレクトリ内
3. ホームディレクトリの中の.configディレクトリ内
4. バイナリが配置されているディレクトリ内

