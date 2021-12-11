package main

import (
	"sort"
	"testing"
)

func TestFFT(t *testing.T) {
	type Snapshot struct {
		step   int
		output string
	}
	type TestCase struct {
		inputSignal string
		snapshots   []*Snapshot
	}

	var testCases = []*TestCase{
		&TestCase{"12345678", []*Snapshot{
			&Snapshot{1, "48226158"},
			&Snapshot{2, "34040438"},
			&Snapshot{3, "03415518"},
			&Snapshot{4, "01029498"},
		}},
		&TestCase{"80871224585914546619083218645595", []*Snapshot{&Snapshot{100, "24176176"}}},
		&TestCase{"19617804207202209144916044189917", []*Snapshot{&Snapshot{100, "73745418"}}},
		&TestCase{"69317163492948606335995924319873", []*Snapshot{&Snapshot{100, "52432133"}}},
	}

	for _, testCase := range testCases {
		t.Run(testCase.inputSignal, func(t *testing.T) {
			t.Logf("Running test for %v", testCase.inputSignal)

			sort.Slice(testCase.snapshots, func(i, j int) bool {
				return testCase.snapshots[i].step < testCase.snapshots[j].step
			})

			for _, snapshot := range testCase.snapshots {
				fft, err := newFFT(testCase.inputSignal)
				if err != nil {
					t.Errorf("ERROR: %v", err)
				} else {
					fft.runNSteps(snapshot.step)
					got := fft.output[:len(snapshot.output)]
					if snapshot.output != got {
						t.Errorf("ERROR: After %v steps.  Expected:%v  Got:%v", snapshot.step, snapshot.output, got)
					} else {
						t.Logf("After %v steps.  Expected:%v  Got:%v", snapshot.step, snapshot.output, got)
					}
				}
			}
		})
	}
}
