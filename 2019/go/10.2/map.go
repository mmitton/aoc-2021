package main

import (
	"bufio"
	"fmt"
	"io"
	"math"
	"sort"
)

type Map struct {
	blocks [][]*MapBlock
}

type MapBlock struct {
	hasAsteroid bool
}

type MapBlockInfo struct {
	totalSee         int
	orderedAsteroids []*Asteroid
}

type Asteroid struct {
	x, y       int
	canSee     bool
	blockCount int
	angle      float64
}

func decodeMap(data []string) (*Map, error) {
	m := &Map{}
	for i, line := range data {
		var row []*MapBlock
		for _, r := range line {
			b := &MapBlock{}
			if r != '.' {
				b.hasAsteroid = true
			}
			row = append(row, b)
		}

		if m.blocks != nil {
			if len(m.blocks[0]) != len(row) {
				return nil, fmt.Errorf("Row %v has %v cells, expected %v", i, len(row), len(m.blocks[0]))
			}
		}
		m.blocks = append(m.blocks, row)
	}

	return m, nil
}

func decodeMapFromReader(r io.Reader) (*Map, error) {
	s := bufio.NewScanner(r)
	var data []string
	for s.Scan() {
		data = append(data, s.Text())
	}
	return decodeMap(data)
}

func (m *Map) findBestBase() (max int, x int, y int, orderedAsteroids []*Asteroid) {
	numbers := m.calculateSeeNumbers()
	for Y, row := range numbers {
		for X, bi := range row {
			if bi.totalSee > max {
				max, x, y, orderedAsteroids = bi.totalSee, X, Y, bi.orderedAsteroids
			}
		}
	}

	return
}

func (m *Map) calculateSeeNumbers() [][]*MapBlockInfo {
	width, height := len(m.blocks[0]), len(m.blocks)

	findAngle := func(x1, y1, x2, y2 int) float64 {
		if x1 == x2 {
			if y2 > y1 {
				return math.Pi
			}
			return 0
		}
		angle := float64(0)
		if x1 != x2 {
			dx := float64(x2 - x1)
			dy := float64(y2 - y1)
			angle = math.Atan(dy/dx) + (math.Pi / 2)
		}
		if x2 < x1 {
			angle += math.Pi
		}
		return angle
	}

	getAsteroids := func(x, y int) []*Asteroid {
		var ret []*Asteroid
		for y1 := 0; y1 < height; y1++ {
			for x1 := 0; x1 < width; x1++ {
				block := m.blocks[y1][x1]
				if block.hasAsteroid && !(y1 == y && x1 == x) {
					asteroid := &Asteroid{
						x:          x1,
						y:          y1,
						blockCount: 0,
						angle:      findAngle(x, y, x1, y1),
						canSee:     true,
					}
					ret = append(ret, asteroid)
				}
			}
		}
		return ret
	}

	gcd := func(a, b int) int {
		if a < 0 {
			a = -a
		}
		if b < 0 {
			b = -b
		}
		for b != 0 {
			t := b
			b = a % b
			a = t
		}
		return a
	}

	var ret [][]*MapBlockInfo
	for y1 := 0; y1 < height; y1++ {
		var seeRow []*MapBlockInfo
		for x1 := 0; x1 < width; x1++ {
			totalSee := 0
			var asteroids []*Asteroid = nil

			if m.blocks[y1][x1].hasAsteroid {
				asteroids = getAsteroids(x1, y1)

				for _, a := range asteroids {
					dx, dy := a.x-x1, a.y-y1
					if dx == 0 {
						if dy < 0 {
							dy = -1
						} else {
							dy = 1
						}
					} else if dy == 0 {
						if dx < 0 {
							dx = -1
						} else {
							dx = 1
						}
					} else {
						gcd := gcd(dx, dy)
						dx /= gcd
						dy /= gcd
					}

					x3, y3 := a.x+dx, a.y+dy
					for x3 >= 0 && x3 < width && y3 >= 0 && y3 < height {
						for _, a2 := range asteroids {
							if a2.x == x3 && a2.y == y3 {
								a2.canSee = false
								a2.blockCount++
							}
						}
						x3 += dx
						y3 += dy
					}
				}

				for _, a := range asteroids {
					if a.canSee {
						totalSee++
					}
				}

				sort.Slice(asteroids, func(i, j int) bool {
					a, b := asteroids[i], asteroids[j]
					if a.blockCount != b.blockCount {
						return a.blockCount < b.blockCount
					}

					return a.angle < b.angle
				})
			}

			seeRow = append(seeRow, &MapBlockInfo{totalSee: totalSee, orderedAsteroids: asteroids})
		}
		ret = append(ret, seeRow)
	}
	return ret
}
