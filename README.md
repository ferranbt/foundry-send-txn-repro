Run the rust script:

```
$ cargo run
```

This script:

- Sends a transaction to the eth_sendTransaction endpoint of an Avail node.
- Outputs the hash of the transaction.
- Returns the JSON for the transaction.

Take the JSON from the output and put it in the `main.go` script.

```
$ go run main.go
```

This script:

- Decodes the JSON file and outputs the transaction hash and the signing hash.

The transaction hash from Avail does not match the transaction hash from Geth.
