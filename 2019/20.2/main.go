package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	s := bufio.NewScanner(os.Stdin)
	var def []string

	for s.Scan() {
		def = append(def, s.Text())
	}

	m, err := newMaze(def)
	if err != nil {
		fmt.Printf("ERROR: %v", err)
		return
	}

	steps, err := m.FindShortestPath("AA", "ZZ")
	if err != nil {
		fmt.Printf("ERROR: %v", err)
		return
	}

	fmt.Printf("Steps: %v\n", steps)
}
