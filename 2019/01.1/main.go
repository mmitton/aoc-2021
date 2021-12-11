// https://adventofcode.com/2019

package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
)

func calcFuel(mass int64) int64 {
	return int64(math.Floor(float64(mass)/3.0) - 2.0)
}

func main() {
	s := bufio.NewScanner(os.Stdin)
	var totalFuel int64 = 0
	for s.Scan() {
		mass, err := strconv.ParseInt(s.Text(), 10, 64)
		if err != nil {
			fmt.Printf("%q: %v", s.Text(), err)
		} else {
			fuel := calcFuel(mass)
			totalFuel += fuel
			fmt.Printf("mass:%v  fuel:%v\n", mass, fuel)

		}
	}
	fmt.Printf("Total fuel needed:%v\n", totalFuel)
}
