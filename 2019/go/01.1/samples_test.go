package main

import "testing"

func TestSamples(t *testing.T) {
	type Sample struct {
		mass     int64
		expected int64
	}
	var samples = []*Sample{
		&Sample{12, 2},
		&Sample{14, 2},
		&Sample{1969, 654},
		&Sample{100756, 33583},
	}

	for _, sample := range samples {
		got := calcFuel(sample.mass)
		if got != sample.expected {
			t.Errorf("Mass:%v  Expected:%v  Got:%v", sample.mass, sample.expected, got)
		} else {
			t.Logf("Mass:%v  Expected:%v  Got:%v", sample.mass, sample.expected, got)
		}
	}
}
