// https://adventofcode.com/2019

package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	s := bufio.NewScanner(os.Stdin)
	var inputSignals []string

	for s.Scan() {
		inputSignals = append(inputSignals, s.Text())
	}

	for inputIdx, inputSignal := range inputSignals {
		fft, err := newFFT(inputSignal)
		if err != nil {
			fmt.Printf("ERROR: %v => %v", inputSignal, err)
			continue
		}

		fft.runNSteps(100)
		fmt.Printf("Signal %v after %v steps: %v\n", inputIdx+1, fft.steps, fft.output[:8])
	}
}
