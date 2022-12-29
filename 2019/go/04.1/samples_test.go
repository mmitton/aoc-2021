package main

import "testing"

func TestSamples(t *testing.T) {
	type TestCase struct {
		val      int64
		expected bool
	}
	var testCases = []*TestCase{
		&TestCase{111111, true},
		&TestCase{223450, false},
		&TestCase{123789, false},
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
