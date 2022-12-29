// https://adventofcode.com/2019

package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"strings"
)

type Entity struct {
	name     string
	parent   *Entity
	children []*Entity
}

func newOrbits(defs []string) (*Entity, error) {
	var entities = make(map[string]*Entity)
	findEntity := func(name string) *Entity {
		e, ok := entities[name]
		if !ok {
			e = &Entity{name: name}
			entities[name] = e
		}
		return e
	}

	for _, def := range defs {
		parts := strings.Split(def, ")")
		parentName, childName := parts[0], parts[1]
		parent := findEntity(parentName)
		child := findEntity(childName)

		if child.parent != nil {
			return nil, fmt.Errorf("Cannot orbit %v around %v, already around %v", childName, parentName, child.parent.name)
		}
		child.parent = parent
		parent.children = append(parent.children, child)
	}

	for _, e := range entities {
		if e.parent == nil && e.name != "COM" {
			return nil, fmt.Errorf("Entity %v is not orbiting around anything!", e.name)
		}
	}

	var root = entities["COM"]
	if root == nil {
		return nil, errors.New("COM not referenced")
	}
	return root, nil
}

func (e *Entity) calcChecksum() int {
	cs := 0
	for p := e.parent; p != nil; p = p.parent {
		cs++
	}
	for _, c := range e.children {
		cs += c.calcChecksum()
	}
	return cs
}

func main() {
	s := bufio.NewScanner(os.Stdin)
	var def []string
	for s.Scan() {
		def = append(def, s.Text())
	}

	o, err := newOrbits(def)
	if err != nil {
		fmt.Printf("ERROR: %v\n", err)
		return
	}

	fmt.Printf("Orbit Checksum: %v\n", o.calcChecksum())
}
