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
	program[0] = 2

	a := newASCII(program)
	if err := a.Run(); err != nil {
		fmt.Printf("Err running IC: %v", err)
		return
	}

	fmt.Printf("Space Dust: %v\n", a.lastOutput)
}
