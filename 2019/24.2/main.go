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

	b, err := newBugs(true, def)
	if err != nil {
		fmt.Printf("ERROR: %v", err)
		return
	}

	b.Output()
	for i := 0; i < 200; i++ {
		b.Tick()
	}
	cnt := 0
	for _, bug := range b.bugs {
		if bug.alive {
			cnt++
		}
	}
	fmt.Printf("minLevel:%v  maxLevel:%v  cnt:%v\n", b.minLevel, b.maxLevel, cnt)
}
