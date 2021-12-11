package main

import "testing"

func TestIntcode(t *testing.T) {
	type TestCase struct {
		program       []int64
		expectedValue int64
		expectedAt    int64
	}
	var testCases = []*TestCase{
		&TestCase{[]int64{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}, 3500, 0},
		&TestCase{[]int64{1101, 100, -1, 4, 0}, 99, 4},
	}

	for _, testCase := range testCases {
		ic := newIntcode(testCase.program)
		err := ic.run()
		if err != nil {
			t.Errorf("Error running program %v: %v", testCase.program, err)
		} else if ic.memory[testCase.expectedAt] != testCase.expectedValue {
			t.Errorf("Program:%v  Result:%v  Expected:%v  Got:%v", testCase.program, ic.memory, testCase.expectedValue, ic.memory[testCase.expectedAt])
		} else {
			t.Logf("Program:%v  Result:%v  Expected:%v  Got:%v", testCase.program, ic.memory, testCase.expectedValue, ic.memory[testCase.expectedAt])
		}
	}
}
