[![Build Status](https://travis-ci.org/safex/tropix.png?branch=master)](https://travis-ci.org/safex/tropix)

# tropix
This is ALPHA level software, use at your own risk; The Buy and Sell methods and Balance methods are trivial.
The trade automator however are not; No one except yourself are responsible for how you use it. This software is in an early testing phase, use with caution.

#install
### Building from source

##### Ubuntu 14.04, 15.04, 15.10

```bash

# install rust stable
curl -sf https://raw.githubusercontent.com/brson/multirust/master/blastoff.sh | sh

# install stable and make it default
sudo multirust update stable
sudo multirust default stable
```

##### OSX with Homebrew

```bash
# install multirust
brew update
brew install multirust

# install stable and make it default
multirust update stable && multirust default stable
```

#### Start using Tropix with git clone

```bash
# download and build safex/tropix
git clone https://github.com/safex/tropix
cd tropix
```
###Once you are inside tropix folder you can use the automation robots:

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

```bash
# overview of usage
# This robot will buy up to a certain amount after the elapse of an interval of time.
1. Enter Bittrex API key
2. Enter Bittrex Secret key
3. Enter the index of the Pair you want to trade
4. Enter a number of coins you want to buy
5. Enter how often every number of seconds you want to buy from option 4's number of coins
```


#### Bittrex CLI trading module

```bash
cargo run --bin bittrexcli
```