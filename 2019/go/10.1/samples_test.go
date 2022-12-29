package main

import (
	"fmt"
	"testing"
)

func TestMapFindBest(t *testing.T) {
	type TestCase struct {
		data    []string
		bestX   int
		bestY   int
		bestSee int
		numbers [][]int
	}
	var testCases = []*TestCase{
		&TestCase{[]string{".#..#", ".....", "#####", "....#", "...##"}, 3, 4, 8, [][]int{[]int{0, 7, 0, 0, 7}, []int{0, 0, 0, 0, 0}, []int{6, 7, 7, 7, 5}, []int{0, 0, 0, 0, 7}, []int{0, 0, 0, 8, 7}}},
		&TestCase{[]string{"......#.#.", "#..#.#....", "..#######.", ".#.#.###..", ".#..#.....", "..#....#.#", "#..#....#.", ".##.#..###", "##...#..#.", ".#....####"}, 5, 8, 33, nil},
		&TestCase{[]string{"#.#...#.#.", ".###....#.", ".#....#...", "##.#.#.#.#", "....#.#.#.", ".##..###.#", "..#...##..", "..##....##", "......#...", ".####.###."}, 1, 2, 35, nil},
		&TestCase{[]string{".#..#..###", "####.###.#", "....###.#.", "..###.##.#", "##.##.#.#.", "....###..#", "..#.#..#.#", "#..#.#.###", ".##...##.#", ".....#.#.."}, 6, 3, 41, nil},
		&TestCase{[]string{".#..##.###...#######", "##.############..##.", ".#.######.########.#", ".###.#######.####.#.", "#####.##.#.##.###.##", "..#####..#.#########", "####################", "#.####....###.#.#.##", "##.#################", "#####.##.###..####..", "..######..##.#######", "####.##.####...##..#", ".#####..#.######.###", "##...#.##########...", "#.##########.#######", ".####.#.###.###.#.##", "....##.##.###..#####", ".#.#.###########.###", "#.#.#.#####.####.###", "###.##.####.##.#..##"}, 11, 13, 210, nil},
	}

	for _, testCase := range testCases {
		m, err := decodeMap(testCase.data)
		if err != nil {
			fmt.Errorf("ERROR: %v", err)
		} else {
			bestSee, bestX, bestY := m.findBestBase()
			if bestSee != testCase.bestSee || bestX != testCase.bestX || bestY != testCase.bestY {
				t.Errorf("ERROR: Best base not right.  Expected:%v (%v,%v)  Got:%v (%v,%v)\n", testCase.bestSee, testCase.bestX, testCase.bestY,
					bestSee, bestX, bestY)
			} else {
				t.Logf("Best base.  Expected:%v (%v,%v)  Got:%v (%v,%v)\n", testCase.bestSee, testCase.bestX, testCase.bestY,
					bestSee, bestX, bestY)
			}

			numbers := m.calculateSeeNumbers()
			t.Logf(" Numbers:%v", numbers)
			if len(testCase.numbers) > 0 {
				t.Logf("Expected:%v", testCase.numbers)
				if len(numbers) != len(testCase.numbers) {
					t.Errorf("ERROR: Expected %v rows in numbers, got %v", len(testCase.numbers), len(numbers))
				} else {
					for i := 0; i < len(numbers); i++ {
						if len(numbers[i]) != len(testCase.numbers[i]) {
							t.Errorf("ERROR: Expected %v cells in row %v, got %v", len(testCase.numbers[i]), i, len(numbers[i]))
						} else {
							for j := 0; j < len(numbers[i]); j++ {
								if numbers[i][j] != testCase.numbers[i][j] {
									t.Errorf("ERROR: Mismatch at %v,%v.  Expected:%v  Got:%v", j, i, testCase.numbers[i][j], numbers[i][j])
								}
							}
						}
					}
				}
			}
		}
	}
}
