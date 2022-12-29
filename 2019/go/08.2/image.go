package main

import (
	"fmt"
	"io"
	"strconv"
	"strings"
	"unicode"
)

type Image struct {
	width, height int
	data          []int
	layers        []ImageLayer
}

type ImageRender []string
type ImageLayer [][]int

func decodeImage(r io.Reader, width, height int) (*Image, error) {
	var i = &Image{width: width, height: height}
	for {
		var dr rune
		if _, err := fmt.Fscanf(r, "%c", &dr); err != nil {
			if err != io.EOF {
				return nil, err
			}
			break
		}
		if unicode.IsSpace(dr) {
			continue
		}
		if unicode.IsDigit(dr) {
			d, err := strconv.ParseInt(string(dr), 10, 64)
			if err != nil {
				return nil, err
			}
			if d < 0 || d > 2 {
				return nil, fmt.Errorf("Unexpected char %c", dr)
			}
			i.data = append(i.data, int(d))
		} else {
			return nil, fmt.Errorf("Unexpected char %c", dr)
		}
	}

	if len(i.data)%(width*height) != 0 {
		return nil, fmt.Errorf("Expected length of data to be a multiple of %v*%v (%v), got %v", width, height, width*height, len(i.data))
	}

	layerPos := 0
	for layerPos < len(i.data) {
		layerData := i.data[layerPos : layerPos+(width*height)]

		var layer ImageLayer = nil
		rowPos := 0
		for rowPos < len(layerData) {
			layer = append(layer, layerData[rowPos:rowPos+width])
			rowPos += width
		}
		i.layers = append(i.layers, layer)

		layerPos += width * height
	}

	return i, nil
}

func (l ImageLayer) Equals(l2 ImageLayer) bool {
	if len(l) != len(l2) {
		return false
	}
	for i := 0; i < len(l); i++ {
		r1, r2 := l[i], l2[i]
		if len(r1) != len(r2) {
			return false
		}
		for j := 0; j < len(r1); j++ {
			if r1[j] != r2[j] {
				return false
			}
		}
	}
	return true
}

func (l ImageLayer) CountDigits(d ...int) int {
	ret := 0
	for _, r := range l {
		for _, c := range r {
			for _, d := range d {
				if d == c {
					ret++
				}
			}
		}
	}

	return ret
}

func (l ImageLayer) ValueAt(x, y int) int {
	return l[y][x]
}

func (i *Image) Render() ImageRender {
	var ret ImageRender

	for y := 0; y < i.height; y++ {
		var line string

		for x := 0; x < i.width; x++ {
			var cell int = 2
			for _, l := range i.layers {
				cell = l.ValueAt(x, y)
				if cell != 2 {
					break
				}
			}
			line += strconv.FormatInt(int64(cell), 10)
		}

		ret = append(ret, line)
	}
	return ret
}

func (ir ImageRender) String() string {
	sb := &strings.Builder{}

	for _, r := range ir {
		fmt.Fprintln(sb, strings.ReplaceAll(r, "0", " "))
	}

	return sb.String()
}
