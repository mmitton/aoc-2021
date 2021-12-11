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

	painter := newPainter(program)
	if err := painter.run(); err != nil {
		fmt.Printf("Err running IC: %v", err)
		return
	}

	output := painter.generateOutput(0, 0, 0, 0, false)
	for _, line := range output {
		fmt.Println(line)
	}

	fmt.Printf("Number of panels painted: %v\n", painter.numberPainted())
}
