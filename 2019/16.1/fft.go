package main

import "fmt"

type FFT struct {
	inputSignal string
	output      string
	steps       int
}

func newFFT(inputSignal string) (*FFT, error) {
	_, err := parseSignal(inputSignal)
	if err != nil {
		return nil, err
	}
	return &FFT{inputSignal: inputSignal, output: inputSignal, steps: 0}, nil
}

func parseSignal(signal string) ([]int, error) {
	var ret []int
	for idx, c := range signal {
		if c < '0' || c > '9' {
			return nil, fmt.Errorf("Expected digit at pos %v, got %v", idx, c)
		}
		ret = append(ret, int(c-'0'))
	}

	return ret, nil
}

func (fft *FFT) runNSteps(steps int) error {
	for i := 0; i < steps; i++ {
		if err := fft.runStep(); err != nil {
			return err
		}
	}

	return nil
}

func (fft *FFT) runStep() error {
	fft.steps++

	nums, err := parseSignal(fft.output)
	if err != nil {
		return err
	}

	generatePattern := func(pos int) []int {
		var ret []int
		for len(ret) <= len(nums) {
			for _, a := range []int{0, 1, 0, -1} {
				for b := 0; b < pos; b++ {
					ret = append(ret, a)
				}
			}
		}

		return ret[1 : len(nums)+1]
	}

	fft.output = ""
	for i := 1; i <= len(nums); i++ {
		pattern := generatePattern(i)
		n := 0
		for j := 0; j < len(nums); j++ {
			n += nums[j] * pattern[j]
		}
		n = n % 10
		if n < 0 {
			n = -n
		}
		fft.output += fmt.Sprintf("%v", n)
	}

	return nil
}
