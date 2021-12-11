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
		rendered      []string
	}
	var testCases = []*TestCase{
		&TestCase{2, 2, "0222112222120000", []ImageLayer{
			ImageLayer{[]int{0, 2}, []int{2, 2}},
			ImageLayer{[]int{1, 1}, []int{2, 2}},
			ImageLayer{[]int{2, 2}, []int{1, 2}},
			ImageLayer{[]int{0, 0}, []int{0, 0}}},
			[]string{"01", "10"},
		},
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

			rendered := image.Render()
			logRendered := func(desc string, lines []string) {
				t.Log(desc)
				for _, l := range lines {
					t.Log("  " + l)
				}
			}
			logRendered("Render Expected:", testCase.rendered)
			logRendered("Render Got:", rendered)
			if len(rendered) != len(testCase.rendered) {
				t.Errorf("ERROR: Expected %v lines rendered, got %v", len(testCase.rendered), len(rendered))
			} else {
				for i := 0; i < len(rendered); i++ {
					if rendered[i] != testCase.rendered[i] {
						t.Errorf("ERROR: Render row %v does not match.  Expected %v, Got %v", i, testCase.rendered[i], rendered[i])
					}
				}
			}
		}
	}
}
