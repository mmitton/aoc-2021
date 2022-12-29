package main

import "testing"

func TestASCII(t *testing.T) {
	type TestCase struct {
		name            string
		input           []string
		expectedCommand string
		expectedSplit   []string
	}

	var testCases = []*TestCase{
		&TestCase{"Simple", []string{"..#..........", "..#..........", "#######...###", "#.#...#...#.#", "#############", "..#...#...#..", "..#####...^.."}, "", nil},
		&TestCase{"Bigger", []string{"#######...#####", "#.....#...#...#", "#.....#...#...#", "......#...#...#", "......#...###.#", "......#.....#.#", "^########...#.#", "......#.#...#.#", "......#########", "........#...#..", "....#########..", "....#...#......", "....#...#......", "....#...#......", "....#####......"}, "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2",
			[]string{"A,B,C,B,A,C", "R,8,R,8", "R,4,R,4,R,8", "L,6,L,2"}},
	}

	for _, testCase := range testCases {
		t.Run(testCase.name, func(t *testing.T) {
			ascii := newASCII(nil)
			for _, line := range testCase.input {
				for _, r := range line {
					ascii.Output(int64(r))
				}
				ascii.Output(10)
			}

			commandString := ascii.FindShortestPath()
			if testCase.expectedCommand != "" {
				if testCase.expectedCommand != commandString.String() {
					t.Errorf("Bad Command String")
					t.Errorf("Expected %v", testCase.expectedCommand)
					t.Errorf("     Got %v", commandString)
				} else {
					t.Logf("Good command! %v", commandString)
				}
			} else {
				t.Logf("Got %v", commandString)
			}

			splits := commandString.Split()
			t.Log("Got Split Commands")
			for _, c := range splits {
				t.Log(c)
			}
			if testCase.expectedSplit != nil {
				t.Log("Expected Split Commands")
				for _, c := range testCase.expectedSplit {
					t.Log(c)
				}
				if len(splits) != len(testCase.expectedSplit) {
					t.Errorf("ERROR:  Expected %v commands, got %v commands", len(testCase.expectedSplit), len(splits))
				} else {
					for i := 0; i < len(testCase.expectedSplit); i++ {
						if testCase.expectedSplit[i] != splits[i].String() {
							t.Errorf("ERROR: split %v doesn't match.  Expected:%v  Got:%v\n", i, testCase.expectedSplit[i], splits[i])
						}
					}
				}
			}
		})
	}
}
