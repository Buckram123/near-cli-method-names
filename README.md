# Contract method names extension

## Available commands:

### blockchain
With this command, you can view the names of contract methods in the blockchain. You need to specify rpc node and account name. You can specify BlockId, by block height, block hash or select `final`.

In the terminal command line type:
```txt
./near-cli method-names blockchain mainnet final aurora
```

<details><summary><i>The result of this command will be as follows:</i></summary>

```txt
new
get_version
get_owner
get_bridge_prover
get_chain_id
get_upgrade_index
...
```
</details>

### wasm
With this command, you can view names of contract methods from .wasm file.

In the terminal command line type:
```txt
./near-cli method-names wasm status_message.wasm
```

<details><summary><i>The result of this command will be as follows:</i></summary>

```txt
set_status
get_status
```
</details>