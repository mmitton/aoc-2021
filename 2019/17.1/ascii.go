package main

import (
	"github.com/buger/goterm"
)

type ASCII struct {
	ic   *Intcode
	x, y int64
	head *Tile
	tail *Tile
}

type RobotDirection rune

const (
	RobotNorth RobotDirection = '^'
	RobotSouth                = 'v'
	RobotEast                 = '>'
	RobotWest                 = '<'
	RobotNone                 = ' '
)

type Tile struct {
	x, y           int64
	isScaffold     bool
	isIntersection bool
	robotDirection RobotDirection
	next           *Tile
}

func newASCII(program []int64) *ASCII {
	a := &ASCII{ic: newIntcode(program)}
	a.ic.Input = a.Input
	a.ic.Output = a.Output

	a.x = 0
	a.y = 0

	return a
}

func (a *ASCII) FindTile(x, y int64, create bool) *Tile {
	for t := a.head; t != nil; t = t.next {
		if t.x == x && t.y == y {
			return t
		}
	}

	if !create {
		return nil
	}
	t := &Tile{x: x, y: y, robotDirection: RobotNone}
	if a.head == nil {
		a.head = t
		a.tail = t
	} else {
		a.tail.next = t
		a.tail = t
	}
	return t
}

func (a *ASCII) Bounds() (minX int64, minY int64, maxX int64, maxY int64) {
	for t := a.head; t != nil; t = t.next {
		if t.x < minX {
			minX = t.x
		}
		if t.x > maxX {
			maxX = t.x
		}
		if t.y < minY {
			minY = t.y
		}
		if t.y > maxY {
			maxY = t.y
		}
	}
	return
}

func (a *ASCII) OutputMap() {
	goterm.MoveCursor(1, 1)
	minX, minY, maxX, maxY := a.Bounds()
	for y := minY; y <= maxY; y++ {
		for x := minX; x <= maxX; x++ {
			goterm.MoveCursor(int(x-minX+1), int(y-minY+2))
			t := a.FindTile(x, y, false)
			if t != nil {
				goterm.Print(t)
			}
		}
	}
	goterm.MoveCursor(1, int(maxY-minY+3))

	goterm.Flush()
}

func (a *ASCII) Run() error {
	goterm.Clear()
	err := a.ic.run()
	a.OutputMap()
	return err
}

func (a *ASCII) Input() (v int64) {
	return 0
}

func (a *ASCII) Output(v int64) {
	if v == 10 {
		a.x = 0
		a.y++
		return
	}
	t := a.FindTile(a.x, a.y, true)
	r := rune(v)
	switch r {
	case '.':
		t.isScaffold = false
		t.isIntersection = false
	case '#':
		t.isScaffold = true
	case '^', 'v', '>', '<':
		t.isScaffold = true
		t.robotDirection = RobotDirection(r)
	}
	a.x++
}

func (a *ASCII) FindIntersections() []*Tile {
	var ret []*Tile
	minX, minY, maxX, maxY := a.Bounds()
	for y := minY; y <= maxY; y++ {
		for x := minX; x <= maxX; x++ {
			c := a.FindTile(x, y, false)
			if c == nil || !c.isScaffold {
				continue
			}

			exits := 0
			if a := a.FindTile(x-1, y, false); a != nil && a.isScaffold {
				exits++
			}
			if a := a.FindTile(x+1, y, false); a != nil && a.isScaffold {
				exits++
			}
			if a := a.FindTile(x, y-1, false); a != nil && a.isScaffold {
				exits++
			}
			if a := a.FindTile(x, y+1, false); a != nil && a.isScaffold {
				exits++
			}

			if exits > 2 {
				c.isIntersection = true
				ret = append(ret, c)
			}
		}
	}

	return ret
}

func (t *Tile) String() string {
	if t.robotDirection == RobotNone {
		if t.isIntersection {
			return "O"
		} else if t.isScaffold {
			return "#"
		} else {
			return "."
		}
	} else {
		return t.robotDirection.String()
	}
}

func (rd RobotDirection) String() string {
	switch rd {
	case RobotNorth:
		return "^"
	case RobotSouth:
		return "v"
	case RobotEast:
		return ">"
	case RobotWest:
		return "<"
	}
	return "?"
}
