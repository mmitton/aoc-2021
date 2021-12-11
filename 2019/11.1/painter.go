package main

import (
	"fmt"
)

type PainterState int64

const (
	PainterWaitingColor PainterState = iota
	PainterWaitingTurn
)

func (ps PainterState) String() string {
	switch ps {
	case PainterWaitingColor:
		return "Waiting Color"
	case PainterWaitingTurn:
		return "Waiting Turn"
	}
	return "Unknown State"
}

type Painter struct {
	head         *Panel
	tail         *Panel
	cur          *Panel
	direction    int
	x, y         int
	ic           *Intcode
	state        PainterState
	stateChanged func(state PainterState)
}

type Panel struct {
	color     int64
	x, y      int
	next      *Panel
	isPainted bool
}

func newPainter(program []int64) *Painter {
	head := &Panel{
		color: 0,
		x:     0,
		y:     0,
		next:  nil,
	}
	p := &Painter{
		head:      head,
		tail:      head,
		cur:       head,
		direction: 0,
		x:         0,
		y:         0,
		ic:        newIntcode(program),
		state:     PainterWaitingColor,
	}

	p.ic.Input = p.Input
	p.ic.Output = p.Output

	return p
}

func (p *Painter) Input() int64 {
	return p.cur.color
}

func (p *Painter) run() error {
	return p.ic.run()
}

func (p *Painter) generateOutput(minX, minY, maxX, maxY int, showDirection bool) []string {
	if minX > maxX {
		minX, maxX = maxX, minX
	}
	if minY > maxY {
		minY, maxY = maxY, minY
	}
	if minX == 0 && maxX == 0 && minY == 0 && maxY == 0 {
		for n := p.head; n != nil; n = n.next {
			if n.isPainted {
				if n.x < minX {
					minX = n.x
				}
				if n.x > maxX {
					maxX = n.x
				}
				if n.y < minY {
					minY = n.y
				}
				if n.y > maxY {
					maxY = n.y
				}
			}
		}
	}

	var ret []string
	for y := minY; y <= maxY; y++ {
		var row string
		for x := minX; x <= maxX; x++ {
			panel := p.findPanel(x, y)
			if panel == p.cur && showDirection {
				switch p.direction {
				case 0:
					row += "^"
				case 1:
					row += ">"
				case 2:
					row += "v"
				case 3:
					row += "<"
				default:
					row += fmt.Sprint(p.direction)
				}
			} else {
				if panel.color == 0 {
					row += "."
				} else {
					row += "#"
				}
			}
		}
		ret = append(ret, row)
	}
	return ret
}

func (p *Painter) findPanel(x, y int) *Panel {
	for n := p.head; n != nil; n = n.next {
		if n.x == x && n.y == y {
			return n
		}
	}

	n := &Panel{
		color: 0,
		x:     x,
		y:     y,
		next:  nil,
	}

	p.tail.next = n
	p.tail = n
	return n
}

func (p *Painter) numberPainted() int {
	total := 0
	for n := p.head; n != nil; n = n.next {
		if n.isPainted {
			total++
		}
	}

	return total
}

func (p *Painter) Output(v int64) {
	if v != 0 && v != 1 {
		fmt.Printf("ERROR: Painter got a value of %v (expecting 0 or 1) while in state %v\n", v, p.state)
		return
	}
	switch p.state {
	case PainterWaitingColor:
		p.cur.color = v
		p.cur.isPainted = true

		if p.stateChanged != nil {
			p.stateChanged(PainterWaitingTurn)
		}

		p.state = PainterWaitingTurn
	case PainterWaitingTurn:
		if v == 0 {
			p.direction--
			if p.direction == -1 {
				p.direction = 3
			}
		} else {
			p.direction++
			if p.direction == 4 {
				p.direction = 0
			}
		}
		switch p.direction {
		case 0:
			// Up (y--)
			p.y--
		case 1:
			// Right (x++)
			p.x++
		case 2:
			// Down (y++)
			p.y++
		case 3:
			// Right (x--)
			p.x--
		}
		p.cur = p.findPanel(p.x, p.y)

		if p.stateChanged != nil {
			p.stateChanged(PainterWaitingTurn)
		}
		p.state = PainterWaitingColor
	}
}
