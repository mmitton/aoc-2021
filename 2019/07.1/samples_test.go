package main

import "testing"

func TestAmplifierCircuit(t *testing.T) {
	type TestCase struct {
		program  []int64
		settings []int64
		expected int64
	}
	var testCases = []*TestCase{
		&TestCase{[]int64{3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0}, []int64{4, 3, 2, 1, 0}, 43210},
		&TestCase{[]int64{3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0}, []int64{0, 1, 2, 3, 4}, 54321},
		&TestCase{[]int64{3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0}, []int64{1, 0, 4, 3, 2}, 65210},
	}

	for _, testCase := range testCases {
		got := amplifierCircuit(testCase.program, testCase.settings...)
		if got != testCase.expected {
			t.Errorf("ERROR: Expected %v, Got %v\n", testCase.expected, got)
		} else {
			t.Logf("Expected %v, Got %v\n", testCase.expected, got)
		}
	}
}
