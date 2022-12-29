// https://adventofcode.com/2019

package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	s := bufio.NewScanner(os.Stdin)

	d, err := newDeck(10007)
	if err != nil {
		fmt.Printf("ERROR: %v", err)
		return
	}

	for s.Scan() {
		command := s.Text()
		if err := d.Shuffle(command); err != nil {
			fmt.Printf("ERROR: %q: %v", command, err)
			return
		}
	}

	for i, c := range d.current {
		if c == 2019 {
			fmt.Printf("Position of card %v is %v\n", c, i)
			return
		}
	}
}
