package main

import "testing"

func TestRelativeMode(t *testing.T) {
	type TestCase struct {
		program []int64
		output  []int64
	}
	var testCases = []*TestCase{
		&TestCase{[]int64{109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99}, []int64{109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99}},
		&TestCase{[]int64{1102, 34915192, 34915192, 7, 4, 7, 99, 0}, []int64{34915192 * 34915192}},
		&TestCase{[]int64{104, 1125899906842624, 99}, []int64{1125899906842624}},
	}

	for _, testCase := range testCases {
		ic := newIntcode(testCase.program)
		var output []int64
		ic.Output = func(v int64) {
			output = append(output, v)
		}
		if err := ic.run(); err != nil {
			t.Errorf("ERROR: %v", err)
		} else {
			t.Logf("Expected %v, Got %v\n", testCase.output, output)
			if len(testCase.output) != len(output) {
				t.Errorf("ERROR: Expected %v output values, Got %v\n", len(testCase.output), len(output))
			} else {
				for i := 0; i < len(output); i++ {
					if output[i] != testCase.output[i] {
						t.Errorf("ERROR: Mismatch at output %v.  Expected:%v  Got:%v\n", i, testCase.output[i], output[i])
					}
				}
			}
		}
	}
}
