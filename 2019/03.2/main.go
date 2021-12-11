// https://adventofcode.com/2019

package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type Paths []*Path
type Points []*Point

type Path struct {
	firstPoint *Point
	lastPoint  *Point
}

type Point struct {
	x, y           int64
	isIntersection bool
	next, prev     *Point
}

func (p *Path) String() string {
	var points []string
	for point := p.firstPoint; point != nil; point = point.next {
		points = append(points, point.String())
	}
	return fmt.Sprintf("Path { %v }", strings.Join(points, ", "))
}

func (p *Path) PropDelay(p1 *Point) int64 {
	delay := int64(0)
	for p2 := p.firstPoint; p2 != nil; p2 = p2.next {
		if p2.prev != nil {
			delay += p2.DistanceTo(p2.prev)
		}
		if p2.Equals(p1) {
			return delay
		}
	}

	return -1
}

func (p *Point) Equals(p2 *Point) bool {
	return p.x == p2.x && p.y == p2.y
}

func (p Paths) Intersections() Points {
	if len(p) < 2 {
		return nil
	}

	var intersections Points
	addPoint := func(x, y int64) {
		addPoint := true
		if x == 0 && y == 0 {
			return
		}
		for _, path := range p[1:] {
			found := false

			for p1 := path.firstPoint; p1.next != nil; p1 = p1.next {
				p2 := p1.next
				if p1.ContainsPoint(p2, x, y, true) {
					found = true
					break
				}
			}

			if !found {
				addPoint = false
				break
			}
		}

		if addPoint {
			newPoint := &Point{x, y, false, nil, nil}
			for _, p := range intersections {
				if p.Equals(newPoint) {
					return
				}
			}
			intersections = append(intersections, newPoint)
		}
	}

	for p1 := p[0].firstPoint; p1.next != nil; p1 = p1.next {
		p2 := p1.next

		var dx, dy int64 = 0, 0
		if p1.x < p2.x {
			dx = 1
		} else if p1.x > p2.x {
			dx = -1
		}
		if p1.y < p2.y {
			dy = 1
		} else if p1.y > p2.y {
			dy = -1
		}

		x, y := p1.x, p1.y
		for {
			addPoint(x, y)

			if p2.x == x && p2.y == y {
				break
			}

			x += dx
			y += dy
		}
	}

	for _, point := range intersections {
		point.isIntersection = true
		for _, path := range p {
			for p := path.firstPoint; p != nil; p = p.next {
				if p.Equals(point) {
					p.isIntersection = true
				}
			}
			for p1 := path.firstPoint; p1.next != nil; p1 = p1.next {
				if p1.ContainsPoint(p1.next, point.x, point.y, false) {
					// Insert new point!
					newPoint := &Point{point.x, point.y, true, p1.next, p1}
					newPoint.prev.next = newPoint
					newPoint.next.prev = newPoint
				}
			}
		}
	}

	sort.Slice(intersections, func(i, j int) bool {
		var a = intersections[i]
		var b = intersections[j]

		if a.DistanceFromCentralPort() != b.DistanceFromCentralPort() {
			return a.DistanceFromCentralPort() < b.DistanceFromCentralPort()
		}

		if a.x != b.x {
			return a.x < b.x
		}
		return a.y < b.y
	})

	return intersections
}

func (ps Points) Contains(p *Point) bool {
	for _, p2 := range ps {
		if p.Equals(p2) {
			return true
		}
	}
	return false
}

func (p *Point) ContainsPoint(p2 *Point, x, y int64, inclusive bool) bool {
	if !inclusive {
		if p.x == x && p.y == y {
			return false
		}
		if p2.x == x && p2.y == y {
			return false
		}
	}

	minX, maxX := p.x, p2.x
	if maxX < minX {
		minX, maxX = maxX, minX
	}

	minY, maxY := p.y, p2.y
	if maxY < minY {
		minY, maxY = maxY, minY
	}

	return minX <= x && x <= maxX && minY <= y && y <= maxY
}

func (p *Point) DistanceFromCentralPort() int64 {
	x := p.x
	y := p.y

	if x < 0 {
		x = -x
	}
	if y < 0 {
		y = -y
	}

	return x + y
}

func (p *Point) DistanceTo(p2 *Point) int64 {
	x := p.x - p2.x
	y := p.y - p2.y

	if x < 0 {
		x = -x
	}
	if y < 0 {
		y = -y
	}

	return x + y
}

func (p *Point) String() string {
	ret := fmt.Sprintf("%v,%v", p.x, p.y)
	if p.isIntersection {
		ret += " INT"
	}
	return "{" + ret + "}"
}

func (p *Path) TotalPoints() int {
	totalPoints := 0
	for point := p.firstPoint; point != nil; point = point.next {
		totalPoints++
	}
	return totalPoints
}

func newPath(definition string) (*Path, error) {
	p := &Path{}

	p.firstPoint = &Point{0, 0, false, nil, nil}
	p.lastPoint = p.firstPoint

	directions := strings.Split(definition, ",")
	for _, dir := range directions {
		dir = strings.ToUpper(strings.TrimSpace(dir))
		dirLen, err := strconv.ParseInt(dir[1:], 10, 64)
		if err != nil {
			return nil, err
		}

		var point *Point
		switch dir[0] {
		case 'U':
			point = &Point{p.lastPoint.x, p.lastPoint.y - dirLen, false, nil, nil}
		case 'D':
			point = &Point{p.lastPoint.x, p.lastPoint.y + dirLen, false, nil, nil}
		case 'L':
			point = &Point{p.lastPoint.x - dirLen, p.lastPoint.y, false, nil, nil}
		case 'R':
			point = &Point{p.lastPoint.x + dirLen, p.lastPoint.y, false, nil, nil}
		}
		if point == nil {
			return nil, fmt.Errorf("Invalid direction: %v", dir)
		}

		p.lastPoint.next = point
		point.prev = p.lastPoint
		p.lastPoint = point
	}

	return p, nil
}

func (p Paths) findSmallestDelayProp() (int64, *Point, error) {
	if len(p) < 2 {
		return 0, nil, fmt.Errorf("Not enough paths: %v generated", len(p))
	}

	intersections := p.Intersections()
	if len(intersections) == 0 {
		return 0, nil, errors.New("Cannot find any intersections")
	}

	var smallestDelayProp int64 = 0
	var smallestDelayPoint *Point
	for _, ip := range intersections {
		totalPropDelay := int64(0)
		for _, path := range p {
			propDelay := path.PropDelay(ip)
			if propDelay < 0 {
				return 0, nil, fmt.Errorf("Intersection point %v not on path %v", ip, path)
			}
			totalPropDelay += propDelay
		}

		if smallestDelayProp == 0 || totalPropDelay < smallestDelayProp {
			smallestDelayProp = totalPropDelay
			smallestDelayPoint = ip
		}
	}

	return smallestDelayProp, smallestDelayPoint, nil
}

func main() {
	s := bufio.NewScanner(os.Stdin)
	var paths Paths
	for s.Scan() {
		path, err := newPath(s.Text())
		if err != nil {
			fmt.Printf("ERR: %v", err)
			return
		}
		paths = append(paths, path)
	}

	smallestDelayProp, smallestDelayPoint, err := paths.findSmallestDelayProp()
	if err != nil {
		fmt.Printf("ERROR: %v", err)
	} else {
		fmt.Printf("Smallest Delay Prop: %v\n", smallestDelayProp)
		fmt.Printf("Smallest Delay Point: %v\n", smallestDelayPoint)
	}
}
