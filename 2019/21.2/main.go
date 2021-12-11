package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	program, err := readProgram(os.Stdin)
	if err != nil {
		fmt.Printf("Err %v\n", err)
		return
	}

	asm := []string{
		"NOT A T",
		"NOT B J",
		"OR T J",
		"NOT C T",
		"OR T J",
		"AND D J",
		"NOT E T",
		"NOT T T",
		"OR H T",
		"AND T J",
	}
	input := strings.Join(asm, "\n") + "\nRUN\n"

	ic := newIntcode(program)
	ic.Input = func() int64 {
		if input == "" {
			return 0
		}
		ret := input[0]
		input = input[1:]
		fmt.Printf("%c", ret)
		return int64(ret)
	}

	ic.Output = func(v int64) {
		if v < 255 {
			fmt.Printf("%c", v)
		} else {
			fmt.Printf("%d\n", v)
		}
	}

	if err := ic.run(); err != nil {
		fmt.Printf("\nERROR: %v\n", err)
		return
	}
}
