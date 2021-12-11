package main

import "fmt"

type Amplifier struct {
	id         rune
	ic         *Intcode
	setting    int64
	didSetting bool
	In         chan int64
	Out        chan int64
	Done       chan bool
	LastOutput int64
}

func newAmplifier(id int, program []int64, setting int64) *Amplifier {
	inChan := make(chan int64, 1)
	outChan := make(chan int64, 1)
	doneChan := make(chan bool, 1)
	a := &Amplifier{
		id:         rune(id),
		ic:         newIntcode(program),
		setting:    setting,
		didSetting: false,
		In:         inChan,
		Out:        outChan,
		Done:       doneChan,
	}

	a.ic.Input = func() (v int64) {
		if a.didSetting {
			return <-a.In
		}

		a.didSetting = true
		return a.setting
	}

	a.ic.Output = func(v int64) {
		a.LastOutput = v
		a.Out <- v
	}

	return a
}

func (a *Amplifier) run() {
	defer func() {
		a.Done <- true
		close(a.Done)
	}()

	err := a.ic.run()
	if err != nil {
		fmt.Printf("ERROR: Amplifier %v: %v\n", a.id, err)
	}
}

func amplifierCircuit(program []int64, settings ...int64) int64 {
	var amps []*Amplifier
	var lastAmp *Amplifier

	for id, setting := range settings {
		amp := newAmplifier(id, program, setting)
		if lastAmp != nil {
			amp.In = lastAmp.Out
			amps[0].In = amp.Out
		}
		lastAmp = amp
		amps = append(amps, lastAmp)
	}

	for _, amp := range amps {
		go amp.run()
	}

	amps[0].In <- 0
	<-lastAmp.Done
	return lastAmp.LastOutput
}
