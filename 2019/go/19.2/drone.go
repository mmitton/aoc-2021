package main

import (
	"errors"
	"fmt"
)

type Drone struct {
	program    []int64
	input      []int64
	pulledAt   [][]bool
	lastOutput int64
}

func newDrone(program []int64) *Drone {
	d := &Drone{program: program}

	return d
}

func (d *Drone) PulledAt(x, y int64) (bool, error) {
	var lastOutput int64
	input := []int64{x, y}
	ic := newIntcode(d.program)
	ic.Input = func() int64 {
		if len(input) > 0 {
			ret := input[0]
			input = input[1:]
			return ret
		}
		return 0
	}

	ic.Output = func(v int64) {
		lastOutput = v
	}
	if err := ic.run(); err != nil {
		return false, err
	}
	return lastOutput == 1, nil
}

func (d *Drone) FindSquare(width, height int64) (int64, int64, error) {
	x := width
	sy := x
	for {
		x++
		for {
			d, err := d.PulledAt(x, sy)
			if err != nil {
				return 0, 0, err
			}
			if d {
				break
			}
			sy++
		}

		y := sy
		for {
			fmt.Printf("x:%v  sy:%v  y:%v\n", x, sy, y)

			c4, err := d.PulledAt(x, y+height-1)
			if err != nil {
				return 0, 0, err
			}
			if !c4 {
				break
			}

			c2, err := d.PulledAt(x+width-1, y)
			if err != nil {
				return 0, 0, err
			}
			c3, err := d.PulledAt(x+width-1, y+height-1)
			if err != nil {
				return 0, 0, err
			}

			if c2 && c3 && c4 {
				return x, y, nil
			}
			y++
		}
	}

	return 0, 0, errors.New("Not Found")
}
