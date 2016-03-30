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



#### Poloniex Auto Buy Robot

```bash
# download and build safex/tropix
git clone https://github.com/safex/tropix
cd tropix
cargo run --bin poloautobuy
```
#### Poloniex Auto Sell Robot

```bash
# download and build safex/tropix
git clone https://github.com/safex/tropix
cd tropix
cargo run --bin poloautosell
```

#### Bittrex CLI trading module

```bash
# download and build safex/tropix
git clone https://github.com/safex/tropix
cd tropix
cargo run --bin bittrexcli
```