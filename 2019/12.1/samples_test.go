package main

import (
	"strconv"
	"strings"
	"testing"
)

func TestMoons(t *testing.T) {
	type TestCase struct {
		defs        []string
		steps       int
		totalEnergy int64
	}

	var testCases = []*TestCase{
		&TestCase{[]string{"<x=-1, y=0, z=2>", "<x=2, y=-10, z=-7>", "<x=4, y=-8, z=8>", "<x=3, y=5, z=-1>"}, 10, 179},
		&TestCase{[]string{"<x=-8, y=-10, z=0>", "<x=5, y=5, z=10>", "<x=2, y=-7, z=3>", "<x=9, y=-8, z=-3>"}, 100, 1940},
	}
	abs := func(v int64) int64 {
		if v < 0 {
			return -v
		}
		return v
	}

	for _, testCase := range testCases {
		var moons Moons
		dumpPosVel := func(after int) {
			t.Logf("After %v steps:", after)
			for _, m := range moons {
				t.Log(m)
			}
		}

		for _, def := range testCase.defs {
			m, err := newMoon(def)
			if err != nil {
				t.Errorf("ERROR: %v => %v", def, err)
			}
			moons = append(moons, m)
		}

		dumpPosVel(0)
		for i := 1; i <= testCase.steps; i++ {
			moons.Step()
			dumpPosVel(i)
		}

		t.Logf("Energy after %v steps:", testCase.steps)
		var sums []string
		var total int64
		for _, m := range moons {
			pot, kin, energy := m.Energy()

			t.Logf("pot: %2v + %2v + %2v = %2v;  kin: %2v + %2v + %2v = %2v;  total: %2v * %2v = %3v",
				abs(m.pos.x), abs(m.pos.y), abs(m.pos.z), pot,
				abs(m.vel.x), abs(m.vel.y), abs(m.vel.z), kin,
				pot, kin, energy)

			sums = append(sums, strconv.FormatInt(total, 10))
			total += energy
		}
		t.Logf("Sum of total energy: %v = %v", strings.Join(sums, " + "), total)

		total = moons.TotalEnergy()
		if total != testCase.totalEnergy {
			t.Errorf("ERROR: Unexpected total energy.  Expected:%v  Got:%v", testCase.totalEnergy, total)
		}
	}
}
