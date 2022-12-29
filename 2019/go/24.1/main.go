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

	b, err := newBugs(def)
	if err != nil {
		fmt.Printf("ERROR: %v", err)
		return
	}

	b.Output()
	seen := make(map[int]bool)
	for {
		bioRating := b.BiodiversityRating()
		if seen[bioRating] {
			b.Output()
			break
		}
		seen[bioRating] = true
		b.Tick()
	}
}
