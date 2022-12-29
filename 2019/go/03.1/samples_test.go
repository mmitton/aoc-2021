package main

import (
	"testing"
)

func TestPath(t *testing.T) {
	type TestCase struct {
		definition     string
		pointDistances []int64
	}
	var testCases = []*TestCase{
		&TestCase{"R8,U5,L5,D3", []int64{0, 8, 13, 8, 5}},
		&TestCase{"U7,R6,D4,L4", []int64{0, 7, 13, 9, 5}},
	}

	for _, testCase := range testCases {
		p, err := newPath(testCase.definition)
		if err != nil {
			t.Fatalf("Error making path %v: %v", testCase.definition, err)
		} else if p.TotalPoints() != len(testCase.pointDistances) {
			t.Errorf("ERROR: Number of points (%v) is not the expected number (%v)", p.TotalPoints(), len(testCase.pointDistances))
		} else {
			t.Log(p)
			point := p.firstPoint
			for i, expected := range testCase.pointDistances {
				got := point.DistanceFromCentralPort()
				if got != expected {
					t.Errorf("ERROR: Point %v  expected:%v  got:%v", i, expected, got)
					//} else {
					//	t.Logf("Point %v  expected:%v  got:%v", i, expected, got)
					//	t.Logf("AllPoints:%v", p.AllPoints())
				}
				point = point.next
			}
		}
	}
}

func TestIntersections(t *testing.T) {
	type TestCase struct {
		definitions           []string
		expectedIntersections Points
		expectedMinDistance   int64
	}
	var testCases = []*TestCase{
		&TestCase{
			[]string{"R8,U5,L5,D3", "U7,R6,D4,L4"},
			Points{
				&Point{2, -3, false, nil, nil},
				&Point{5, -5, false, nil, nil},
			},
			6,
		},
		&TestCase{
			[]string{"R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"},
			nil,
			159,
		},
		&TestCase{
			[]string{"R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"},
			nil,
			135,
		},
	}

	for _, testCase := range testCases {
		var paths Paths
		for _, def := range testCase.definitions {
			p, err := newPath(def)
			if err != nil {
				t.Fatalf("Cannot create path %v: %v", def, err)
			}
			paths = append(paths, p)
		}
		intersections := paths.Intersections()
		if len(intersections) == 0 {
			t.Fatalf("No intersections found!")
		}
		t.Logf("Intersections: %v", intersections)
		if testCase.expectedMinDistance != intersections[0].DistanceFromCentralPort() {
			t.Errorf("ERROR: Min distance does not match.  Expected:%v  Got:%v", testCase.expectedMinDistance, intersections[0].DistanceFromCentralPort())
		} else {
			t.Logf("Expected Min Distance:%v  Got:%v", testCase.expectedMinDistance, intersections[0].DistanceFromCentralPort())
		}

		if len(testCase.expectedIntersections) > 0 {
			if len(intersections) != len(testCase.expectedIntersections) {
				t.Fatalf("Number of intersections (%v) is not expected (%v)", len(intersections), len(testCase.expectedIntersections))
			} else {
				for _, p := range intersections {
					if testCase.expectedIntersections.Contains(p) {
						t.Errorf("Expected intersection %v, did not find in %v", p, testCase.expectedIntersections)
					}
				}
			}
		}
	}
}
