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

func (n *NuclearReport) safeReportsWithDampener() ([]int, error) {
	var result []int

	for index, row := range n.grid {
		safe, err := IsRowSafeWithDampener(row)

		if err != nil {
			return nil, err
		} else if !safe {
			continue
		}

		result = append(result, index)
	}

	return result, nil
}

func IsRowSafeWithDampener(row []uint8) (bool, error) {
	return isRowSafeWithDampenerRecursive(row, -1)
}

func processNeighbors(row []uint8, index int) (bool, error) {
	// Correct for:
	// A) "Any two adjacent levels differ by at least one and at most three."
	// B) "The levels are either all increasing or all decreasing." (divergence from established acceleration)
	if index > 0 {
		l, err := isRowSafeWithDampenerRecursive(row, index-1)
		if err != nil {
			return false, err
		}
		if l {
			return true, nil
		}
	}

	// Correct for:
	// A) "The levels are either all increasing or all decreasing." (possible faulty initial acceleration)
	if index == 2 {
		l, err := isRowSafeWithDampenerRecursive(row, 0)
		if err != nil {
			return false, err
		}
		if l {
			return true, nil
		}
		r, err := isRowSafeWithDampenerRecursive(row, 1)
		if err != nil {
			return false, err
		}
		if r {
			return true, nil
		}
	}

	m, err := isRowSafeWithDampenerRecursive(row, index)
	if err != nil {
		return false, err
	}
	return m, nil
}

func isRowSafeWithDampenerRecursive(row []uint8, skip int) (bool, error) {
	if len(row) < 2 {
		return false, errors.New("cannot check the safety of a row of less than two elements")
	}

	var acceleration int8

	start_index := 0

	if skip == 0 {
		start_index = 1
	}

	prev := row[start_index]

	for i := start_index + 1; i < len(row); i++ {
		if skip == i {
			continue
		}

		this := row[i]

		difference := max(this, prev) - min(this, prev)

		if difference < 1 || difference > 3 {
			if skip == -1 {
				return processNeighbors(row, i)
			}

			return false, nil
		}

		if acceleration == 0 {
			acceleration = int8(this) - int8(prev)
		}

		if acceleration > 0 && this < prev {
			if skip == -1 {
				return processNeighbors(row, i)
			}

			return false, nil
		}

		if acceleration < 0 && this > prev {
			if skip == -1 {
				return processNeighbors(row, i)
			}

			return false, nil
		}

		prev = this
	}

	return true, nil
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
