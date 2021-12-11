// https://adventofcode.com/2019

package main

import (
	"fmt"
	"os"
	"sync"
)

func main() {
	program, err := readProgram(os.Stdin)
	if err != nil {
		fmt.Printf("Err %v\n", err)
		return
	}

	network := newNetwork()
	var nics []*NIC
	var wg = &sync.WaitGroup{}

	for i := 0; i < 50; i++ {
		nic := newNIC(program, int64(i), network)
		nics = append(nics, nic)
		wg.Add(1)
		go func() {
			if err := nic.Run(); err != nil {
				fmt.Printf("ERROR: %v\n", err)
				return
			}
			wg.Done()
		}()
	}

	wg.Wait()
}
