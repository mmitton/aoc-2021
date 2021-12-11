package main

func newAmplifier(program []int64, setting int64, input <-chan int64) <-chan int64 {
	ic := newIntcode(program)
	outChan := make(chan int64)

	didSetting := false
	ic.Input = func() int64 {
		if didSetting {
			return <-input
		}
		didSetting = true
		return setting
	}

	ic.Output = func(v int64) {
		outChan <- v
	}

	go func() {
		ic.run()
		close(outChan)
	}()
	return outChan
}

func amplifierCircuit(program []int64, settings ...int64) int64 {
	var signals []<-chan int64
	startSignal := make(chan int64)
	signals = append(signals, startSignal)

	for _, setting := range settings {
		signals = append(signals, newAmplifier(program, setting, signals[len(signals)-1]))
	}

	startSignal <- 0
	return <-signals[len(signals)-1]
}
