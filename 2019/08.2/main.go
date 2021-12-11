// https://adventofcode.com/2019

package main

import (
	"fmt"
	"os"
)

func main() {
	image, err := decodeImage(os.Stdin, 25, 6)
	if err != nil {
		fmt.Printf("Err %v\n", err)
		return
	}

	fmt.Print(image.Render())
}
