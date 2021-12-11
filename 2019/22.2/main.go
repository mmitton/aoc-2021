// https://adventofcode.com/2019

package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	s := bufio.NewScanner(os.Stdin)
	var commands []string
	for s.Scan() {
		command := s.Text()
		commands = append(commands, command)
	}

	d := newQuickDeck(119315717514047, commands)
	d.MapPosition(1)

	start := 2020
	pos := start
	loops := 101741582076661
	for i := 0; i < loops; i++ {
		pos2 := 0 // d.MapPosition(pos)
		if i%10000000000 == 0 {
			fmt.Printf("%v (%v): %v => %v\n", i, i*100/loops, pos, pos2)
		}
		pos = pos2
	}

	fmt.Printf("Position of card %v is %v\n", start, pos)
}
