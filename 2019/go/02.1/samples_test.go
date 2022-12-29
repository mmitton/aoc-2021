package main

import "testing"

func TestIntcode(t *testing.T) {
	type Sample struct {
		program  []int64
		expected int64
	}
	var samples = []*Sample{
		&Sample{[]int64{1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50}, 3500},
	}

	for _, sample := range samples {
		ic := newIntcode(sample.program)
		err := ic.run()
		if err != nil {
			t.Errorf("Error running program %v: %v", sample.program, err)
		} else if ic.program[0] != sample.expected {
			t.Errorf("Program:%v  Result:%v  Expected:%v  Got:%v", sample.program, ic.program, sample.expected, ic.program[0])
		} else {
			t.Logf("Program:%v  Result:%v  Expected:%v  Got:%v", sample.program, ic.program, sample.expected, ic.program[0])
		}
	}
}
