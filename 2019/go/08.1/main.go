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

	minLayer := 0
	min0 := -1
	min1 := 0
	min2 := 0

	for i, l := range image.layers {
		num0 := l.CountDigits(0)
		num1 := l.CountDigits(1)
		num2 := l.CountDigits(2)

		if num0 < min0 || min0 < 0 {
			min0, min1, min2 = num0, num1, num2
			minLayer = i
		}
	}

	fmt.Printf("minLayer:%v  min0:%v  min1:%v  min2:%v\n", minLayer, min0, min1, min2)
	fmt.Printf("min1*min2: %v\n", min1*min2)
}
