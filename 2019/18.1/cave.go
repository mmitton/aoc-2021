package main

import (
	"errors"
	"fmt"
	"sort"
	"strings"
	"unicode"
)

type Cave struct {
	tiles    [][]*Tile
	starting *Tile
	keys     []*Tile
	doors    []*Tile
	nodes    []*Node
}

type Tile struct {
	x, y      int
	isWall    bool
	isPassage bool
	isStart   bool
	isNode    bool
	key       rune
	door      rune
	doorAt    *Tile
	keyAt     *Tile
	neighbors []*Tile
	node      *Node
}

type Node struct {
	t     *Tile
	paths []*NodePath
}

type NodePath struct {
	from       *Tile
	to         *Tile
	steps      int
	keysNeeded []*Tile
	keysOnPath []*Tile
}

func (t *Tile) String() string {
	if t.door != 0 {
		return fmt.Sprintf("{%v,%v Door %c}", t.x, t.y, t.door)
	}
	if t.key != 0 {
		return fmt.Sprintf("{%v,%v Key %c}", t.x, t.y, t.key)
	}
	if t.isStart {
		return fmt.Sprintf("{%v,%v Start}", t.x, t.y)
	}
	if t.isPassage {
		return fmt.Sprintf("{%v,%v Passage}", t.x, t.y)
	}
	if t.isWall {
		return fmt.Sprintf("{%v,%v Wall}", t.x, t.y)
	}

	return fmt.Sprintf("{%v,%v}", t.x, t.y)
}

type TileList []*Tile

func (tl TileList) Contains(t *Tile) bool {
	for _, t2 := range tl {
		if t2 == t {
			return true
		}
	}

	return false
}

func (tl TileList) clone() TileList {
	var tl2 TileList
	tl2 = append(tl2, tl...)
	return tl2
}

func newCave(caveMap []string) (*Cave, error) {
	if len(caveMap) < 3 {
		return nil, fmt.Errorf("Too few lines.  expected at least 3, got %v", len(caveMap))
	}

	var allPassages []*Tile
	c := &Cave{}
	for i, l := range caveMap {
		if len(l) != len(caveMap[0]) {
			return nil, fmt.Errorf("Line %v isn't the correct size.  Expected %v, got %v", i+1, len(caveMap[0]), len(l))
		}

		var row []*Tile
		for j, v := range l {
			var tile = &Tile{x: j, y: i}
			if v == '#' {
				tile.isWall = true
			} else {
				if v == '.' {
					tile.isPassage = true
				} else if v >= 'a' && v <= 'z' {
					tile.isPassage = true
					tile.key = v
					c.keys = append(c.keys, tile)
				} else if v >= 'A' && v <= 'Z' {
					tile.isPassage = true
					tile.door = v
					c.doors = append(c.doors, tile)
				} else if v == '@' {
					tile.isPassage = true
					tile.isStart = true
					c.starting = tile
				}
			}
			if tile.isPassage {
				allPassages = append(allPassages, tile)
			}
			row = append(row, tile)
		}
		c.tiles = append(c.tiles, row)
	}

	for _, k := range c.keys {
		for _, d := range c.doors {
			if d.door == unicode.ToUpper(k.key) {
				k.doorAt = d
				d.keyAt = k
				break
			}
		}
	}

	var allNodes TileList
	for _, t := range allPassages {
		if t2 := c.GetTile(t.x-1, t.y); t2 != nil && !t2.isWall {
			t.neighbors = append(t.neighbors, t2)
		}
		if t2 := c.GetTile(t.x+1, t.y); t2 != nil && !t2.isWall {
			t.neighbors = append(t.neighbors, t2)
		}
		if t2 := c.GetTile(t.x, t.y-1); t2 != nil && !t2.isWall {
			t.neighbors = append(t.neighbors, t2)
		}
		if t2 := c.GetTile(t.x, t.y+1); t2 != nil && !t2.isWall {
			t.neighbors = append(t.neighbors, t2)
		}

		if t.key != 0 || t.isStart {
			t.isNode = !t.isStart
			allNodes = append(allNodes, t)
		}
	}

	for _, t := range allNodes {
		seen := make(map[*Tile]bool)
		seen[t] = true

		t.node = &Node{t: t}
		c.nodes = append(c.nodes, t.node)

		var tiles = []*NodePath{&NodePath{t, t, 0, nil, nil}}
		for len(tiles) > 0 {
			t2 := tiles[0]
			tiles = tiles[1:]

			if t2.to.key != 0 && t2.to != t {
				t.node.paths = append(t.node.paths, t2)
			}

			for _, t3 := range t2.to.neighbors {
				if !seen[t3] {
					seen[t3] = true
					keysNeeded := t2.keysNeeded
					keysOnPath := t2.keysOnPath

					if t3.key != 0 {
						keysOnPath = nil
						keysOnPath = append(keysOnPath, t2.keysOnPath...)
						keysOnPath = append(keysOnPath, t3)
					}
					if t3.door != 0 {
						keysNeeded = nil
						keysNeeded = append(keysNeeded, t2.keysNeeded...)
						keysNeeded = append(keysNeeded, t3.keyAt)
					}

					tiles = append(tiles, &NodePath{t, t3, t2.steps + 1, keysNeeded, keysOnPath})
				}
			}
		}

		// fmt.Printf("paths from %v: %v\n", t, len(t.node.paths))
		// for _, p := range t.node.paths {
		// 	fmt.Printf("\tPath from %v to %v.  Steps:%v  Keys Needed:%v  Keys On Path:%v\n", p.from, p.to, p.steps, p.keysNeeded, p.keysOnPath)
		// }
	}

	// fmt.Printf("%v total nodes\n", len(c.nodes))

	return c, nil
}

