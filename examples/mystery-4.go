package main

import "fmt"

type NumberCruncher struct {
	number int
}

func (nc *NumberCruncher) RecursiveDigitSum() int {
	return nc.auxiliarySum(nc.number)
}

func (nc *NumberCruncher) auxiliarySum(n int) int {
	if n == 0 {
		return 0
	}
	return n%10 + nc.auxiliarySum(n/10)
}

func main() {
	nc := NumberCruncher{number: 12345}
	fmt.Printf("The sum of digits is: %d\n", nc.RecursiveDigitSum())
}
