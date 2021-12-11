package main

import (
	"fmt"
	"sort"
	"strings"

	"github.com/buger/goterm"
)

type ASCII struct {
	ic   *Intcode
	x, y int64
	head *Tile
	tail *Tile

	input      string
	inputCount int
	lastOutput int64
}

type Direction rune

const (
	DirNorth Direction = '^'
	DirSouth Direction = 'v'
	DirEast  Direction = '>'
	DirWest  Direction = '<'
	DirNone  Direction = ' '
)

type Tile struct {
	x, y           int64
	isScaffold     bool
	isIntersection bool
	isEndpoint     bool
	dir            Direction
	next           *Tile
	neighbors      []*TileDirection
}

type TileDirection struct {
	tile           *Tile
	dir            Direction
	endpoint       *Tile
	pathToEndpoint Path
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
	t := &Tile{x: x, y: y, dir: DirNone}
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
	goterm.Clear()
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
	return err
}

func (a *ASCII) Input() (v int64) {
	if len(a.input) == 0 {
		a.OutputMap()
		commandString := a.FindShortestPath()
		fmt.Printf("Command String: %v\n", commandString)
		commands := commandString.Split()
		for _, command := range commands {
			fmt.Printf("Command (len:%2v): %v\n", len(command.String()), command)
			a.input += command.String() + "\n"
		}
		a.input += "n\n"
	}

	a.x = 0
	a.y = 0
	ret := a.input[0]
	a.input = a.input[1:]
	a.inputCount++
	return int64(ret)
}

func (a *ASCII) Output(v int64) {
	a.lastOutput = v
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
		t.dir = Direction(r)
	}
	a.x++
}

func (a *ASCII) FindIntersections() []*Tile {
	var intersections []*Tile
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
				c.isEndpoint = true
				intersections = append(intersections, c)
			}
			if exits == 1 {
				c.isEndpoint = true
			}
			if c.dir != DirNone {
				c.isEndpoint = true
			}
		}
	}

	return intersections
}

func (t *Tile) String() string {
	if t.dir == DirNone {
		if t.isIntersection {
			return "O"
		} else if t.isScaffold {
			return "#"
		} else {
			return "."
		}
	} else {
		return t.dir.String()
	}
}

func (rd Direction) String() string {
	switch rd {
	case DirNorth:
		return "^"
	case DirSouth:
		return "v"
	case DirEast:
		return ">"
	case DirWest:
		return "<"
	}
	return "?"
}

func (a *ASCII) mapTileDirections() {
	a.FindIntersections()

	var endpoints []*Tile
	for t := a.head; t != nil; t = t.next {
		t.neighbors = nil

		if t.isScaffold {
			surrounding := []*TileDirection{
				&TileDirection{a.FindTile(t.x-1, t.y, false), DirWest, nil, nil},
				&TileDirection{a.FindTile(t.x+1, t.y, false), DirEast, nil, nil},
				&TileDirection{a.FindTile(t.x, t.y-1, false), DirNorth, nil, nil},
				&TileDirection{a.FindTile(t.x, t.y+1, false), DirSouth, nil, nil},
			}
			for _, n := range surrounding {
				if n.tile == nil || !n.tile.isScaffold {
					continue
				}
				t.neighbors = append(t.neighbors, n)
			}

			if t.isEndpoint {
				endpoints = append(endpoints, t)
			}
		}
	}

	// Assign endpoints for endpoint neighbors
	for _, t := range endpoints {
		for _, n := range t.neighbors {
			curTile := n.tile
			for {
				n.pathToEndpoint = append(n.pathToEndpoint, curTile)
				if curTile.isEndpoint {
					n.endpoint = curTile
					break
				}

				var nextTile *Tile
				for _, n2 := range curTile.neighbors {
					if n2.tile == t && len(n.pathToEndpoint) == 1 {
						continue
					}
					if !n.pathToEndpoint.Contains(n2.tile) {
						nextTile = n2.tile
						break
					}
				}

				if nextTile == nil {
					panic("Cannot find path!")
				}
				curTile = nextTile
			}

		}
	}
}

type Path []*Tile

