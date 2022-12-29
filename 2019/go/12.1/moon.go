package main

import (
	"fmt"
	"regexp"
	"strconv"
)

type Moons []*Moon

type Triple struct {
	x, y, z int64
}

type Moon struct {
	pos *Triple
	vel *Triple
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
		return ret, nil
	}

	m := &Moon{pos: &Triple{}, vel: &Triple{}}
	var err error
	if m.pos.x, err = parseInt64("x", parts[1]); err != nil {
		return nil, err
	}
	if m.pos.y, err = parseInt64("y", parts[2]); err != nil {
		return nil, err
	}
	if m.pos.z, err = parseInt64("z", parts[3]); err != nil {
		return nil, err
	}

	return m, nil
}

func (t *Triple) String() string {
	return fmt.Sprintf("<x=%3v, y=%3v, z=%3v>", t.x, t.y, t.z)
}

func (m *Moon) String() string {
	return fmt.Sprintf("pos=%v  vel=%v", m.pos, m.vel)
}

func (m *Moon) Energy() (int64, int64, int64) {
	abs := func(v int64) int64 {
		if v < 0 {
			return -v
		}
		return v
	}
	pot := abs(m.pos.x) + abs(m.pos.y) + abs(m.pos.z)
	kin := abs(m.vel.x) + abs(m.vel.y) + abs(m.vel.z)

	return pot, kin, pot * kin
}

func (ms Moons) TotalEnergy() int64 {
	var total int64
	for _, m := range ms {
		_, _, energy := m.Energy()

		total += energy
	}
	return total
}

func (ms Moons) LoopsAfter() int64 {
	var previousStates = []Moons{ms.clone()}

	var steps int64
	for {
		steps++
		ms.Step()
		for _, ms2 := range previousStates {
			if ms2.Equals(ms) {
				return steps
			}
		}

		if steps < 0 {
			return -1
		}

		previousStates = append(previousStates, ms.clone())
	}
}

func (ms Moons) clone() Moons {
	var ret Moons
	for _, m := range ms {
		ret = append(ret, m.clone())
	}
	return ret
}

func (m *Moon) clone() *Moon {
	return &Moon{pos: &Triple{m.pos.x, m.pos.y, m.pos.z}, vel: &Triple{m.vel.x, m.vel.y, m.vel.z}}
}

func (ms Moons) Equals(ms2 Moons) bool {
	for i := 0; i < len(ms); i++ {
		if !ms[i].Equals(ms2[i]) {
			return false
		}
	}

	return true
}

func (m *Moon) Equals(m2 *Moon) bool {
	return m.pos.Equals(m2.pos) && m.vel.Equals(m2.vel)
}

func (t *Triple) Equals(t2 *Triple) bool {
	return t.x == t2.x && t.y == t2.y && t.z == t2.z
}

func (ms Moons) Step() {
	calcPull := func(a, b int64) int64 {
		if a < b {
			return 1
		}
		if a > b {
			return -1
		}
		return 0
	}

	for _, m1 := range ms {
		for _, m2 := range ms {
			if m1 == m2 {
				continue
			}

			m1.vel.x += calcPull(m1.pos.x, m2.pos.x)
			m1.vel.y += calcPull(m1.pos.y, m2.pos.y)
			m1.vel.z += calcPull(m1.pos.z, m2.pos.z)
		}
	}

	for _, m := range ms {
		m.pos.x += m.vel.x
		m.pos.y += m.vel.y
		m.pos.z += m.vel.z
	}
}
