package main

import (
	"fmt"
	"math/big"

	"github.com/ethereum/go-ethereum/core/types"
)

func main() {
	jsonTxn := `// TODO`
	var tx2 types.Transaction
	if err := tx2.UnmarshalJSON([]byte(jsonTxn)); err != nil {
		panic(err)
	}

	// fmt.Println(tx.Hash().Hex())
	fmt.Printf("Real hash: %s\n", tx2.Hash().Hex())

	cc := types.NewCancunSigner(big.NewInt(31337))
	sig := cc.Hash(&tx2)

	fmt.Printf("Signing hash: %s\n", sig)
}
