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

	r := newRobot(program)
	r.OutputMap()

	if err := r.Run(); err != nil {
		fmt.Printf("Err running IC: %v", err)
		return
	}

	pathToOxygen := r.FindPathFromTo(r.head, r.oxygenAt)
	fmt.Printf("Path to Oxygen: %v\n", Path(pathToOxygen))
	fmt.Printf("Distance: %v\n", len(pathToOxygen)-1)
	fmt.Printf("Time to Fill: %v\n", r.TimeToFill())
}
