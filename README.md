# wasm

[wasm book](https://rustwasm.github.io/docs/book/game-of-life/hello-world.html)
install rust    
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```
  
wasm pack  
```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh 
```
and cargo generate  
```
cargo install cargo-generate
```

Node and update npm
```
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.2/install.sh | bash
nvp install node
npm install npm@latest -g
```
  
Prepare project
```
cargo generate --git https://github.com/rustwasm/wasm-pack-template
cd name-of-project
wasm-pack build
```
