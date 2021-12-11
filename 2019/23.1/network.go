package main

import (
	"fmt"
	"sync"
)

type Network struct {
	sync.Mutex
	queues []*NetworkQueue
	nat    []int64
}

type NetworkQueue struct {
	sync.Mutex
	idleCount  int
	address    int64
	emptyCount int64
	packets    [][]int64
}

func newNetwork() *Network {
	return &Network{}
}

func (n *Network) SendPacket(from, to int64, packet []int64) {
	fmt.Printf("SendPacket(%v, %v, %v)\n", from, to, packet)

	fromQ := n.findQueue(from)
	fromQ.Lock()
	fromQ.idleCount = 0
	defer fromQ.Unlock()

	if to == 255 {
		n.nat = packet
		return
	}

	toQ := n.findQueue(to)
	toQ.Lock()
	toQ.idleCount = 0
	defer toQ.Unlock()

	toQ.packets = append(toQ.packets, packet)
}

func (n *Network) RecvPacket(address int64) (ret []int64) {
	queue := n.findQueue(address)
	queue.Lock()
	defer func() {
		queue.Unlock()
		if ret != nil {
			fmt.Printf("GetPacket(%v) = %v\n", address, ret)
		}

		if address == 0 {
			ret = n.checkNAT()
		}
	}()

	if len(queue.packets) == 0 {
		queue.idleCount++
		return nil
	}

	ret = queue.packets[0]
	queue.packets = queue.packets[1:]
	return ret
}

func (n *Network) findQueue(address int64) *NetworkQueue {
	n.Lock()
	defer n.Unlock()

	for _, q := range n.queues {
		if q.address == address {
			return q
		}
	}

	q := &NetworkQueue{address: address}
	n.queues = append(n.queues, q)
	return q
}

func (n *Network) checkNAT() []int64 {
	n.Lock()
	defer n.Unlock()

	if len(n.nat) == 0 {
		return nil
	}

	fmt.Printf("Checking NAT\n")

	for _, q := range n.queues {
		if q.idleCount < 1 {
			return nil
		}
	}

	// All idle
	fmt.Printf("NAT Kicking in: %v\n", n.nat)
	ret := n.nat
	n.nat = nil
	return ret
}
