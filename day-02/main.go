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

	fmt.Printf("Analyze the unusual data from the engineers. How many reports are safe? The Answer is %d\n", safe)

	return nil
}

func PartTwo(report *NuclearReport) error {
	safeReports, err := report.safeReportsWithDampener()

	if err != nil {
		return err
	}

	safe := len(safeReports)

	fmt.Printf("Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports. How many reports are now safe? The Answer is %d", safe)

	return nil
}

func main() {
	report, err := ParseLinesFromFile("./input.txt")

	if err != nil {
		log.Fatal(err)
	}

	PartOne(report)
	PartTwo(report)
}
