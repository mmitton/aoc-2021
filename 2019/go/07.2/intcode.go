// https://adventofcode.com/2019

package main

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
	"strings"
	"unicode"
	"unicode/utf8"
)

type Intcode struct {
	memory []int64
	Input  func() int64
	Output func(v int64)
}

type OpCode byte
type AccessMode byte

const (
	PositionMode AccessMode = iota
	IndirectMode
	ImmediateMode
)

const (
	OpDone OpCode = 99
	OpAdd         = 1
	OpMul         = 2
	OpIn          = 3
	OpOut         = 4
	OpJmpT        = 5
	OpJmpF        = 6
	OpLT          = 7
	OpEQ          = 8
)

func newIntcode(program []int64) *Intcode {
	ic := &Intcode{}
	for _, v := range program {
		ic.memory = append(ic.memory, v)
	}

	ic.Input = func() int64 {
		var v int64
		fmt.Scanf("%d", &v)
		return v
	}

	ic.Output = func(v int64) {
		fmt.Println(v)
	}
	return ic
}

func (ic *Intcode) getMemory(mode AccessMode, addr int64) int64 {
	switch mode {
	case ImmediateMode:
		return ic.memory[addr]
	case PositionMode:
		return ic.memory[ic.memory[addr]]
	case IndirectMode:
		return ic.memory[ic.memory[ic.memory[addr]]]
	}

	return 0
}

func (ic *Intcode) setMemory(mode AccessMode, addr int64, v int64) {
	switch mode {
	case PositionMode:
		ic.memory[ic.memory[addr]] = v
	case IndirectMode:
		ic.memory[ic.memory[ic.memory[addr]]] = v
	}
}

func (ic *Intcode) patch(noun, verb int64) {
	ic.memory[1] = noun
	ic.memory[2] = verb
}

func (ic *Intcode) run() error {
	getMode := func(op, which int64) AccessMode {
		op = op / 100
		for which > 1 {
			op = op / 10
			which--
		}

		switch op % 10 {
		case 0:
			return PositionMode
		case 1:
			return ImmediateMode
		}

		return PositionMode
	}

	pc := int64(0)
	for {
		op := ic.getMemory(ImmediateMode, pc)
		mode1 := getMode(op, 1)
		mode2 := getMode(op, 2)
		mode3 := getMode(op, 3)
		switch OpCode(op % 100) {
		case OpDone:
			return nil
		case OpAdd:
			arg1 := ic.getMemory(mode1, pc+1)
			arg2 := ic.getMemory(mode2, pc+2)
			ic.setMemory(mode3, pc+3, arg1+arg2)
			pc += 4
		case OpMul:
			arg1 := ic.getMemory(mode1, pc+1)
			arg2 := ic.getMemory(mode2, pc+2)
			ic.setMemory(mode3, pc+3, arg1*arg2)
			pc += 4
		case OpIn:
			val := ic.Input()
			ic.setMemory(mode1, pc+1, val)
			pc += 2
		case OpOut:
			arg1 := ic.getMemory(mode1, pc+1)
			ic.Output(arg1)
			pc += 2
		case OpJmpT:
			arg1 := ic.getMemory(mode1, pc+1)
			if arg1 != 0 {
				pc = ic.getMemory(mode2, pc+2)
			} else {
				pc += 3
			}
		case OpJmpF:
			arg1 := ic.getMemory(mode1, pc+1)
			if arg1 == 0 {
				pc = ic.getMemory(mode2, pc+2)
			} else {
				pc += 3
			}
		case OpLT:
			arg1 := ic.getMemory(mode1, pc+1)
			arg2 := ic.getMemory(mode2, pc+2)
			if arg1 < arg2 {
				ic.setMemory(mode3, pc+3, 1)
			} else {
				ic.setMemory(mode3, pc+3, 0)
			}
			pc += 4
		case OpEQ:
			arg1 := ic.getMemory(mode1, pc+1)
			arg2 := ic.getMemory(mode2, pc+2)
			if arg1 == arg2 {
				ic.setMemory(mode3, pc+3, 1)
			} else {
				ic.setMemory(mode3, pc+3, 0)
			}
			pc += 4
		default:
			return fmt.Errorf("Bad opcode at %v! %v", pc, op)
		}
	}
	return nil
}

func readProgram(r io.Reader) ([]int64, error) {
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

	s := bufio.NewScanner(r)
	s.Split(scanInts)

	var program []int64
	for s.Scan() {
		t := strings.TrimSpace(s.Text())
		i, err := strconv.ParseInt(t, 10, 64)
		if err != nil {
			return nil, err
		} else {
			program = append(program, i)
		}
	}

	return program, nil
}

func newIntcodeFromReader(r io.Reader) (*Intcode, error) {
	program, err := readProgram(r)
	if err != nil {
		return nil, err
	}
	return newIntcode(program), nil
}
