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

func TestCmpJmp(t *testing.T) {
	type TestCase struct {
		program        []int64
		input          int64
		expectedOutput int64
	}
	var testCases = []*TestCase{
		&TestCase{[]int64{3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8}, 7, 0},
		&TestCase{[]int64{3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8}, 8, 1},
		&TestCase{[]int64{3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8}, 9, 0},

		&TestCase{[]int64{3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8}, 7, 1},
		&TestCase{[]int64{3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8}, 8, 0},
		&TestCase{[]int64{3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8}, 9, 0},

		&TestCase{[]int64{3, 3, 1108, -1, 8, 3, 4, 3, 99}, 7, 0},
		&TestCase{[]int64{3, 3, 1108, -1, 8, 3, 4, 3, 99}, 8, 1},
		&TestCase{[]int64{3, 3, 1108, -1, 8, 3, 4, 3, 99}, 9, 0},

		&TestCase{[]int64{3, 3, 1107, -1, 8, 3, 4, 3, 99}, 7, 1},
		&TestCase{[]int64{3, 3, 1107, -1, 8, 3, 4, 3, 99}, 8, 0},
		&TestCase{[]int64{3, 3, 1107, -1, 8, 3, 4, 3, 99}, 9, 0},

		&TestCase{[]int64{3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9}, -1, 1},
		&TestCase{[]int64{3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9}, 0, 0},
		&TestCase{[]int64{3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9}, 1, 1},

		&TestCase{[]int64{3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1}, -1, 1},
		&TestCase{[]int64{3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1}, 0, 0},
		&TestCase{[]int64{3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1}, 1, 1},

		&TestCase{[]int64{3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99}, 7, 999},
		&TestCase{[]int64{3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99}, 8, 1000},
		&TestCase{[]int64{3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99}, 9, 1001},
	}

	for _, testCase := range testCases {
		ic := newIntcode(testCase.program)
		var lastOutput int64

		ic.Input = func() int64 {
			return testCase.input
		}
		ic.Output = func(v int64) {
			lastOutput = v
		}

		err := ic.run()
		if err != nil {
			t.Errorf("ERROR: %v: %v", testCase.program, err)
		} else if lastOutput != testCase.expectedOutput {
			t.Errorf("ERROR: Program:%v  Result:%v  Expected:%v  Got:%v", testCase.program, ic.memory, testCase.expectedOutput, lastOutput)
		} else {
			t.Logf("Program:%v  Result:%v  Expected:%v  Got:%v", testCase.program, ic.memory, testCase.expectedOutput, lastOutput)
		}
	}
}
