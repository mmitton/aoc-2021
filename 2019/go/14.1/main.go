// https://adventofcode.com/2019

package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	s := bufio.NewScanner(os.Stdin)
	var defs []string

	for s.Scan() {
		defs = append(defs, s.Text())
	}

	nf, err := newNanoFactory(defs)
	if err != nil {
		fmt.Printf("ERROR: %v\n", err)
		return
	}

	ore := nf.findChemical("ORE")
	fmt.Printf("Total ORE required: %v\n", ore.required)
}
