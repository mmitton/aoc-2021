// https://adventofcode.com/2019

package main

import (
	"fmt"
	"os"
)

func main() {
	f, err := os.Open("input")
	if err != nil {
		fmt.Printf("Err %v\n", err)
		return
	}
	defer f.Close()

	program, err := readProgram(f)
	if err != nil {
		fmt.Printf("Err %v\n", err)
		return
	}

	fmt.Printf("Read %v int64\n", len(program))

	a := newASCII(program)
	if err := a.Run(); err != nil {
		fmt.Printf("Err running IC: %v", err)
		return
	}
}
