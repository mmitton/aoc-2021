package main

import (
	"strconv"
)

type QuickDeckFunc func(pos int) int
type QuickDeck []QuickDeckFunc

func (qd QuickDeck) MapPosition(pos int) int {
	for _, f := range qd {
		pos = f(pos)
	}
	return pos
}

func newQuickDeck(numCards int, commands []string) QuickDeck {
	var qd QuickDeck
	for _, command := range commands {
		if command == "deal into new stack" {
			qd = append(qd, func(pos int) int {
				return numCards - pos - 1
			})
		} else if command[:4] == "cut " {
			cut, err := strconv.ParseInt(command[4:], 10, 64)
			if err != nil {
				panic(err)
			}
			if cut < 0 {
				cut = int64(numCards) + cut
			}
			qd = append(qd, func(pos int) int {
				return (numCards + pos - int(cut)) % numCards
			})
		} else if command[:20] == "deal with increment " {
			inc, err := strconv.ParseInt(command[20:], 10, 64)
			if err != nil {
				panic(err)
			}
			qd = append(qd, func(pos int) int {
				return (pos * int(inc)) % numCards
			})
		}
	}
	return qd
}
