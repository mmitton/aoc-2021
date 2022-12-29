package main

import (
	"fmt"
	"strconv"
	"strings"
)

type NanoFactory struct {
	chemicals map[string]*Chemical
}

type Chemical struct {
	name     string
	required int
	reaction string
	inputs   map[*Chemical]int
	output   int
	used     int
}

func newNanoFactory(reactions []string) (*NanoFactory, error) {
	parseNumberChemical := func(v string) (int, string, error) {
		v = strings.TrimSpace(v)
		parts := strings.Split(v, " ")
		if len(parts) != 2 {
			return 0, "", fmt.Errorf("%v: Expected 'Number Chemical'", v)
		}
		c, err := strconv.ParseInt(parts[0], 10, 64)
		if err != nil {
			return 0, "", fmt.Errorf("%v: %q is not a number", v, parts[0])
		}

		return int(c), parts[1], nil
	}

	nf := &NanoFactory{chemicals: make(map[string]*Chemical)}
	var fuel *Chemical
	for _, reaction := range reactions {
		inOut := strings.Split(reaction, "=>")
		if len(inOut) != 2 {
			return nil, fmt.Errorf("%v: No => found", reaction)
		}

		outNum, outName, err := parseNumberChemical(inOut[1])
		if err != nil {
			return nil, fmt.Errorf("%v: %v", reaction, err)
		}

		if outName == "ORE" {
			return nil, fmt.Errorf("%v: Cannot define reaction to make ORE", reaction)
		}

		chem := nf.findChemical(outName)
		if chem.output != 0 {
			return nil, fmt.Errorf("%v: %v has multiple reactions", reaction, outName)
		}
		chem.output = outNum
		chem.reaction = reaction

		if outName == "FUEL" {
			fuel = chem
		}

		inputs := strings.Split(inOut[0], ",")
		for _, input := range inputs {
			inNum, inName, err := parseNumberChemical(input)
			if err != nil {
				return nil, fmt.Errorf("%v: %v", reaction, err)
			}

			chem.inputs[nf.findChemical(inName)] = inNum
		}

		if len(chem.inputs) == 0 {
			return nil, fmt.Errorf("%v: No inputs defined", reaction)
		}
	}

	if fuel == nil {
		return nil, fmt.Errorf("No reaction to make FUEL")
	}

	fuel.make(1)

	return nf, nil
}

func (nf *NanoFactory) findChemical(name string) *Chemical {
	chem := nf.chemicals[name]
	if chem != nil {
		return chem
	}

	chem = &Chemical{name: name, inputs: make(map[*Chemical]int)}
	nf.chemicals[name] = chem
	return chem
}

func (chem *Chemical) make(n int) {
	if chem.name == "ORE" {
		chem.required += n
		return
	}
	for chem.used+n > chem.required {
		chem.required += chem.output
		for chem, needed := range chem.inputs {
			chem.make(needed)
		}
	}
	chem.used += n
}

func (nf *NanoFactory) OREtoFUEL(totalOre int) int {
	fuel := findChemical("FUEL")
	ore := findChemical("ORE")
	for ore.required < totalOre {
		fuel.make(1)
	}

	return fuel.required - 1
}
