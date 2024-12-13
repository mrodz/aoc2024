package main

import (
	"bufio"
	"errors"
	"os"
	"strconv"
	"strings"
)

type NuclearReport struct {
	grid [][]uint8
}

func (n *NuclearReport) safeReports() ([]int, error) {
	var result []int

	for index, row := range n.grid {
		safe, err := IsRowSafe(row)

		if err != nil {
			return nil, err
		} else if !safe {
			continue
		}

		result = append(result, index)
	}

	return result, nil
}

func IsRowSafe(row []uint8) (bool, error) {
	if len(row) < 2 {
		return false, errors.New("cannot check the safety of a row of less than two elements")
	}

	// Determine increasing/decreasing, check edge case
	increasing := row[1] > row[0]

	difference := max(row[0], row[1]) - min(row[0], row[1])

	if difference < 1 && difference > 3 {
		return false, nil
	}

	// Index is for the previous element, data is current
	for index, data := range row[1:] {
		// Any two adjacent levels differ by at least one and at most three.
		difference := max(row[index+1], row[index]) - min(row[index+1], row[index])

		if difference < 1 || difference > 3 {
			return false, nil
		}

		// The levels are either all increasing or all decreasing.
		if increasing && data < row[index] {
			return false, nil
		}

		if !increasing && data > row[index] {
			return false, nil
		}
	}

	return true, nil
}

func ParseLinesFromFile(filePath string) (*NuclearReport, error) {
	file, err := os.Open(filePath)

	if err != nil {
		return nil, err
	}
	defer file.Close()

	scanner := bufio.NewScanner(file)

	var rows [][]uint8

	for scanner.Scan() {
		line := scanner.Text()

		var columns []uint8

		for _, numberString := range strings.Fields(line) {
			number, err := strconv.Atoi(numberString)

			if err != nil {
				return nil, err
			}

			columns = append(columns, uint8(number))
		}

		rows = append(rows, columns)
	}

	if err := scanner.Err(); err != nil {
		return nil, err
	}

	return &NuclearReport{
		grid: rows,
	}, nil
}
