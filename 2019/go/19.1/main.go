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

	var width, height int64 = 50, 50
	d := newDrone(program)

	totalPulled := 0
	for y := int64(0); y < height; y++ {
		for x := int64(0); x < width; x++ {
			pulledAt, err := d.PulledAt(x, y)
			if err != nil {
				fmt.Printf("\nERROR: %v\n", err)
				return
			}
			if pulledAt {
				totalPulled++
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}

	fmt.Printf("Total pulled: %v\n", totalPulled)
}
