package main

import (
	"bufio"
	"fmt"
	"io"
)

type Map struct {
	blocks [][]*MapBlock
}

type MapBlock struct {
	label       string
	hasAsteroid bool
	canSee      bool
}

func decodeMap(data []string) (*Map, error) {
	m := &Map{}
	for i, line := range data {
		var row []*MapBlock
		for _, r := range line {
			b := &MapBlock{}
			if r != '.' {
				b.hasAsteroid = true
				b.label = string(r)
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

func (m *Map) findBestBase() (max int, x int, y int) {
	numbers := m.calculateSeeNumbers()
	max, maxX, maxY := 0, 0, 0
	for y, row := range numbers {
		for x, cell := range row {
			if cell > max {
				max, maxX, maxY = cell, x, y
			}
		}
	}

	return max, maxX, maxY
}

func (m *Map) calculateSeeNumbers() [][]int {
	width, height := len(m.blocks[0]), len(m.blocks)

	resetCanSee := func(x, y int) {
		for y1 := 0; y1 < height; y1++ {
			for x1 := 0; x1 < width; x1++ {
				if y1 == y && x1 == x {
					m.blocks[y1][x1].canSee = false
				} else {
					m.blocks[y1][x1].canSee = true
				}
			}
		}
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

	var ret [][]int
	for y1 := 0; y1 < height; y1++ {
		var seeRow []int
		for x1 := 0; x1 < width; x1++ {
			totalSee := 0

			if m.blocks[y1][x1].hasAsteroid {
				resetCanSee(x1, y1)

				for y2 := 0; y2 < height; y2++ {
					for x2 := 0; x2 < width; x2++ {
						if x1 == x2 && y1 == y2 {
							continue
						}

						if m.blocks[y2][x2].hasAsteroid {
							dx, dy := x2-x1, y2-y1
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

							x3, y3 := x2+dx, y2+dy
							for x3 >= 0 && x3 < width && y3 >= 0 && y3 < height {
								m.blocks[y3][x3].canSee = false
								x3 += dx
								y3 += dy
							}
						}
					}
				}

				for y2 := 0; y2 < height; y2++ {
					for x2 := 0; x2 < width; x2++ {
						if m.blocks[y2][x2].hasAsteroid && m.blocks[y2][x2].canSee {
							totalSee++
						}
					}
				}
			}

			seeRow = append(seeRow, totalSee)
		}
		ret = append(ret, seeRow)
	}
	return ret
}
