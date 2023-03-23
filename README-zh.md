# HeyGPT
這是一個簡單的命令行介面工具，讓您可以與OpenAI的ChatGPT進行互動。

您可以使用它來：
- 與ChatGPT聊天並獲得回應
- 快速聊天並獲得回應
- 將文本從另一個命令輸出到HeyGpt並獲得回應

## 要求
目前，要使用HeyGpt，您需要從OpenAI獲得API密鑰。您可以在其網站上註冊。

## 用法
請注意，在初始化期間，您將收到要求輸入OpenAI API密鑰的提示，因此請確保已經準備好。

您也可以修改位於您的主目錄中的`.heygpt-config`的配置文件。

- 要初始化您的HeyGpt配置，請使用`init`命令：
```bash
$ heygpt init
```

- 要與ChatGPT聊天，只需執行`heygpt`，然後輸入您的消息並按Enter鍵。
```bash 
$ heygpt
```
- 要快速聊天與HeyGpt，請使用快速聊天命令
```bash
$ heygpt "如何查找我的ID地址" # 回复將根据您的系統而异。
```

- 要從另一個命令輸出文本，請使用管道`|`字符後跟`heygpt`，如下所示：

```bash
$ vim --help | heygpt "翻譯成繁體中文" # 將cli工具說明消息翻譯
```
```bash
$ cat main.py | heygpt "為此腳本編寫README.md" >> README.md # 為某些腳本生成文檔
```

## 安裝
需要安裝Rust
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### 從源碼構建
1. 克隆存儲庫
2. 執行可執行文件 `cargo build --release`
3. 將 `target/release/heygpt` 複製到 `/usr/local/bin`

### macOS
即將推出

### Debian/Ubuntu
即將推出

### Windows
即將推出