func (c *Cave) OutputCave() {
	for _, row := range c.tiles {
		for _, t := range row {
			if t.isWall {
				fmt.Print("#")
			} else if t.key != 0 {
				fmt.Printf("%c", t.key)
			} else if t.door != 0 {
				fmt.Printf("%c", t.door)
			} else if t.isStart {
				fmt.Print("@")
			} else if t.isPassage {
				fmt.Print(".")
			} else {
				fmt.Print(" ")
			}
		}
		fmt.Println()
	}
}

func (c *Cave) GetTile(x, y int) *Tile {
	if y < 0 || y >= len(c.tiles) {
		return nil
	}
	row := c.tiles[y]
	if x < 0 || x >= len(row) {
		return nil
	}
	return row[x]
}

type Path struct {
	steps     int
	start     *Tile
	at        *Tile
	keys      TileList
	doors     TileList
	nodePaths []*NodePath
}

func (p *Path) newPath(np *NodePath) *Path {
	p2 := &Path{}
	p2.steps = p.steps + np.steps
	p2.start = p.start
	p2.at = np.to

	p2.keys = append(p2.keys, p.keys...)
	p2.doors = append(p2.doors, p.doors...)

	if np.to.key != 0 && !p2.keys.Contains(np.to) {
		p2.keys = append(p2.keys, np.to)
	}
	if np.to.doorAt != nil && !p2.doors.Contains(np.to.doorAt) {
		p2.doors = append(p2.doors, np.to.doorAt)
	}

	p2.nodePaths = append(p2.nodePaths, p.nodePaths...)
	p2.nodePaths = append(p2.nodePaths, np)

	return p2
}

func (p *Path) Keys() string {
	var ret []string
	for _, k := range p.keys {
		ret = append(ret, string(k.key))
	}

	return strings.Join(ret, ", ")
}

func (p *Path) String() string {
	var nodes []string
	nodes = append(nodes, p.start.String())
	for _, np := range p.nodePaths {
		nodes = append(nodes, np.to.String())
	}
	return fmt.Sprintf("Path{%v steps, [%v]}", p.steps, strings.Join(nodes, ", "))
}

func (p *Path) findNext(c *Cave, shortestDistance int) []*Path {
	var ret []*Path

	for _, next := range p.at.node.paths {
		if p.keys.Contains(next.to) {
			continue
		}

		if shortestDistance > 0 && shortestDistance < p.steps+next.steps {
			continue
		}
		hasKeys := true
		for _, key := range next.keysNeeded {
			if !p.keys.Contains(key) {
				hasKeys = false
			}
		}

		for _, key := range next.keysOnPath {
			if key == next.to {
				continue
			}
			if !p.keys.Contains(key) {
				hasKeys = false
			}
		}

		if hasKeys {
			ret = append(ret, p.newPath(next))
		}
	}

	sort.Slice(ret, func(i, j int) bool {
		return ret[i].steps > ret[j].steps
	})

	// fmt.Printf("Found %v paths from %v: Existing keys:%v  Found keys:%v\n", len(ret), p.at, p.Keys(), keysFound)
	return ret
}

func (c *Cave) FindShortestPaths() ([]*Path, error) {
	var workingPaths = []*Path{&Path{start: c.starting, at: c.starting}}
	var completedPaths []*Path
	var shortestDistance int

	considered := 0
	for len(workingPaths) > 0 {
		considered++
		p := workingPaths[len(workingPaths)-1]
		workingPaths = workingPaths[:len(workingPaths)-1]
		if considered%100000 == 0 {
			fmt.Printf("considered:%v  shortestDistance:%v  len(completedPaths):%v  len(keys):%v  steps:%v  total:%v  len(workingPaths):%v\n", considered, shortestDistance, len(completedPaths), len(p.keys), p.steps, len(c.keys), len(workingPaths))
		}

		if shortestDistance > 0 && p.steps >= shortestDistance {
			continue
		}

		for _, p := range p.findNext(c, shortestDistance) {
			if shortestDistance > 0 && p.steps > shortestDistance {
				continue
			}
			if len(p.keys) == len(c.keys) {
				printMessage := false
				if shortestDistance == 0 || p.steps < shortestDistance {
					// found shorter, reset
					completedPaths = nil
					printMessage = true
				}
				shortestDistance = p.steps
				completedPaths = append(completedPaths, p)
				if printMessage {
					fmt.Printf("considered:%v  shortestDistance:%v  len(completedPaths):%v  len(keys):%v  steps:%v  total:%v  len(workingPaths):%v\n", considered, shortestDistance, len(completedPaths), len(p.keys), p.steps, len(c.keys), len(workingPaths))
				}
				continue
			}

			workingPaths = append(workingPaths, p)
		}
	}

	if len(completedPaths) == 0 {
		return nil, errors.New("No paths found")
	}

	fmt.Printf("Found %v total paths at distance %v.  considered:%v\n", len(completedPaths), completedPaths[0].steps, considered)
	return completedPaths, nil
}
