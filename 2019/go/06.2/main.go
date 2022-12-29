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

func (e *Entity) String() string {
	return e.name
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

func (e *Entity) find(name string) *Entity {
	if e.name == name {
		return e
	}
	for _, c := range e.children {
		e2 := c.find(name)
		if e2 != nil {
			return e2
		}
	}

	return nil
}

func (e *Entity) findPath(fromName string, toName string) ([]*Entity, error) {
	find := func(name string) (*Entity, error) {
		e := e.find(name)
		if e == nil {
			return nil, fmt.Errorf("Cannot find entity %v", name)
		}
		if len(e.children) != 0 {
			return nil, fmt.Errorf("%v has children", name)
		}
		if e.parent == nil {
			return nil, fmt.Errorf("%v isn't orbiting anything", name)
		}
		return e, nil
	}
	from, err := find(fromName)
	if err != nil {
		return nil, err
	}
	to, err := find(toName)
	if err != nil {
		return nil, err
	}
	if to == from || to.parent == from.parent {
		return nil, nil
	}

	var paths = [][]*Entity{[]*Entity{from.parent}}
	checkLoop := func(path []*Entity, e *Entity) bool {
		if len(path) == 1 {
			return false
		}
		if path[len(path)-2] == e {
			return true
		}
		return false
	}
	addEdge := func(path []*Entity, e *Entity) {
		if e != nil && !checkLoop(path, e) {
			newPath := append([]*Entity(nil), path...)
			newPath = append(newPath, e)
			paths = append(paths, newPath)
		}
	}

	for i := 0; i < len(paths); i++ {
		path := paths[i]
		tail := path[len(path)-1]
		if tail == to {
			return path[1 : len(path)-1], nil
		}
		addEdge(path, tail.parent)
		for _, c := range tail.children {
			addEdge(path, c)
		}
	}

	return nil, fmt.Errorf("Cannot find path from %v to %v", fromName, toName)
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
	p, err := o.findPath("YOU", "SAN")
	if err != nil {
		fmt.Printf("ERROR: %v\n", err)
		return
	}

	fmt.Printf("Path from YOU to SAN is %v long\n", len(p))
}
