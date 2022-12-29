package main

import (
	"errors"
	"fmt"
	"unicode"
)

type Maze []*Node

type Node struct {
	x, y      int
	outside   bool
	label     string
	neighbors []*Node
}

func newMaze(def []string) (Maze, error) {
	if len(def) < 5 {
		return nil, errors.New("Not enough rows")
	}

	var m Maze

	getLabel := func(x1, y1, x2, y2 int) string {
		c1 := rune(def[y1][x1])
		c2 := rune(def[y2][x2])

		if unicode.IsLetter(c1) && unicode.IsLetter(c2) {
			return fmt.Sprintf("%c%c", c1, c2)
		}
		return ""
	}

	for y := 0; y < len(def); y++ {
		if len(def[y]) != len(def[0]) {
			return nil, fmt.Errorf("Row %v doesn't have enough width. Expected %v, got %v", y+1, len(def[0]), len(def[y]))
		}

		for x := 0; x < len(def[y]); x++ {
			c := rune(def[y][x])
			if c == '.' {
				n := &Node{x, y, false, "", nil}
				m = append(m, n)

				if label := getLabel(x-2, y, x-1, y); label != "" {
					n.label = label
				} else if label := getLabel(x+1, y, x+2, y); label != "" {
					n.label = label
				} else if label := getLabel(x, y-2, x, y-1); label != "" {
					n.label = label
				} else if label := getLabel(x, y+1, x, y+2); label != "" {
					n.label = label
				}

				if n.label != "" {
					if y == 2 || y == len(def)-3 || x == 2 || x == len(def[y])-3 {
						n.outside = true
					}
				}
			}
		}
	}

	for _, n := range m {
		if n2 := m.FindNodeXY(n.x-1, n.y); n2 != nil {
			n.neighbors = append(n.neighbors, n2)
		}
		if n2 := m.FindNodeXY(n.x+1, n.y); n2 != nil {
			n.neighbors = append(n.neighbors, n2)
		}
		if n2 := m.FindNodeXY(n.x, n.y-1); n2 != nil {
			n.neighbors = append(n.neighbors, n2)
		}
		if n2 := m.FindNodeXY(n.x, n.y+1); n2 != nil {
			n.neighbors = append(n.neighbors, n2)
		}

		if n2 := m.FindWarpExit(n); n2 != nil {
			n.neighbors = append(n.neighbors, n2)
		}
	}

	return m, nil
}

func (m Maze) FindNodeXY(x, y int) *Node {
	for _, n := range m {
		if n.x == x && n.y == y {
			return n
		}
	}
	return nil
}

func (m Maze) FindWarpExit(n *Node) *Node {
	if n.label == "" {
		return nil
	}
	for _, n2 := range m {
		if n2.label == n.label && n2 != n {
			return n2
		}
	}
	return nil
}

func (m Maze) FindWarpPoints(label string) []*Node {
	if label == "" {
		return nil
	}
	var ret []*Node
	for _, n := range m {
		if n.label == label {
			ret = append(ret, n)
		}
	}

	return ret
}

func (m Maze) FindShortestPath(from, to string) (int, error) {
	froms := m.FindWarpPoints(from)
	if len(froms) != 1 {
		return 0, fmt.Errorf("Cannot find single warp point %v", from)
	}
	tos := m.FindWarpPoints(to)
	if len(tos) != 1 {
		return 0, fmt.Errorf("Cannot find single warp point %v", to)
	}

	return m.findPathFromTo(froms[0], tos[0])
}

func (m Maze) findPathFromTo(from, to *Node) (int, error) {
	seen := []map[*Node]bool{make(map[*Node]bool)}
	seen[0][from] = true

	type NodeSteps struct {
		n     *Node
		steps int
		level int
	}

	work := []*NodeSteps{&NodeSteps{from, 0, 0}}
	for len(work) > 0 {
		ns := work[0]
		work = work[1:]

		for _, next := range ns.n.neighbors {
			if next == to && ns.level == 0 {
				return ns.steps + 1, nil
			}

			level := ns.level
			if next.label != "" && next.label == ns.n.label {
				if ns.n.outside {
					level--
				} else {
					level++
				}
				if level < 0 {
					continue
				}
			}

			if level >= len(seen) {
				seen = append(seen, make(map[*Node]bool))
			}
			if seen[level][next] {
				continue
			}

			seen[level][next] = true
			work = append(work, &NodeSteps{next, ns.steps + 1, level})
		}
	}

	return 0, fmt.Errorf("Cannot find a path from %v to %v", from, to)
}

func (n *Node) String() string {
	if n.label != "" {
		return fmt.Sprintf("%v {%v,%v}", n.label, n.x, n.y)
	}
	return fmt.Sprintf("{%v,%v}", n.x, n.y)
}
