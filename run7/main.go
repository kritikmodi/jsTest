package main

import (
	"github.com/suborbital/reactr/api/tinygo/runnable" 
)

type Run7 struct{}

func (h Run7) Run(input []byte) ([]byte, error) {
	return []byte("Hello, " + string(input)), nil
}

// initialize runnable, do not edit //
func main() {
	runnable.Use(Run7{})
}
