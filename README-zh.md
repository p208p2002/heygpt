# HeyGPT
這是一個簡單的命令行界面工具，可以讓您與 OpenAI 的 ChatGPT 進行交互。

您可以使用它來：
- 與 ChatGPT 聊天並獲得回覆。
- 快速與 ChatGPT 聊天並獲得回覆。
- 將文本從其他命令傳遞到 HeyGPT，並獲得回應。

## 前提條件
目前，要使用 HeyGpt，您需要從 OpenAI 獲得 API 金鑰。您可以在他們的網站上註冊。

## 安裝
### Debian / Ubuntu / macOS
```bash
$ curl https://raw.githubusercontent.com/p208p2002/heygpt/main/setup.sh | bash
```

## 用法
請注意，在初始化期間，您將被提示輸入您的 OpenAI API 金鑰，所以請確保您已經準備好了。

您還可以修改位於您的主目錄`.heygpt-config`的配置文件。

- 若要初始化HeyGpt配置，請使用`init`命令：
```bash
$ heygpt init
```

- 與 ChatGPT聊天，只需執行 `heygpt` 然後輸入您的消息並按 Enter。
```bash 
$ heygpt
```
- 快速與 HeyGpt聊天，使用快速聊天命令
```bash
$ heygpt "how to find my ip address" # 回答將根據您的系統而有所不同。
```

- 要從另一個命令中傳遞文本，請使用管道符“ | ”，然後是“ heygpt”，如下所示：

```bash
$ vim --help | heygpt "translate to traditional chinese" # cli-tool help message translate
```
```bash
$ cat main.py | heygpt "wirte README.md for this script" >> README.md # generate document for some script
```

## 從源代碼構建
安裝Rust是必需的。如果您沒有安裝Rust，您可以使用以下命令安裝它：
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

1. 克隆此存儲庫

2. 進入 `heygpt` 然後運行命令 `cargo build --release`
    > 對於Ubuntu還需要安裝：`build-essential`、`pkg-config`、`openssl`、`libssl-dev`、`curl`、`git`

3. 將 `target/release/heygpt` 複製到您的 `PATH` 包含的可執行文件目錄中。
