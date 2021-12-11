package main

import (
	"errors"
	"fmt"
)

type Bugs struct {
	tickCount int
	bugs      [5][5]Bug
	next      [5][5]Bug
}
type Bug bool

func (b Bug) String() string {
	if b {
		return "#"
	}
	return "."
}

func newBugs(def []string) (*Bugs, error) {
	if len(def) != 5 {
		fmt.Printf("def: %v\n", def)
		return nil, errors.New("Bad Def")
	}
	b := &Bugs{}
	for y := 0; y < 5; y++ {
		if len(def[y]) != 5 {
			return nil, errors.New("Bad Def")
		}
		for x := 0; x < 5; x++ {
			if def[y][x] == '#' {
				b.bugs[y][x] = true
			}
		}
	}

	return b, nil
}

func (b *Bugs) BiodiversityRating() int {
	bioRating := 0
	po2 := 1
	for y := 0; y < 5; y++ {
		for x := 0; x < 5; x++ {
			if b.bugs[y][x] {
				bioRating += po2
			}
			po2 *= 2
		}
	}

	return bioRating
}

func (b *Bugs) Tick() {
	bugAt := func(x, y int) Bug {
		if x < 0 || x >= 5 {
			return false
		}
		if y < 0 || y >= 5 {
			return false
		}

		return b.bugs[y][x]
	}

	for y := 0; y < 5; y++ {
		for x := 0; x < 5; x++ {
			count := 0
			if bugAt(x-1, y) {
				count++
			}
			if bugAt(x+1, y) {
				count++
			}
			if bugAt(x, y-1) {
				count++
			}
			if bugAt(x, y+1) {
				count++
			}

			if b.bugs[y][x] && count != 1 {
				b.next[y][x] = false
			} else if !b.bugs[y][x] && (count == 1 || count == 2) {
				b.next[y][x] = true
			} else {
				b.next[y][x] = b.bugs[y][x]
			}
		}
	}

	b.tickCount++
	b.next, b.bugs = b.bugs, b.next
}

func (b *Bugs) Output() {
	if b.tickCount == 0 {
		fmt.Printf("Initial state:\n")
	} else {
		fmt.Printf("After %v minute(s):\n", b.tickCount)
	}
	fmt.Printf("Bio Rating: %v\n", b.BiodiversityRating())
	for y := 0; y < 5; y++ {
		for x := 0; x < 5; x++ {
			fmt.Print(b.bugs[y][x])
		}
		fmt.Println()
	}
	fmt.Println()
}
