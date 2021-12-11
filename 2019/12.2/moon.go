package main

import (
	"fmt"
	"regexp"
	"strconv"
)

type Moons struct {
	moons []*Moon
}

type Moon struct {
	pX, pY, pZ int64
	vX, vY, vZ int64
}

func abs(v int64) int64 {
	if v < 0 {
		return -v
	}
	return v
}

func newMoons(defs []string) (*Moons, error) {
	ms := &Moons{}
	for defIdx, def := range defs {
		m, err := newMoon(def)
		if err != nil {
			return nil, fmt.Errorf("Def %v: %v", defIdx, err)
		}
		ms.moons = append(ms.moons, m)
	}

	return ms, nil
}

func newMoon(def string) (*Moon, error) {
	re := regexp.MustCompile(`<x=([-]?\d+), y=([-]?\d+), z=([-]?\d+)>`)
	parts := re.FindStringSubmatch(def)
	if len(parts) != 4 {
		return nil, fmt.Errorf("'%v' is not a proper moon position", def)
	}

	parseInt64 := func(s string, v string) (int64, error) {
		ret, err := strconv.ParseInt(v, 10, 64)
		if err != nil {
			return 0, fmt.Errorf("Cannot convert %v=%v to number: %v", s, v, err)
		}
		return int64(ret), nil
	}

	m := &Moon{}
	var err error
	if m.pX, err = parseInt64("x", parts[1]); err != nil {
		return nil, err
	}
	if m.pY, err = parseInt64("y", parts[2]); err != nil {
		return nil, err
	}
	if m.pZ, err = parseInt64("z", parts[3]); err != nil {
		return nil, err
	}

	return m, nil
}

func (m *Moon) String() string {
	return fmt.Sprintf("pos=<x=%3v, y=%3v, z=%3v>  vel=<x=%3v, y=%3v, z=%3v>", m.pX, m.pY, m.pZ, m.vX, m.vY, m.vZ)
}

func (m *Moon) Energy() (int64, int64, int64) {
	pot := abs(m.pX) + abs(m.pY) + abs(m.pZ)
	kin := abs(m.vX) + abs(m.vY) + abs(m.vZ)

	return pot, kin, pot * kin
}

func (ms *Moons) TotalEnergy() int64 {
	var total int64
	for _, m := range ms.moons {
		_, _, energy := m.Energy()

		total += energy
	}
	return total
}

func (ms *Moons) RepeatsAfter() int64 {
	findSteps := func(p []int64) int64 {
		var steps int64
		var v = make([]int64, len(p))

		for {
			steps++
			for i := 0; i < len(p); i++ {
				for j := i + 1; j < len(p); j++ {
					if p[i] < p[j] {
						v[i]++
						v[j]--
					} else if p[i] > p[j] {
						v[i]--
						v[j]++
					}
				}
			}
			for i := 0; i < len(p); i++ {
				p[i] += v[i]
			}

			done := true
			for _, v := range v {
				if v != 0 {
					done = false
					break
				}
			}
			if done {
				return steps
			}
		}
	}

	var x, y, z []int64
	for _, m := range ms.moons {
		x = append(x, m.pX)
		y = append(y, m.pY)
		z = append(z, m.pZ)
	}
	stepsX := findSteps(x)
	stepsY := findSteps(y)
	stepsZ := findSteps(z)

	loopCount := stepsX
	if loopCount < stepsY {
		loopCount = stepsY
	}
	if loopCount < stepsZ {
		loopCount = stepsZ
	}
	fmt.Printf("stepsX:%v  stepsY:%v  stepsZ:%v  loopCount:%v\n", stepsX, stepsY, stepsZ, loopCount)

	var steps int64
	for {
		steps += loopCount
		if steps < 0 {
			return -1
		}
		mX := steps % stepsX
		mY := steps % stepsY
		mZ := steps % stepsZ
		// fmt.Printf("steps:%v  mX:%v  mY:%v  mZ:%v\n", steps, mX, mY, mZ)
		if mX == 0 && mY == 0 && mZ == 0 {
			return steps * 2
		}
	}
}

func (ms *Moons) clone() *Moons {
	ret := &Moons{}
	for _, m := range ms.moons {
		ret.moons = append(ret.moons, m.clone())
	}
	return ret
}

func (m *Moon) clone() *Moon {
	return &Moon{m.pX, m.pY, m.pZ, m.vX, m.vY, m.vZ}
}

func (ms *Moons) Equals(ms2 *Moons) bool {
	for i := 0; i < len(ms.moons); i++ {
		if !ms.moons[i].Equals(ms2.moons[i]) {
			return false
		}
	}

	return true
}

func (m *Moon) Equals(m2 *Moon) bool {
	return m.pX == m2.pX && m.pY == m2.pY && m.pZ == m2.pZ && m.vX == m2.vX && m.vY == m2.vY && m.vZ == m2.vZ
}

func (ms *Moons) Step() {
	totalMoons := len(ms.moons)
	for i := 0; i < totalMoons; i++ {
		for j := i + 1; j < totalMoons; j++ {
			m1, m2 := ms.moons[i], ms.moons[j]
			x1, y1, z1 := m1.pX, m1.pY, m1.pZ
			x2, y2, z2 := m2.pX, m2.pY, m2.pZ

			if x1 < x2 {
				m1.vX++
				m2.vX--
			} else if x1 > x2 {
				m1.vX--
				m2.vX++
			}

			if y1 < y2 {
				m1.vY++
				m2.vY--
			} else if y1 > y2 {
				m1.vY--
				m2.vY++
			}

			if z1 < z2 {
				m1.vZ++
				m2.vZ--
			} else if z1 > z2 {
				m1.vZ--
				m2.vZ++
			}
		}
	}
	for _, m := range ms.moons {
		m.pX += m.vX
		m.pY += m.vY
		m.pZ += m.vZ
	}
}
