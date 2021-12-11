package main

import "testing"

func TestSamples(t *testing.T) {
	type TestCase struct {
		val      int64
		expected bool
	}
	var testCases = []*TestCase{
		&TestCase{111111, false},
		&TestCase{223450, false},
		&TestCase{123789, false},
		&TestCase{112233, true},
		&TestCase{123444, false},
		&TestCase{111122, true},
	}

	for _, testCase := range testCases {
		got := isPossiblePassword(testCase.val)
		if got != testCase.expected {
			t.Errorf("ERROR: Password:%v  Expected:%v  Got:%v", testCase.val, testCase.expected, got)
		} else {
			t.Logf("Password:%v  Expected:%v  Got:%v", testCase.val, testCase.expected, got)
		}
	}
}
