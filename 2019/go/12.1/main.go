// https://adventofcode.com/2019

package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	s := bufio.NewScanner(os.Stdin)
	var moons Moons
	for s.Scan() {
		line := s.Text()
		m, err := newMoon(line)
		if err != nil {
			fmt.Printf("ERROR: %v => %v\n", line, err)
			return
		}
		moons = append(moons, m)
	}

	for i := 1; i <= 1000; i++ {
		moons.Step()
	}

	fmt.Printf("Total Energy: %v\n", moons.TotalEnergy())
}
