// https://adventofcode.com/2019

package main

import (
	"fmt"
	"os"
)

func main() {
	program, err := readProgram(os.Stdin)
	if err != nil {
		fmt.Printf("Err %v\n", err)
		return
	}

	a := newArcade(program)
	a.ic.memory[0] = 2
	if err := a.Run(); err != nil {
		fmt.Printf("Err running IC: %v", err)
		return
	}

	fmt.Printf("Total Blocks: %v\n", a.TotalBlocks())
}
