package main

import "fmt"

type ArcadeWaitState uint8

const (
	ArcadeWaitX ArcadeWaitState = iota
	ArcadeWaitY
	ArcadeWaitTile
)

type Arcade struct {
	ic        *Intcode
	head      *Tile
	tail      *Tile
	x, y      int64
	waitState ArcadeWaitState
}

type TileState uint8

const (
	TileEmpty  TileState = 0
	TileWall             = 1
	TileBlock            = 2
	TilePaddle           = 3
	TileBall             = 4
)

type Tile struct {
	x, y  int64
	state TileState
	next  *Tile
}

func newArcade(program []int64) *Arcade {
	a := &Arcade{ic: newIntcode(program)}
	a.ic.Input = a.Input
	a.ic.Output = a.Output
	return a
}

func (a *Arcade) FindTile(x, y int64) *Tile {
	for t := a.head; t != nil; t = t.next {
		if t.x == x && t.y == y {
			return t
		}
	}

	t := &Tile{x: x, y: y}
	if a.head == nil {
		a.head = t
		a.tail = t
	} else {
		a.tail.next = t
		a.tail = t
	}
	return t
}

func (a *Arcade) CurrentTile() *Tile {
	return a.FindTile(a.x, a.y)
}

func (a *Arcade) Bounds() (minX int64, minY int64, maxX int64, maxY int64) {
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

func (a *Arcade) Dump() {
	minX, minY, maxX, maxY := a.Bounds()
	for y := minY; y <= maxY; y++ {
		for x := minX; x <= maxX; x++ {
			t := a.FindTile(x, y)
			fmt.Print(t.state.String())
		}
		fmt.Print("\n")
	}
	fmt.Print("\n")
}

func (a *Arcade) Run() error {
	return a.ic.run()
}

func (a *Arcade) Input() int64 {
	return 0
}

func (a *Arcade) Output(v int64) {
	switch a.waitState {
	case ArcadeWaitX:
		a.x = v
		a.waitState = ArcadeWaitY
	case ArcadeWaitY:
		a.y = v
		a.waitState = ArcadeWaitTile
	case ArcadeWaitTile:
		a.CurrentTile().state = TileState(v)
		a.waitState = ArcadeWaitX
		a.Dump()
	}
}

func (t TileState) String() string {
	switch t {
	case TileEmpty:
		return " "
	case TileWall:
		return "|"
	case TileBlock:
		return "X"
	case TilePaddle:
		return "_"
	case TileBall:
		return "O"
	}
	return "?"
}

func (a *Arcade) TotalBlocks() int {
	totalBlocks := 0
	for t := a.head; t != nil; t = t.next {
		if t.state == TileBlock {
			totalBlocks++
		}
	}

	return totalBlocks
}
