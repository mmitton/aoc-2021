package main

import "testing"

func TestOrbitChecksum(t *testing.T) {
	type TestCase struct {
		defs     []string
		expected int
	}
	var testCases = []*TestCase{
		&TestCase{[]string{"COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L"}, 42},
	}

	for _, testCase := range testCases {
		com, err := newOrbits(testCase.defs)
		if err != nil {
			t.Errorf("ERROR: %v", err)
		} else {
			got := com.calcChecksum()
			if got != testCase.expected {
				t.Errorf("ERROR: Def:%v  Expected:%v  Got:%v", testCase.defs, testCase.expected, got)
			} else {
				t.Logf("Def:%v  Expected:%v  Got:%v", testCase.defs, testCase.expected, got)
			}
		}
	}
}
