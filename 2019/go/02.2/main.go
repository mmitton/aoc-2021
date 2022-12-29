// https://adventofcode.com/2019

package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"unicode"
	"unicode/utf8"
)

type Intcode struct {
	program []int64
}

func newIntcode(program []int64) *Intcode {
	ic := &Intcode{}
	for _, m := range program {
		ic.program = append(ic.program, m)
	}
	return ic
}

func (ic *Intcode) getDirect(addr int64) int64 {
	// fmt.Printf("getDirect(%v) = %v\n", addr, ic.program[addr])
	return ic.program[addr]
}

func (ic *Intcode) getIndirect(addr int64) int64 {
	//fmt.Printf("getIndirect(%v) = %v => %v\n", addr, ic.program[addr], ic.program[ic.program[addr]])
	return ic.program[ic.program[addr]]
}

func (ic *Intcode) setIndirect(addr int64, v int64) {
	//fmt.Printf("setDirect(%v, %v) = %v => %v\n", addr, v, ic.program[addr], ic.program[ic.program[addr]])
	ic.program[ic.program[addr]] = v
}

func (ic *Intcode) patch(noun, verb int64) {
	ic.program[1] = noun
	ic.program[2] = verb
}

func (ic *Intcode) run() error {
	pc := int64(0)
	for {
		op := ic.getDirect(pc)
		switch op {
		case 99:
			return nil
		case 1:
			arg1 := ic.getIndirect(pc + 1)
			arg2 := ic.getIndirect(pc + 2)
			ic.setIndirect(pc+3, arg1+arg2)
			pc += 4
		case 2:
			arg1 := ic.getIndirect(pc + 1)
			arg2 := ic.getIndirect(pc + 2)
			ic.setIndirect(pc+3, arg1*arg2)
			pc += 4
		default:
			return fmt.Errorf("Bad opcode at %v! %v", pc, op)
		}
	}
	return nil
}

func main() {
	scanInts := func(data []byte, atEOF bool) (advance int, token []byte, err error) {
		// Skip leading spaces.
		start := 0
		for width := 0; start < len(data); start += width {
			var r rune
			r, width = utf8.DecodeRune(data[start:])
			if !unicode.IsSpace(r) {
				break
			}
		}
		// Scan until space, marking end of word.
		for width, i := 0, start; i < len(data); i += width {
			var r rune
			r, width = utf8.DecodeRune(data[i:])
			if r == ',' {
				return i + width, data[start:i], nil
			}
		}
		// If we're at EOF, we have a final, non-empty, non-terminated word. Return it.
		if atEOF && len(data) > start {
			return len(data), data[start:], nil
		}
		// Request more data.
		return start, nil, nil
	}

	s := bufio.NewScanner(os.Stdin)
	s.Split(scanInts)

	var program []int64
	for s.Scan() {
		t := strings.TrimSpace(s.Text())
		i, err := strconv.ParseInt(t, 10, 64)
		if err != nil {
			fmt.Printf("%q: %v", t, err)
			return
		} else {
			program = append(program, i)
		}
	}

	for noun := int64(0); noun < 100; noun++ {
		for verb := int64(0); verb < 100; verb++ {
			ic := newIntcode(program)
			ic.patch(noun, verb)

			err := ic.run()
			if err != nil {
				fmt.Printf("%v %v: Err %v\n", noun, verb, err)
			} else {
				if ic.program[0] == 19690720 {
					fmt.Printf("Result: %v\n", (noun*100)+verb)
					return
				}
			}
		}
	}
}
