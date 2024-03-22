package main

import "fmt"

type NumberManipulator struct {
    n int
}

func (nm *NumberManipulator) do() int {
    a, b := 0, 1
    for i := 0; i < nm.n; i++ {
        a, b = b, a+b
    }
    return a
}

func main() {
    nm := NumberManipulator{n: 10}
    fmt.Printf("The 10th number in the Fibonacci sequence is: %d\n", nm.do())
}
