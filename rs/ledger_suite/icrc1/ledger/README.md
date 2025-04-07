# Init
 
 This repo update the dfx.json
 ```json
 {
     "canisters": {
       "icrc1-ledger": {
         "type": "rust",
         "package": "ic-icrc1-ledger",
         "candid": "ledger.did"
       }
     },
     "defaults": {
     "build": {
       "args": "",
       "packtool": ""
     }
   },
   "output_env_file": ".env",
   "version": 1
 }
 ```
 
 When deploy, use `cargo build --target wasm32-unknown-unknown --release -p ic-icrc1-ledger --locked` to generate the wasm at `target/wasm32-unknown-unknown/release/ic-icrc1-ledger.wasm`
 
 Change the file name `ic-icrc1-ledger.wasm` to `ic_icrc1_ledger.wasm`
 
 and `dfx deploy` 