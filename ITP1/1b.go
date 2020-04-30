package main

import "fmt"

func main() {
	var num int
	if _, err := fmt.Scanln(&num); err != nil {
		panic(err)
	}
	fmt.Println(num * num * num)
}
