package main

import (
	"errors"
	"fmt"
)

type Bugs struct {
	tickCount  int
	multiLevel bool
	maxLevel   int
	minLevel   int
	bugs       []*Bug
}

type Bug struct {
	alive       bool
	next        bool
	x, y, level int
}

func newBugs(multiLevel bool, def []string) (*Bugs, error) {
	if len(def) != 5 {
		fmt.Printf("def: %v\n", def)
		return nil, errors.New("Bad Def")
	}
	b := &Bugs{multiLevel: multiLevel}
	for y := 0; y < 5; y++ {
		if len(def[y]) != 5 {
			return nil, errors.New("Bad Def")
		}
		for x := 0; x < 5; x++ {
			if def[y][x] == '#' {
				b.setAlive(x, y, 0, true)
			}
		}
	}

	return b, nil
}

func (b *Bugs) setAlive(x, y, level int, alive bool) {
	for _, bug := range b.bugs {
		if bug.x == x && bug.y == y && bug.level == level {
			bug.alive = alive
			return
		}
	}

	if alive {
		bug := &Bug{x: x, y: y, level: level}
		bug.alive = alive

		b.bugs = append(b.bugs, bug)
		if b.minLevel > level {
			b.minLevel = level
		}
		if b.maxLevel < level {
			b.maxLevel = level
		}
	}
}

func (b *Bugs) setNext(x, y, level int, next bool) {
	for _, bug := range b.bugs {
		if bug.x == x && bug.y == y && bug.level == level {
			bug.next = next
			return
		}
	}

	if next {
		bug := &Bug{x: x, y: y, level: level}
		bug.next = next

		b.bugs = append(b.bugs, bug)
		if b.minLevel > level {
			b.minLevel = level
		}
		if b.maxLevel < level {
			b.maxLevel = level
		}
	}
}

func (b *Bugs) isAlive(x, y, level int) bool {
	for _, bug := range b.bugs {
		if bug.x == x && bug.y == y && bug.level == level {
			return bug.alive
		}
	}
	return false
}

func (b *Bugs) BiodiversityRating() int {
	bioRating := 0
	po2 := 1
	for level := b.minLevel; level <= b.maxLevel; level++ {
		for y := 0; y < 5; y++ {
			for x := 0; x < 5; x++ {
				if b.isAlive(x, y, level) {
					bioRating += po2
				}
				po2 *= 2
			}
		}
	}

	return bioRating
}

func (b *Bugs) Tick() {
	minLevel := b.minLevel
	maxLevel := b.maxLevel
	if b.multiLevel {
		minLevel--
		maxLevel++
	}

	var isAliveInt func(x, y, level, cx, cy int) int
	isAliveInt = func(x, y, level, cx, cy int) int {
		if b.multiLevel {
			if y == -1 {
				return isAliveInt(2, 1, level-1, cx, cy)
			}
			if y == 5 {
				return isAliveInt(2, 3, level-1, cx, cy)
			}
			if x == -1 {
				return isAliveInt(1, 2, level-1, cx, cy)
			}
			if x == 5 {
				return isAliveInt(3, 2, level-1, cx, cy)
			}
			if x == 2 && y == 2 {
				// Map Inner
				cnt := 0
				for i := 0; i < 5; i++ {
					if cx == 2 && cy == 1 {
						// Top row
						cnt += isAliveInt(i, 0, level+1, cx, cy)
					} else if cx == 2 && cy == 3 {
						// Bottom row
						cnt += isAliveInt(i, 4, level+1, cx, cy)
					} else if cx == 1 && cy == 2 {
						// Left row
						cnt += isAliveInt(0, i, level+1, cx, cy)
					} else if cx == 3 && cy == 2 {
						// Right row
						cnt += isAliveInt(4, i, level+1, cx, cy)
					}
				}
				return cnt
			}
		}
		if b.isAlive(x, y, level) {
			return 1
		}
		return 0
	}
	aliveNeighbors := func(x, y, level int) int {
		if b.multiLevel && x == 2 && y == 2 {
			return 0
		}
		return isAliveInt(x-1, y, level, x, y) +
			isAliveInt(x+1, y, level, x, y) +
			isAliveInt(x, y-1, level, x, y) +
			isAliveInt(x, y+1, level, x, y)
	}

	for level := minLevel; level <= maxLevel; level++ {
		for y := 0; y < 5; y++ {
			for x := 0; x < 5; x++ {
				count := aliveNeighbors(x, y, level)
				isAlive := b.isAlive(x, y, level)

				if isAlive && count != 1 {
					b.setNext(x, y, level, false)
				} else if !isAlive && (count == 1 || count == 2) {
					b.setNext(x, y, level, true)
				} else {
					b.setNext(x, y, level, isAlive)
				}
			}
		}
	}

	for _, bug := range b.bugs {
		bug.alive = bug.next
	}
	b.tickCount++
}

func (b *Bugs) Output() {
	if b.tickCount == 0 {
		fmt.Printf("Initial state:\n")
	} else {
		fmt.Printf("After %v minute(s):\n", b.tickCount)
	}
	fmt.Printf("Bio Rating: %v\n", b.BiodiversityRating())
	for level := b.minLevel; level <= b.maxLevel; level++ {
		fmt.Printf("Level %v\n", level)
		for y := 0; y < 5; y++ {
			for x := 0; x < 5; x++ {
				if b.multiLevel && x == 2 && y == 2 {
					fmt.Print("?")
				} else if b.isAlive(x, y, level) {
					fmt.Print("#")
				} else {
					fmt.Print(".")
				}
			}
			fmt.Println()
		}
		fmt.Println()
	}
}
