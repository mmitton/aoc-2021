// https://adventofcode.com/2019

package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	s := bufio.NewScanner(os.Stdin)
	var defs []string

	for s.Scan() {
		defs = append(defs, s.Text())
	}
	moons, err := newMoons(defs)
	if err != nil {
		fmt.Printf("ERROR: %v", err)
	}

	repeatsAfter := moons.RepeatsAfter()
	fmt.Printf("Repeats after: %v\n", repeatsAfter)
}
