[![Build Status](https://travis-ci.org/safex/tropix.png?branch=master)](https://travis-ci.org/safex/tropix)

# tropix
This is ALPHA level software, use at your own risk; The Buy and Sell methods and Balance methods are trivial.
The trade automator however are not; No one except yourself are responsible for how you use it. This software is in an early testing phase, use with caution.

### For questions or Discussion about the software visit the forum thread:
### https://safe.exchange/t/tropix-and-poloniex/248

#install

## Build dependencies

Tropix is fully compatible with Stable Rust.

We recommend installing Rust through [rustup](https://www.rustup.rs/). If you don't already have rustup, you can install it like this:

- Linux:
	```bash
	$ curl https://sh.rustup.rs -sSf | sh
	```

- OSX:
	```bash
	$ curl https://sh.rustup.rs -sSf | sh
	```

	`clang` is required. It comes with Xcode command line tools or can be installed with homebrew.

- Windows

    Make sure you have Visual Studio 2015 with C++ support installed. Next, download and run the rustup installer from
	https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe, start "VS2015 x64 Native Tools Command Prompt", and use the following command to install and set up the msvc toolchain:
    ```
	$ rustup default stable-x86_64-pc-windows-msvc
    ```

Once you have rustup, download and build from source

- Docker

    Build the image: `docker build -t tropix .`. Then open docker and build from source:

```
docker run --rm -it -v $(pwd):/rust/tropix tropix /bin/bash
cd tropix
cargo build --release
```


----


## Build from source

```bash
# download Tropix code
$ git clone https://github.com/safex/tropix
$ cd tropix

# build in release mode
$ cargo build --release
```

###Once you are inside tropix folder you can use the automation robots:

#### Poloniex Check Buys vs Sells Trade History Analysis (Public API)

```bash
cargo run --bin polobuysell
```

Use this program to see what is the buy sell ratio of a given pair since a given time

```bash
# overview of usage
1. Choose a pair by entering its corresponding index
2. Enter how many hours ago you want to analyze from
3. See the results!
```

#### Poloniex Auto Buy Robot Under Price

```bash
cargo run --bin polobuyprice
```

This robot will buy up to a certain amount after the elapse of an interval of time.

```bash
# overview of usage
1. Enter Poloniex API key
2. Enter Poloniex Secret key
3. Enter the index of the Pair you want to trade
4. Enter a number of coins you want to buy
5. Enter how often you want to buy #4 options number of coins
6. give a 1 for margin trading.. give a 0 for non margin trading
7. give a price which is the upper limit so robot buys only below this price
```

#### Poloniex Auto Buy Robot

```bash
cargo run --bin poloautobuy
```

This robot will buy up to a certain amount after the elapse of an interval of time.

```bash
# overview of usage
1. Enter Poloniex API key
2. Enter Poloniex Secret key
3. Enter the index of the Pair you want to trade
4. Enter a number of coins you want to buy
5. Enter how often you want to buy #4 options number of coins
6. give a 1 for margin trading.. give a 0 for non margin trading
```

#### Poloniex Auto Sell Robot

```bash
cargo run --bin poloautosell
```

This robot will sell up to a certain amount after the elapse of an interval of time.

```bash
# overview of usage
1. Enter Poloniex API key
2. Enter Poloniex Secret key
3. Enter the index of the Pair you want to trade
4. Enter a number of coins you want to sell
5. Enter how often you want to sell #4 options number of coins
6. give a 1 for margin trading.. give a 0 for non margin trading
```

#### Bittrex Auto Buy Robot

```bash
cargo run --bin bittrexautobuy
```
This robot will buy up to a certain amount after the elapse of an interval of time.

```bash
# overview of usage
1. Enter Bittrex API key
2. Enter Bittrex Secret key
3. Enter the index of the Pair you want to trade
4. Enter a number of coins you want to buy
5. Enter how often every number of seconds you want to buy from option 4s number of coins
```

#### Bittrex Auto Sell Robot

```bash
cargo run --bin bittrexautosell
```
This robot will sell up to a certain amount after the elapse of an interval of time.

```bash
# overview of usage
1. Enter Bittrex API key
2. Enter Bittrex Secret key
3. Enter the index of the Pair you want to trade
4. Enter a number of coins you want to sell
5. Enter how often every number of seconds you want to sell from option 4s number of coins
```

#### Bittrex Auto Buy Robot Under Price

```bash
cargo run --bin bittrexbuyprice
```

This robot will buy up to a certain amount after the elapse of an interval of time.

```bash
# overview of usage
1. Enter Bittrex API key
2. Enter Bittrex Secret key
3. Enter the index of the Pair you want to trade
4. Enter a number of coins you want to buy
5. Enter how often you want to buy #4 options number of coins
6. give a price which is the upper limit so robot buys only below this price
```


#### Bittrex CLI trading module

```bash
cargo run --bin bittrexcli
```