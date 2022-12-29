// https://adventofcode.com/2019

package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	s := bufio.NewScanner(os.Stdin)
	var inputSignals []string

	for s.Scan() {
		sb := &strings.Builder{}
		for i := 0; i < 10000; i++ {
			sb.WriteString(s.Text())
		}
		inputSignals = append(inputSignals, sb.String())
	}

	for _, inputSignal := range inputSignals {
		displaySignal := inputSignal
		if len(displaySignal) > 20 {
			displaySignal = displaySignal[:37] + "..."
		}
		offset, err := strconv.ParseInt(inputSignal[:7], 10, 64)
		if err != nil {
			fmt.Printf("ERROR: %v => %v\n", displaySignal, err)
			continue
		}

		fft, err := newFFT(inputSignal)
		if err != nil {
			fmt.Printf("ERROR: %v => %v\n", displaySignal, err)
			continue
		}

		message, err := fft.getMessage(100, int(offset))
		if err != nil {
			fmt.Printf("ERROR: %v => %v\n", displaySignal, err)
			continue
		}

		fmt.Printf("signal:%v  offset:%v  message:%v\n", displaySignal, offset, message)
		// fft.runNSteps(100)
		// output := fft.Output()
		// fmt.Printf("Signal %v after %v steps: offset:%v  message:%v\n", inputIdx+1, fft.steps, offset, output[offset:offset+8])
	}
}
