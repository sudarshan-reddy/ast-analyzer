package main

import (
	"fmt"
	"math"
)

type Oddity struct {
	value int
}

func (o *Oddity) EnigmaValidity() bool {
	if o.value < 2 {
		return false
	}
	for i := 2; float64(i) <= math.Sqrt(float64(o.value)); i++ {
		if o.value%i == 0 {
			return false
		}
	}
	return true
}

func main() {
	test := Oddity{value: 29}
	fmt.Printf("Is 29 something? %v\n", test.EnigmaValidity())
}
