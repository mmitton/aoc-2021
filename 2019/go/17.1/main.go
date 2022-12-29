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

	a := newASCII(program)
	if err := a.Run(); err != nil {
		fmt.Printf("Err running IC: %v", err)
		return
	}

	intersections := a.FindIntersections()
	a.OutputMap()

	sum := int64(0)
	for _, i := range intersections {
		ap := i.x * i.y
		fmt.Printf("Intersection at %v,%v:  alignmentParameter:%v\n", i.x, i.y, ap)
		sum += ap
	}
	fmt.Printf("Sum of alignment parameters: %v\n", sum)
}
