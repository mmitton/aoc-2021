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
	f := math.Floor(float64(mass)/3.0) - 2.0
	if f <= 0 {
		f = 0
	}
	return int64(f)
}

func calcTotalFuel(masses ...int64) int64 {
	totalFuel := int64(0)
	for _, m := range masses {
		totalFuel += calcFuel(m)
	}

	added := totalFuel
	for {
		if added <= 0 {
			break
		}
		fmt.Printf("%v\n", added)
		added = calcFuel(added)
		totalFuel += added
	}

	return totalFuel
}

func main() {
	s := bufio.NewScanner(os.Stdin)
	totalFuel := int64(0)
	for s.Scan() {
		mass, err := strconv.ParseInt(s.Text(), 10, 64)
		if err != nil {
			fmt.Printf("%q: %v", s.Text(), err)
		} else {
			fuel := calcTotalFuel(mass)
			totalFuel += fuel
			fmt.Printf("mass:%v  fuel:%v\n", mass, fuel)
		}
	}

	fmt.Printf("Total fuel needed:%v\n", totalFuel)
}
