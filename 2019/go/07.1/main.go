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

	for a := int64(0); a <= 4; a++ {
		for b := int64(0); b <= 4; b++ {
			if a == b {
				continue
			}
			for c := int64(0); c <= 4; c++ {
				if a == c || b == c {
					continue
				}
				for d := int64(0); d <= 4; d++ {
					if a == d || b == d || c == d {
						continue
					}
					for e := int64(0); e <= 4; e++ {
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
