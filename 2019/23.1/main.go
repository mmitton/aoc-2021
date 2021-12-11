// https://adventofcode.com/2019

package main

import (
	"fmt"
	"os"
	"time"
)

func main() {
	program, err := readProgram(os.Stdin)
	if err != nil {
		fmt.Printf("Err %v\n", err)
		return
	}

	network := newNetwork()
	var nics []*NIC
	for i := 0; i < 50; i++ {
		nic := newNIC(program, int64(i), network)
		nics = append(nics, nic)
		go nic.Run()
	}

	for {
		packet := network.RecvPacket(255)
		if packet != nil {
			fmt.Printf("Packet: %v\n", packet)
			return
		}
		time.Sleep(10)
	}
}
