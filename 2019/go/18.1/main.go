package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	s := bufio.NewScanner(os.Stdin)
	var caveMap []string

	for s.Scan() {
		caveMap = append(caveMap, s.Text())
	}
	c, err := newCave(caveMap)
	if err != nil {
		fmt.Printf("ERROR: %v", err)
		return
	}

	paths, err := c.FindShortestPaths()
	if err != nil {
		fmt.Printf("ERROR: %v", err)
		return
	}

	for i, path := range paths {
		fmt.Printf("paths[%v]: %v %v\n", i, path.steps, path.Keys())
	}
}
