// https://adventofcode.com/2019

package main

import (
	"fmt"
	"os"
)

func main() {
	m, err := decodeMapFromReader(os.Stdin)
	if err != nil {
		fmt.Printf("ERROR: %v\n", err)
		return
	}

	bestSee, x, y, orderedAsteroids := m.findBestBase()
	fmt.Printf("Best bet is at %v,%v where %v asteroids can be seen\n", x, y, bestSee)
	for i, a := range orderedAsteroids {
		fmt.Printf("asteroid %v to be vaporized is at %v,%v  %v\n", i+1, a.x, a.y, (a.x*100)+a.y)
	}
}
