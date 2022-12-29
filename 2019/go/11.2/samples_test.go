package main

import "testing"

func TestPainter(t *testing.T) {
	type Step struct {
		expectedColor int64
		setColor      int64
		turn          int64
		state         []string
	}

	type TestCase struct {
		minX, minY int
		maxX, maxY int
		steps      []*Step
		painted    int
	}

	var testCases = []*TestCase{
		&TestCase{-2, -2, 2, 2,
			[]*Step{
				&Step{1, 1, 0, []string{".....", ".....", ".<#..", ".....", "....."}},
				&Step{0, 0, 0, []string{".....", ".....", "..#..", ".v...", "....."}},
				&Step{-1, 1, 0, []string{}},
				&Step{-1, 1, 0, []string{".....", ".....", "..^..", ".##..", "....."}},
				&Step{-1, 0, 1, []string{}},
				&Step{-1, 1, 0, []string{}},
				&Step{-1, 1, 0, []string{".....", "..<#.", "...#.", ".##..", "....."}}},
			6},
	}

	for _, testCase := range testCases {
		p := newPainter(nil)
		for stepIdx, step := range testCase.steps {
			if step.expectedColor == 0 || step.expectedColor == 1 {
				gotColor := p.Input()
				if gotColor != step.expectedColor {
					t.Errorf("ERROR: Got unexpected color at step %v.  Expected:%v  Got:%v", stepIdx, step.expectedColor, gotColor)
				}
			}
			p.Output(step.setColor)
			newColor := p.Input()
			if newColor != step.setColor {
				t.Errorf("ERROR: Got unexpected color at step %v after setting color.  Expected:%v  Got:%v", stepIdx, step.setColor, newColor)
			}
			p.Output(step.turn)

			output := p.generateOutput(testCase.minX, testCase.minY, testCase.maxX, testCase.maxY, true)
			badOutput := false
			t.Logf("Output after step %v", stepIdx)
			for idx, line := range output {
				if idx < len(step.state) && step.state[idx] != line {
					badOutput = true
				}
				t.Log(line)
			}

			if badOutput {
				t.Errorf("ERROR: Unexpected output at step %v.  Expected:", stepIdx)
				for _, line := range step.state {
					t.Error(line)
				}
			}
		}

		numberPainted := p.numberPainted()
		if numberPainted != testCase.painted {
			t.Errorf("ERROR: Unexpected number of panels painted.  Expected:%v  Got:%v", testCase.painted, numberPainted)
		} else {
			t.Logf("Number of panels painted:%v", numberPainted)
		}
	}
}
