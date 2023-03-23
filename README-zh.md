# HeyGPT

這是一個簡單的命令行接口工具，允許您與來自OpenAI的GPT-3互動。

您可以使用它來：
- 與GPT-3聊天並獲得回覆
- 快速與GPT-3聊天並獲得回覆
- 將文本從另一個命令輸入到HeyGpt中並獲得回應


## 要求

- 當前，要使用HeyGpt，您需要從OpenAI獲取API密鑰。您可以在他們的網站上註冊。
- 您需要安裝Rust才能編譯和使用HeyGpt。


## 安裝

1. 複製存儲庫

`git clone https://github.com/DominikN/HeyGPT.git`

2. 運行執行檔

`cargo run`

或者，您可以使用`cargo build`構建可執行文件，然後將其移動到您的“ bin”文件夾中。


## 用法

- 要與GPT-3聊天，只需輸入您的消息並按Enter鍵。
- 要快速與GPT-3聊天，請使用快速聊天命令`cargo run“您的快速聊天消息”`
- 要從另一個命令傳遞文本，請使用管道`|`字符，然後是`cargo run`，如下所示：
        `echo“ hello”| cargo run`
- 要初始化您的HeyGpt配置，請使用“ init”命令：
        `cargo run init`

請注意，在初始化期間，您將被提示輸入您的OpenAI API密鑰，因此請確保您已準備好該密鑰。

您還可以修改位於主目錄中的`.heygpt-config`的配置文件。


## 許可證

該項目根據MIT許可證許可 - 請參閱[ LICENSE ](LICENSE)文件以了解詳細信息。