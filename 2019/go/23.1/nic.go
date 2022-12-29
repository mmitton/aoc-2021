package main

import "fmt"

type NIC struct {
	ic      *Intcode
	address int64
	input   []int64
	output  []int64
	network *Network
}

func newNIC(program []int64, address int64, network *Network) *NIC {
	nic := &NIC{ic: newIntcode(program), address: address, network: network}
	nic.ic.Input = nic.Input
	nic.ic.Output = nic.Output
	nic.input = []int64{address}
	return nic
}

func (nic *NIC) Run() (err error) {
	defer func() {
		if err != nil {
			fmt.Printf("ERROR: %v", err)
		}
	}()
	if err := nic.ic.run(); err != nil {
		return fmt.Errorf("NIC %v: %v", nic.address, err)
	}

	return nil
}

func (nic *NIC) Input() int64 {
	if len(nic.input) == 0 {
		nic.input = nic.network.RecvPacket(nic.address)
		if len(nic.input) == 0 {
			return -1
		}
	}

	ret := nic.input[0]
	nic.input = nic.input[1:]
	return ret
}

func (nic *NIC) Output(v int64) {
	nic.output = append(nic.output, v)
	if len(nic.output) == 3 {
		nic.network.SendPacket(nic.address, nic.output[0], nic.output[1:3])
		nic.output = nic.output[3:]
	}
}
