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
   
putting it into a web page and install dependencies
```
npm init wasm-app www
cd www
npm install
``` 

Then proceed with the tutorial by adding a dependency to our wasm-test project and updating the index.js to import this dependency. Finally `npm install`.

# If error

if error when running `npm run start`, look into [this](https://stackoverflow.com/questions/69692842/error-message-error0308010cdigital-envelope-routinesunsupported), or just try this:  
```
export NODE_OPTIONS=--openssl-legacy-provider
```
