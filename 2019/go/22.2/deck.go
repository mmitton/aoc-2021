package main

import (
	"errors"
	"strconv"
	"strings"
)

type Deck struct {
	numCards int
	current  []int
	last     []int
}

func newDeck(numCards int) (*Deck, error) {
	if numCards < 1 {
		return nil, errors.New("Must have 1 or more cards in your deck")
	}

	d := &Deck{numCards: numCards}
	for i := 0; i < numCards; i++ {
		d.current = append(d.current, i)
		d.last = append(d.last, i)
	}

	return d, nil
}

func (d *Deck) newStack() {
	d.last, d.current = d.current, d.last
	for i := 0; i < d.numCards; i++ {
		d.current[i] = d.last[d.numCards-1-i]
	}
}

func (d *Deck) cutDeck(cut int) error {
	if cut < 0 {
		cut = d.numCards + cut
	}

	if cut >= d.numCards || cut < 0 {
		return errors.New("Bad cut number")
	}

	d.last, d.current = d.current, d.last
	for i := 0; i < d.numCards; i++ {
		d.current[i] = d.last[(i+cut)%d.numCards]
	}

	return nil
}

func (d *Deck) incrementShuffle(inc int) error {
	d.last, d.current = d.current, d.last
	for i := 0; i < d.numCards; i++ {
		d.current[i] = -1
	}

	pos := 0
	for i := 0; i < d.numCards; i++ {
		if d.current[pos] != -1 {
			return errors.New("Tried to double stack a card")
		}
		d.current[pos] = d.last[i]
		pos = (pos + inc) % d.numCards
	}

	return nil
}

func (d *Deck) Shuffle(command string) error {
	if command == "deal into new stack" {
		d.newStack()
		return nil
	} else if command[:4] == "cut " {
		cut, err := strconv.ParseInt(command[4:], 10, 64)
		if err != nil {
			return err
		}
		return d.cutDeck(int(cut))
	} else if command[:20] == "deal with increment " {
		inc, err := strconv.ParseInt(command[20:], 10, 64)
		if err != nil {
			return err
		}
		return d.incrementShuffle(int(inc))
	}
	return errors.New("Not Implemented")
}

func (d *Deck) String() string {
	var ret []string
	for _, c := range d.current {
		ret = append(ret, strconv.FormatInt(int64(c), 10))
	}
	return strings.Join(ret, " ")
}
