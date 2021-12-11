// https://adventofcode.com/2019

package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func isPossiblePassword(number int64) bool {
	if number < 0 {
		return false
	}
	chars := strconv.FormatInt(number, 10)
	if len(chars) != 6 {
		return false
	}

	foundDouble := byte(0)
	foundDoubleCount := 0
	for i := 1; i < len(chars); i++ {
		if chars[i-1] == chars[i] {
			if chars[i-1] == foundDouble {
				foundDoubleCount++
			} else if foundDoubleCount != 2 {
				foundDouble = chars[i-1]
				foundDoubleCount = 2
			}
		} else if chars[i-1] >= chars[i] {
			return false
		}
	}

	return foundDoubleCount == 2
}

func findPossiblePasswords(low, high int64) []int64 {
	if low > high {
		low, high = high, low
	}
	fmt.Printf("low:%v  high:%v\n", low, high)

	var possiblePasswords []int64
	for i := low; i < high; i++ {
		if isPossiblePassword(i) {
			possiblePasswords = append(possiblePasswords, i)
		}
	}
	return possiblePasswords
}

func main() {
	s := bufio.NewScanner(os.Stdin)
	s.Scan()
	numberStrings := strings.Split(s.Text(), "-")

	if len(numberStrings) != 2 {
		fmt.Printf("Needed 2 numbers, got %v\n", numberStrings)
		return
	}

	var numbers []int64
	for _, numberString := range numberStrings {
		number, err := strconv.ParseInt(numberString, 10, 64)
		if err != nil {
			fmt.Printf("ERROR: Cannot convert %q to number: %v\n", numberString, err)
			return
		}
		numbers = append(numbers, number)
	}

	possiblePasswords := findPossiblePasswords(numbers[0], numbers[1])
	fmt.Printf("Total Possible Numbers: %v\n", len(possiblePasswords))
}
