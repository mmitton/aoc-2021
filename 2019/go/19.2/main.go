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

	d := newDrone(program)

	x, y, err := d.FindSquare(100, 100)
	if err != nil {
		fmt.Printf("\nERROR: %v\n", err)
		return
	}

	fmt.Printf("x:%v  y:%v  answer:%v\n", x, y, (x*10000)+y)
}
