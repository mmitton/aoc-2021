package main

import "testing"

func TestSteps(t *testing.T) {
	type TestCase struct {
		name             string
		caveMap          []string
		shortestPath     string
		shortestDistance int
	}

	var testCases = []*TestCase{
		&TestCase{"Sample 1", []string{"#########", "#b.A.@.a#", "#########"}, "a, b", 8},
		&TestCase{"Sample 2", []string{"########################", "#f.D.E.e.C.b.A.@.a.B.c.#", "######################.#", "#d.....................#", "########################"}, "a, b, c, d, e, f", 86},
		&TestCase{"Sample 3", []string{"########################", "#...............b.C.D.f#", "#.######################", "#.....@.a.B.c.d.A.e.F.g#", "########################"}, "b, a, c, d, f, e, g", 132},
		&TestCase{"Sample 4", []string{"#################", "#i.G..c...e..H.p#", "########.########", "#j.A..b...f..D.o#", "########@########", "#k.E..a...g..B.n#", "########.########", "#l.F..d...h..C.m#", "#################"}, "a, f, b, j, g, n, h, d, l, o, e, p, c, i, k, m", 136},
		&TestCase{"Sample 5", []string{"########################", "#@..............ac.GI.b#", "###d#e#f################", "###A#B#C################", "###g#h#i################", "########################"}, "a, c, f, i, d, g, b, e, h", 81},
	}

	for _, testCase := range testCases {
		t.Run(testCase.name, func(t *testing.T) {
			c, err := newCave(testCase.caveMap)
			if err != nil {
				t.Errorf("ERROR: %v", err)
				return
			}

			c.OutputCave()

			shortestPaths, err := c.FindShortestPaths()
			if err != nil {
				t.Errorf("ERROR: %v", err)
				return
			}

			t.Logf("Found %v paths with distance %v", len(shortestPaths), shortestPaths[0].steps)
			foundExpected := false
			for i, p := range shortestPaths {
				t.Logf("path[%v]: %v", i, p.Keys())
				if p.Keys() == testCase.shortestPath {
					foundExpected = true
				}
			}
			if shortestPaths[0].steps != testCase.shortestDistance {
				t.Errorf("ERROR: Expected shortest distance:%v  got:%v", testCase.shortestDistance, shortestPaths[0].steps)
				return
			}
			if !foundExpected {
				t.Errorf("ERROR: Expected to find path %v and did not", testCase.shortestPath)
				return
			}
		})
	}
}