func (a *ASCII) FindShortestPath() CommandString {
	totalScaffolds := 0
	var paths []Path

	newPath := func(existingPath Path, newPath Path) Path {
		var p Path
		p = append(p, existingPath...)
		p = append(p, newPath...)
		return p
	}

	startingDir := DirNone
	for t := a.head; t != nil; t = t.next {
		if t.isScaffold {
			totalScaffolds++
		}
		if t.dir != DirNone {
			paths = append(paths, newPath(nil, Path{t}))
			startingDir = t.dir
		}
	}

	a.mapTileDirections()

	// var endpoints []*Tile
	// for t := a.head; t != nil; t = t.next {
	// 	if t.isEndpoint {
	// 		endpoints = append(endpoints, t)
	// 		for _, n := range t.neighbors {
	// 			fmt.Printf("Endpoint: %v,%v  %v %v\n", t.x, t.y, n.dir, n.pathToEndpoint)
	// 		}
	// 	}
	// }

	var completedPaths []CommandString
	checkedPaths := 0
	for len(paths) > 0 {
		p := paths[0]
		paths = paths[1:]
		checkedPaths++

		// if i%10000 == 0 {
		// 	a.OutputMap()
		// 	for _, t := range p {
		// 		goterm.MoveCursor(int(t.x+1), int(t.y+2))
		// 		goterm.Print("*")
		// 	}
		// 	goterm.Flush()
		// }
		if p.UniqueTiles() == totalScaffolds {
			cs := p.CommandString(startingDir)
			if split := cs.Split(); split != nil {
				completedPaths = append(completedPaths, cs)
			}
			continue
		}
		curTile := p[len(p)-1]
		dir := DirNone
		if len(p) > 1 {
			lastTile := p[len(p)-2]
			if lastTile.x == curTile.x {
				if lastTile.y < curTile.y {
					dir = DirSouth
				} else {
					dir = DirNorth
				}
			} else {
				if lastTile.x < curTile.x {
					dir = DirEast
				} else {
					dir = DirWest
				}
			}
		}

		moved := false
		for _, n := range curTile.neighbors {
			if !p.Contains(n.tile) {
				if dir == DirNone || dir == n.dir {
					p2 := newPath(p, n.pathToEndpoint)
					paths = append(paths, p2)
					moved = true
				}
			}
		}

		if !moved && len(p) > 1 {
			// fmt.Printf("Backtrack!\n")
			// for j := len(p) - 2; j >= 0; j-- {
			// 	backtrackTile := p[j]

			// 	var next []*Tile
			// 	for _, n := range backtrackTile.neighbors {
			// 		if !hasWalked(p, n.tile) {
			// 			next = append(next, n.tile)
			// 		}
			// 	}

			// 	if len(next) > 0 {
			// 		fmt.Printf("found backtrack point: %v,%v\n", backtrackTile.x, backtrackTile.y)
			// 		break
			// 	}
			// }
		}
	}

	fmt.Printf("Found %v completed paths out of %v considered paths\n", len(completedPaths), checkedPaths)
	if len(completedPaths) > 0 {
		return completedPaths[0]
	}
	return nil
}

func (p Path) String() string {
	var positions []string
	for _, t := range p {
		positions = append(positions, fmt.Sprintf("%v,%v", t.x, t.y))
	}
	return fmt.Sprintf("Path Length:%3v  Unique:%3v  {%v}", len(p), p.UniqueTiles(), strings.Join(positions, ", "))
}

func (p Path) Contains(t *Tile) bool {
	for _, t2 := range p {
		if t2 == t {
			return true
		}
	}
	return false
}

func (p Path) UniqueTiles() int {
	unique := 0
	for i := 0; i < len(p); i++ {
		found := false
		for j := i + 1; j < len(p); j++ {
			if p[i] == p[j] {
				found = true
				break
			}
		}

		if !found {
			unique++
		}
	}
	return unique
}

func (p Path) CommandString(dir Direction) CommandString {
	turns := map[Direction]map[Direction]string{
		DirNorth: map[Direction]string{
			DirNorth: "",
			DirWest:  "L",
			DirSouth: "LL",
			DirEast:  "R",
		},
		DirWest: map[Direction]string{
			DirNorth: "R",
			DirWest:  "",
			DirSouth: "L",
			DirEast:  "RR",
		},
		DirSouth: map[Direction]string{
			DirNorth: "LL",
			DirWest:  "R",
			DirSouth: "",
			DirEast:  "L",
		},
		DirEast: map[Direction]string{
			DirNorth: "L",
			DirWest:  "RR",
			DirSouth: "R",
			DirEast:  "",
		},
	}

	var ret CommandString
	lastDir := dir
	moveCount := int64(0)
	for i := 1; i < len(p); i++ {
		curTile, lastTile := p[i], p[i-1]

		if lastTile.x == curTile.x {
			if lastTile.y < curTile.y {
				dir = DirSouth
			} else {
				dir = DirNorth
			}
		} else {
			if lastTile.x < curTile.x {
				dir = DirEast
			} else {
				dir = DirWest
			}
		}

		turn := turns[lastDir][dir]
		if turn != "" {
			if moveCount != 0 {
				ret = append(ret, moveCount)
				moveCount = 0
			}
			for _, r := range turn {
				ret = append(ret, int64(r))
			}
		}
		moveCount++
		lastDir = dir
	}

	if moveCount != 0 {
		ret = append(ret, moveCount)
	}

	return ret
}

