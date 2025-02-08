package main

import "fmt"

func main() {
	fmt.Println(loopadd(3))
}
func loopadd(n int) int {
	result := 0
	for i := 0; i <= n; i++ {
		result += i
	}
	return result
}
