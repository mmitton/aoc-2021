package main

import (
	"strings"
	"testing"
)

func TestImageDecode(t *testing.T) {
	type TestCase struct {
		width, height int
		data          string
		layers        []ImageLayer
	}
	var testCases = []*TestCase{
		&TestCase{3, 2, "123456789012", []ImageLayer{
			ImageLayer{[]int{1, 2, 3}, []int{4, 5, 6}},
			ImageLayer{[]int{7, 8, 9}, []int{0, 1, 2}},
		}},
	}

	for _, testCase := range testCases {
		image, err := decodeImage(strings.NewReader(testCase.data), testCase.width, testCase.height)
		if err != nil {
			t.Errorf("ERROR: Cannot decode image: %v", err)
		} else {
			t.Logf("Image: %v", image)
			if len(image.layers) != len(testCase.layers) {
				t.Errorf("ERROR: Expected %v layers, got %v", len(testCase.layers), len(image.layers))
			} else {
				for i := 0; i < len(image.layers); i++ {
					if !image.layers[i].Equals(testCase.layers[i]) {
						t.Errorf("ERROR: Layer %v does not match.  Expected %v, Got %v", i, testCase.layers[i], image.layers[i])
					}
				}
			}
		}
	}
}
