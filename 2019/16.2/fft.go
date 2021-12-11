package main

import (
	"fmt"
	"strings"
	"sync"
	"time"
)

type FFT struct {
	inputSignal  string
	outputBuffer []int
	tmpBuffer    []int
	adds         [][]int
	subs         [][]int
	steps        int
}

func newFFT(inputSignal string) (*FFT, error) {
	outputBuffer, err := parseSignal(inputSignal)
	if err != nil {
		return nil, err
	}
	fft := &FFT{inputSignal: inputSignal, outputBuffer: outputBuffer, steps: 0}
	fft.tmpBuffer = make([]int, len(outputBuffer))

	return fft, nil
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

func (fft *FFT) runNSteps(steps int) {
	for i := 0; i < steps; i++ {
		fft.runStep()
	}
}

func (fft *FFT) getMessage(steps, offset int) (string, error) {
	if offset < 0 || offset >= len(fft.outputBuffer)-8 {
		return "", fmt.Errorf("%v out of bounds", offset)
	}
	if offset < len(fft.outputBuffer)/2 {
		for i := 0; i < steps; i++ {
			fft.runStep()
		}
	} else {
		totalNumbers := len(fft.outputBuffer)
		fmt.Printf("totalNumbers:%v  offset:%v\n", totalNumbers, offset)
		fmt.Printf("Quick method! %v\n", totalNumbers-offset)
		start := time.Now()
		for i := 0; i < steps; i++ {
			now := time.Now()
			if i > 0 {
				fmt.Printf("Step %v, Duration:%v, Time Left:%v\n", i+1, now.Sub(start), now.Sub(start)*time.Duration(steps)/time.Duration(i))
			} else {
				fmt.Printf("Step %v\n", i+1)
			}

			n := 0
			for j := totalNumbers - 1; j >= offset; j-- {
				n += fft.outputBuffer[j]
				fft.tmpBuffer[j] = n % 10
			}

			fft.outputBuffer, fft.tmpBuffer = fft.tmpBuffer, fft.outputBuffer
		}
	}
	output := fft.Output()
	return output[offset : offset+8], nil
}

func (fft *FFT) runStep() {
	totalNumbers := len(fft.outputBuffer)
	fft.steps++
	fmt.Printf("runStep: %v %v\n", fft.steps, totalNumbers)

	calcDigit := func(i int) {
		l := i + 1
		n := 0
		mul := 1
		for j := i; j < totalNumbers; j += l {
			for k := 0; k < l && j+k < totalNumbers; k++ {
				n += fft.outputBuffer[j+k] * mul
			}
			j += l
			mul *= -1
		}
		n = n % 10
		if n < 0 {
			n = -n
		}
		fft.tmpBuffer[i] = n
	}

	wg := &sync.WaitGroup{}

	worker := func(worker int, indexes []int) {
		for _, i := range indexes {
			calcDigit(i)
		}
		wg.Done()
	}

	var totalWorkers = 16
	wg.Add(totalWorkers)
	var workers = make([][]int, totalWorkers)
	for i := 0; i < totalNumbers; i++ {
		worker := i % totalWorkers
		workers[worker] = append(workers[worker], i)
	}
	for i := 0; i < totalWorkers; i++ {
		go worker(i, workers[i])
	}

	wg.Wait()
	fft.outputBuffer, fft.tmpBuffer = fft.tmpBuffer, fft.outputBuffer
}

func (fft *FFT) Output() string {
	var ret = &strings.Builder{}
	for _, n := range fft.outputBuffer {
		fmt.Fprintf(ret, "%v", n)
	}
	return ret.String()
}
