# HeyGPT
This is a simple command-line interface tool that allows you to interact with ChatGPT from OpenAI.

You can use it to:
- Chat with ChatGPT and get responses
- Quick chat with ChatGPT and get responses
- Pipe text from another command into HeyGpt and get responses

## Requirements
Currently, to use HeyGpt you need an API key from OpenAI. You can sign up for that on their website.

## Usage
Please note that you will be prompted to enter your OpenAI API key during initialization, so make sure you have that ready.

You can also modify your configuration file located at `.heygpt-config` in your home directory.

- To initialize your HeyGpt configuration, use the `init` command:
```bash
$ heygpt init
```

- To chat with ChatGPT, simply exec `heygpt` then type your message and press Enter.
```bash 
$ heygpt
```
- To quick chat with HeyGpt, use the quick chat command 
```bash
$ heygpt "how to find my id address" # The response will vary based on your system.
```

- To pipe text from another command, use the pipe `|` character followed by `heygpt`, like so:

```bash
$ vim --help | heygpt "translate to traditional chinese" # cli-tool help message translate
```
```bash
$ cat main.py | heygpt "wirte README.md for this script" >> README.md # generate document for some script
```

## Installation
Installing Rust is necessary
```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Build From Src
1. Clone the repository
2. Run the executable `cargo build --release`
3. copy `target/release/heygpt` to `/usr/local/bin`

### macOS
Comming soon

### Debian/Ubuntu
Comming soon

### Windows
Comming soon
