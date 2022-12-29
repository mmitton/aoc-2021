package main

import (
	"fmt"
	"strings"
	"testing"
)

type TestCase struct {
	name     string
	numCards int
	commands []string
	result   string
}

var testCases = []*TestCase{
	&TestCase{"New Stack", 10, []string{"deal into new stack"}, "9 8 7 6 5 4 3 2 1 0"},
	&TestCase{"Cut 3", 10, []string{"cut 3"}, "3 4 5 6 7 8 9 0 1 2"},
	&TestCase{"Cut -4", 10, []string{"cut -4"}, "6 7 8 9 0 1 2 3 4 5"},
	&TestCase{"Increment 3", 10, []string{"deal with increment 3"}, "0 7 4 1 8 5 2 9 6 3"},
	&TestCase{"Multiple 1", 10, []string{"deal with increment 7", "deal into new stack", "deal into new stack"}, "0 3 6 9 2 5 8 1 4 7"},
	&TestCase{"Multiple 2", 10, []string{"cut 6", "deal with increment 7", "deal into new stack"}, "3 0 7 4 1 8 5 2 9 6"},
	&TestCase{"Multiple 3", 10, []string{"deal with increment 7", "deal with increment 9", "cut -2"}, "6 3 0 7 4 1 8 5 2 9"},
	&TestCase{"Multiple 4", 10, []string{"deal into new stack", "cut -2", "deal with increment 7", "cut 8", "cut -4", "deal with increment 7", "cut 3", "deal with increment 9", "deal with increment 3", "cut -1"}, "9 2 5 8 1 4 7 0 3 6"},
}

func TestShuffles(t *testing.T) {
	for _, testCase := range testCases {
		t.Run(testCase.name, func(t *testing.T) {
			deck, err := newDeck(testCase.numCards)
			if err != nil {
				t.Fatalf("ERROR: %v", err)
				return
			}

			for _, command := range testCase.commands {
				t.Log(command)
				if err := deck.Shuffle(command); err != nil {
					t.Fatalf("ERROR: %q: %v", command, err)
					return
				}
			}

			if deck.String() != testCase.result {
				t.Errorf("ERROR: Unexpected deck order.  Expected:%v  Got:%v", testCase.result, deck.String())
				return
			}
			t.Logf("Result: %v", deck.String())
		})
	}
}

func TestQuickDeck(t *testing.T) {
	for _, testCase := range testCases {
		t.Run(testCase.name, func(t *testing.T) {
			qd := newQuickDeck(testCase.numCards, testCase.commands)
			var gotArray = make([]string, testCase.numCards)
			for i := 0; i < testCase.numCards; i++ {
				newPos := qd.MapPosition(i)
				gotArray[newPos] = fmt.Sprint(i)
			}
			got := strings.Join(gotArray, " ")
			t.Logf("Expected: %v", testCase.result)
			t.Logf("     Got: %v", got)
			if got != testCase.result {
				t.Errorf("ERROR: Does not match")
				return
			}
		})
	}
}
