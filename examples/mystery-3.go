package main

import "fmt"

type StringTwister struct {
	data string
}

func (st *StringTwister) TwistAndTurn() string {
	runes := []rune(st.data)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

func main() {
	st := StringTwister{data: "hello"}
	fmt.Printf("The twisted string is: %s\n", st.TwistAndTurn())
}
