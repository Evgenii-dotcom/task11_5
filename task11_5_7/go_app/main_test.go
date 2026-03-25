package main

import "testing"

func TestReadData(t *testing.T) {
	_, err := readData("nonexistent.txt")
	if err == nil {
		t.Error("Expected error")
	}
}