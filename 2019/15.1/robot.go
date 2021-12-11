package main

import (
	"fmt"
	"strings"

	"github.com/buger/goterm"
)

type Robot struct {
	ic   *Intcode
	head *Tile
	tail *Tile

	curTile  *Tile
	nextTile *Tile
	path     []*Tile

	oxygenAt *Tile
}

type TileState int8

const (
	TileUnknown TileState = -1
	TileWall              = 0
	TileEmpty             = 1
	TileOxygen            = 2
)

type Tile struct {
	x, y   int64
	state  TileState
	walked bool
	next   *Tile
}

func newRobot(program []int64) *Robot {
	r := &Robot{ic: newIntcode(program)}
	r.ic.Input = r.Input
	r.ic.Output = r.Output

	r.curTile = r.FindTile(0, 0, true, true)
	r.curTile.state = TileEmpty

	return r
}

func (r *Robot) FindTile(x, y int64, create bool, expandUnknown bool) *Tile {
	defer func() {
		if expandUnknown {
			r.FindTile(x-1, y, true, false)
			r.FindTile(x+1, y, true, false)
			r.FindTile(x, y-1, true, false)
			r.FindTile(x, y+1, true, false)
		}
	}()

	for t := r.head; t != nil; t = t.next {
		if t.x == x && t.y == y {
			return t
		}
	}

	if !create {
		return nil
	}
	t := &Tile{x: x, y: y, state: TileUnknown}
	if r.head == nil {
		r.head = t
		r.tail = t
	} else {
		r.tail.next = t
		r.tail = t
	}
	return t
}

func (r *Robot) Bounds() (minX int64, minY int64, maxX int64, maxY int64) {
	for t := r.head; t != nil; t = t.next {
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

func (r *Robot) OutputMap() {
	goterm.MoveCursor(1, 1)
	minX, minY, maxX, maxY := r.Bounds()
	for y := minY; y <= maxY; y++ {
		for x := minX; x <= maxX; x++ {
			goterm.MoveCursor(int(x-minX+1), int(y-minY+2))
			t := r.FindTile(x, y, false, false)
			if t == r.curTile {
				goterm.Print("D")
			} else if t == nil {
				goterm.Print(" ")
			} else {
				goterm.Print(t.state.String())
			}
		}
	}
	goterm.MoveCursor(1, int(maxY-minY+3))

	goterm.Flush()
}

func (r *Robot) Run() error {
	goterm.Clear()
	err := r.ic.run()
	// r.OutputMap(true)
	return err
}

func (r *Robot) Input() (v int64) {
	//defer time.Sleep(5 * time.Millisecond)

	if len(r.path) == 0 {
		r.path = r.findNewPath()
	}
	if len(r.path) == 0 {
		return 0
	}

	r.nextTile = r.path[0]
	r.path = r.path[1:]

	if r.curTile.x == r.nextTile.x && r.curTile.y == r.nextTile.y+1 {
		// North
		return 1
	} else if r.curTile.x == r.nextTile.x && r.curTile.y == r.nextTile.y-1 {
		// South
		return 2
	} else if r.curTile.x == r.nextTile.x-1 && r.curTile.y == r.nextTile.y {
		// East
		return 4
	} else if r.curTile.x == r.nextTile.x+1 && r.curTile.y == r.nextTile.y {
		// West
		return 3
	}

	return 0
}

func (r *Robot) newPath(p []*Tile, t *Tile) []*Tile {
	var ret []*Tile
	for _, t := range p {
		ret = append(ret, t)
	}
	return append(ret, t)
}

func (r *Robot) clearWalked() {
	for t := r.head; t != nil; t = t.next {
		t.walked = false
	}
}

func (r *Robot) findPath(startTile *Tile, isDest func(*Tile) bool) []*Tile {
	r.clearWalked()

	startTile.walked = true
	var work = [][]*Tile{r.newPath(nil, startTile)}

	for len(work) > 0 {
		p := work[0]
		work = work[1:]
		t := p[len(p)-1]

		tiles := []*Tile{
			r.FindTile(t.x-1, t.y, false, false),
			r.FindTile(t.x+1, t.y, false, false),
			r.FindTile(t.x, t.y-1, false, false),
			r.FindTile(t.x, t.y+1, false, false),
		}

		for _, t2 := range tiles {
			if t2 == nil {
				continue
			}
			if t2.walked {
				continue
			}
			if t2.state == TileWall {
				continue
			}
			t2.walked = true

			np := r.newPath(p, t2)
			if isDest(t2) {
				return np
			}
			work = append(work, np)
		}
	}

	return nil
}

func (r *Robot) findPathTo(t *Tile) []*Tile {
	return r.findPath(r.FindTile(0, 0, false, false), func(t2 *Tile) bool {
		if t2 == t {
			return true
		}
		return false
	})
}

func (r *Robot) findNewPath() []*Tile {
	// Find next tile to explore
	newPath := r.findPath(r.FindTile(0, 0, false, false), func(t2 *Tile) bool {
		if t2.state == TileUnknown {
			return true
		}
		return false
	})

	if newPath == nil {
		return nil
	}

	destTile := newPath[len(newPath)-1]
	newPath = r.findPath(r.curTile, func(t2 *Tile) bool {
		if t2 == destTile {
			return true
		}
		return false
	})
	if newPath == nil {
		return nil
	}

	return newPath[1:]
}

func (r *Robot) Output(v int64) {
	t := TileState(v)
	if r.nextTile.state != t {
		r.OutputMap()
	}

	r.nextTile.state = t
	switch t {
	case TileEmpty, TileOxygen:
		r.curTile = r.nextTile
		r.FindTile(r.curTile.x, r.curTile.y, true, true)
		if t == TileOxygen {
			r.oxygenAt = r.curTile
		}
	}
}

func (t TileState) String() string {
	switch t {
	case TileWall:
		return "#"
	case TileEmpty:
		return "."
	case TileOxygen:
		return "o"
	}
	return "?"
}

type Path []*Tile

func (p Path) String() string {
	var ret []string

	for _, t := range p {
		if t != nil {
			ret = append(ret, t.String())
		}
	}
	return fmt.Sprintf("{%v}", strings.Join(ret, ", "))
}

func (t *Tile) String() string {
	return fmt.Sprintf("{%v,%v %v}", t.x, t.y, t.state.String())
}
