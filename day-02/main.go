package main

import (
	"fmt"
	"log"
)

func PartOne(report *NuclearReport) error {
	safeReports, err := report.safeReports()

	if err != nil {
		return err
	}

	safe := len(safeReports)

	fmt.Printf("Analyze the unusual data from the engineers. How many reports are safe? %d", safe)

	return nil
}

func main() {
	report, err := ParseLinesFromFile("./input.txt")

	if err != nil {
		log.Fatal(err)
	}

	PartOne(report)
}
