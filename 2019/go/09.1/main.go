// https://adventofcode.com/2019

package main

import (
	"fmt"
	"os"
)

func main() {
	ic, err := newIntcodeFromReader(os.Stdin)
	if err != nil {
		fmt.Printf("Err %v\n", err)
	}

	ic.Input = func() int64 {
		return 1
	}

	ic.run()

}
