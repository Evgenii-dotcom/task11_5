package main

import (
	"fmt"
	"log"
	"os"
)

func readData(path string) (string, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return "", fmt.Errorf("read error: %w", err)
	}
	return string(data), nil
}

func main() {
	content, err := readData("/data/shared.txt")
	if err != nil {
		log.Println(err)
		return
	}

	fmt.Println(content)
}