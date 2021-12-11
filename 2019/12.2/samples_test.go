package main

import (
	"strconv"
	"strings"
	"testing"
)

func TestMoons(t *testing.T) {
	type TestCase struct {
		defs         []string
		steps        int
		totalEnergy  int64
		repeatsAfter int64
	}

	var testCases = []*TestCase{
		&TestCase{[]string{"<x=-1, y=0, z=2>", "<x=2, y=-10, z=-7>", "<x=4, y=-8, z=8>", "<x=3, y=5, z=-1>"}, 10, 179, 2772},
		&TestCase{[]string{"<x=-8, y=-10, z=0>", "<x=5, y=5, z=10>", "<x=2, y=-7, z=3>", "<x=9, y=-8, z=-3>"}, 100, 1940, 4686774924},
	}
	dumpPosVel := func(moons *Moons, after int) {
		t.Logf("After %v steps:", after)
		for _, m := range moons.moons {
			t.Log(m)
		}
	}

	for _, testCase := range testCases {
		moons, err := newMoons(testCase.defs)
		if err != nil {
			t.Errorf("ERROR: %v", err)
		}

		dumpPosVel(moons, 0)
		for i := 1; i <= testCase.steps; i++ {
			moons.Step()
			if i%10000000 == 0 {
				t.Logf("Step %v out of %v (%0.4v%%)", i, testCase.steps, float64(100*i)/float64(testCase.steps))
			}
			if testCase.steps-i < 10 {
				dumpPosVel(moons, i)
			}
		}

		t.Logf("Energy after %v steps:", testCase.steps)
		var sums []string
		var total int64
		for _, m := range moons.moons {
			pot, kin, energy := m.Energy()

			t.Logf("pot: %2v + %2v + %2v = %2v;  kin: %2v + %2v + %2v = %2v;  total: %2v * %2v = %3v",
				abs(m.pX), abs(m.pY), abs(m.pZ), pot,
				abs(m.vX), abs(m.vY), abs(m.vZ), kin,
				pot, kin, energy)

			sums = append(sums, strconv.FormatInt(int64(total), 10))
			total += energy
		}
		t.Logf("Sum of total energy: %v = %v", strings.Join(sums, " + "), total)

		total = moons.TotalEnergy()
		if total != testCase.totalEnergy {
			t.Errorf("ERROR: Unexpected total energy.  Expected:%v  Got:%v", testCase.totalEnergy, total)
		}

		if testCase.repeatsAfter > 0 {
			repeatMoons, err := newMoons(testCase.defs)
			if err != nil {
				t.Errorf("ERROR: %v", err)
			}
			repeatsAfter := repeatMoons.RepeatsAfter()
			if repeatsAfter < 0 {
				t.Error("ERROR: Could not found repeating pattern")
			} else {
				t.Logf("Found repeat")
				if repeatsAfter != testCase.repeatsAfter {
					t.Errorf("ERROR: Unexpected repeats after.  Expected:%v  Got:%v", testCase.repeatsAfter, repeatsAfter)
				}
			}
		}
	}
}

func BenchmarkSteps(b *testing.B) {
	moons, err := newMoons([]string{"<x=-8, y=-10, z=0>", "<x=5, y=5, z=10>", "<x=2, y=-7, z=3>", "<x=9, y=-8, z=-3>"})

	if err != nil {
		b.Fatal(err)
	}
	for i := 1; i <= b.N; i++ {
		moons.Step()
	}
}
