package main

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