type CommandString []int64

func (cs CommandString) Equals(cs2 CommandString) bool {
	if len(cs) != len(cs2) {
		return false
	}

	for i := 0; i < len(cs); i++ {
		if cs[i] != cs2[i] {
			return false
		}
	}

	return true
}

func (cs CommandString) clone() CommandString {
	var cs2 CommandString
	cs2 = append(cs2, cs...)
	return cs2
}

func (cs CommandString) String() string {
	var ret []string

	for _, c := range cs {
		if c > 255 {
			ret = append(ret, fmt.Sprintf("%c", c-512+'a'))
		} else if c < 'A' {
			ret = append(ret, fmt.Sprintf("%d", c))
		} else {
			ret = append(ret, fmt.Sprintf("%c", c))
		}
	}

	return strings.Join(ret, ",")
}

func (cs CommandString) Inputs() []int64 {
	var ret []int64
	for _, c := range cs {
		ret = append(ret, ',', c)
	}
	return ret[1:]
}

func (cs CommandString) subCommand(s, l int) CommandString {
	if s+l > len(cs) {
		return nil
	}
	var cs2 CommandString
	for i := s; i < s+l; i++ {
		cs2 = append(cs2, cs[i])
	}

	if len(cs2.String()) > 20 {
		return nil
	}
	return cs2
}

func (cs CommandString) Split() []CommandString {
	var findPattern func(pattern CommandString, cs2 CommandString, routines []CommandString) CommandString
	findPattern = func(pattern CommandString, cs2 CommandString, routines []CommandString) CommandString {
		if cs2.Equals(cs) {
			return pattern
		}

		if len(cs2) > len(cs) {
			fmt.Printf("Too big.  %v  %v\n", pattern, cs2.String())
			return nil
		}

		for i := 0; i < len(routines); i++ {
			if len(cs2)+len(routines[i]) > len(cs) {
				continue
			}

			s := len(cs2)
			e := s + len(routines[i])
			if cs[s:e].Equals(routines[i]) {
				newPattern := pattern.clone()
				newPattern = append(newPattern, int64('A'+i))
				if len(newPattern.String()) > 20 {
					continue
				}
				newCS := cs[:e]
				if goodPattern := findPattern(newPattern, newCS, routines); goodPattern != nil {
					return goodPattern
				}
			}
		}

		return nil
	}

	var dictionary []CommandString
	for s := 0; s < len(cs); s++ {
		for l := 0; l < 20; l++ {
			r := cs.subCommand(s, l)
			if r != nil {
				found := false
				for _, r2 := range dictionary {
					if r2.Equals(r) {
						found = true
						break
					}
				}
				if !found {
					dictionary = append(dictionary, r)
				}
			}
		}
	}

	sort.Slice(dictionary, func(i, j int) bool {
		if len(dictionary[i]) != len(dictionary[j]) {
			return len(dictionary[i]) > len(dictionary[j])
		}

		return dictionary[i].String() < dictionary[j].String()
	})

	for a := 0; a < len(dictionary); a++ {
		var ra = dictionary[a]
		if !dictionary[a].Equals(cs[:len(dictionary[a])]) {
			continue
		}
		for b := 0; b < len(dictionary); b++ {
			if a == b {
				continue
			}
			var rb = dictionary[b]
			for c := 0; c < len(dictionary); c++ {
				if a == c || a == b {
					continue
				}
				var rc = dictionary[c]

				pattern := findPattern(CommandString{'A'}, ra, []CommandString{ra, rb, rc})
				if pattern != nil {
					return []CommandString{pattern, ra, rb, rc}
				}
			}
		}
	}

	return nil
}
