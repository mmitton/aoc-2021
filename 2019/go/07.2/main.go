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
	}

	var maxSignal int64
	var values []int64

	var settings = []int64{5, 6, 7, 8, 9}

	for _, a := range settings {
		for _, b := range settings {
			if a == b {
				continue
			}
			for _, c := range settings {
				if a == c || b == c {
					continue
				}
				for _, d := range settings {
					if a == d || b == d || c == d {
						continue
					}
					for _, e := range settings {
						if a == e || b == e || c == e || d == e {
							continue
						}
						signal := amplifierCircuit(program, a, b, c, d, e)
						if signal > maxSignal {
							maxSignal = signal
							values = []int64{a, b, c, d, e}
						}
					}
				}
			}
		}
	}

	fmt.Printf("MaxSignal: %v  Settings: %v\n", maxSignal, values)
}
