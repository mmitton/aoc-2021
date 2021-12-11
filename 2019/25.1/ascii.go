package main

import (
	"fmt"
	"strings"
)

var ignoreItems = map[string]bool{
	"giant electromagnet": true,
	"molten lava":         true,
	"photons":             true,
	"infinite loop":       true,
	"escape pod":          true,
}

type ASCII struct {
	ic *Intcode

	currentRoom *Room
	rooms       []*Room

	state      State
	outputLine string
	inputs     string
	roomPath   []*Room
	items      Items
	finalItems Items
	itemsGuess []bool
}

type Items []string

type State uint8

const (
	StateNone State = iota
	StateDoors
	StateItems
	StateCommand
)

type Room struct {
	name  string
	items Items
	doors map[string]*Room
	x, y  int
}

func newASCII(program []int64) *ASCII {
	a := &ASCII{ic: newIntcode(program)}
	a.ic.Input = a.Input
	a.ic.Output = a.Output
	a.state = StateNone

	a.currentRoom = a.newRoom(0, 0)

	return a
}

func (a *ASCII) newRoom(x, y int) *Room {
	room := &Room{x: x, y: y}
	room.doors = make(map[string]*Room)
	a.rooms = append(a.rooms, room)
	return room
}

func (a *ASCII) getPath(isTarget func(r *Room) bool) []*Room {
	var paths = [][]*Room{[]*Room{a.currentRoom}}

	seen := make(map[*Room]bool)
	seen[a.currentRoom] = true
	for len(paths) > 0 {
		path := paths[0]
		paths = paths[1:]
		currentRoom := path[len(path)-1]
		if isTarget(currentRoom) {
			return path[1:]
		}

		for _, room := range currentRoom.doors {
			if seen[room] {
				continue
			}
			var newPath []*Room
			newPath = append(newPath, path...)
			newPath = append(newPath, room)
			seen[room] = true
			paths = append(paths, newPath)
		}
	}

	return nil
}

func (a *ASCII) Input() int64 {
	if len(a.inputs) == 0 {
		if len(a.roomPath) == 0 {
			// Find next room to explore
			a.roomPath = a.getPath(func(r *Room) bool { return r.name == "" })
		}
		if len(a.roomPath) == 0 {
			for _, r := range a.rooms {
				fmt.Println(r)
			}
			for idx, item := range a.items {
				fmt.Printf("Item[%v]:%q\n", idx, item)
			}
			if len(a.finalItems) == 0 {
				for _, item := range a.items {
					a.finalItems = append(a.finalItems, item)
					a.itemsGuess = append(a.itemsGuess, true)
				}
			} else {
				var commands []string
				idx := len(a.itemsGuess) - 1
				for idx >= 0 {
					a.itemsGuess[idx] = !a.itemsGuess[idx]
					if a.itemsGuess[idx] {
						// pick back up
						commands = append(commands, fmt.Sprintf("take %v\n", a.finalItems[idx]))
						idx--
					} else {
						// drop
						commands = append(commands, fmt.Sprintf("drop %v\n", a.finalItems[idx]))
						break
					}
				}
				if idx < 0 {
					panic("oops")
				}
				a.inputs = strings.Join(commands, "")
			}
			a.roomPath = a.getPath(func(r *Room) bool { return r.name == "Pressure-Sensitive Floor" })

			// if len(a.inputs) == 0 {
			// 	panic("nothing to explore")
			// }
		}

		if len(a.inputs) == 0 && len(a.roomPath) > 0 {
			nextRoom := a.roomPath[0]
			a.roomPath = a.roomPath[1:]
			if a.currentRoom.x == nextRoom.x && a.currentRoom.y > nextRoom.y {
				a.inputs = "north\n"
			} else if a.currentRoom.x == nextRoom.x && a.currentRoom.y < nextRoom.y {
				a.inputs = "south\n"
			} else if a.currentRoom.x < nextRoom.x && a.currentRoom.y == nextRoom.y {
				a.inputs = "east\n"
			} else if a.currentRoom.x > nextRoom.x && a.currentRoom.y == nextRoom.y {
				a.inputs = "west\n"
			}
			a.currentRoom = nextRoom
		}
	}

	if len(a.inputs) == 0 {
		panic("nothing to do")
	}

	var ret = a.inputs[0]
	a.inputs = a.inputs[1:]
	fmt.Printf("%c", ret)
	return int64(ret)
}

func (a *ASCII) Output(v int64) {
	r := rune(v)
	if r == '\n' {
		fmt.Println(a.outputLine)
		if len(a.outputLine) > 0 {
			if a.outputLine[0:3] == "== " {
				roomName := a.outputLine[3 : len(a.outputLine)-3]
				if a.currentRoom.name == "" {
					a.currentRoom.name = roomName
				} else if a.currentRoom.name != roomName {
					for _, r := range a.currentRoom.doors {
						if r.name == roomName {
							a.currentRoom = r
							break
						}
					}

					if a.currentRoom.name != roomName {
						panic("Bounced to a room we can't find")
					}
				}
				a.currentRoom.items = nil
			} else if a.outputLine == "Doors here lead:" {
				a.state = StateDoors
			} else if a.outputLine == "Items here:" {
				a.state = StateItems
			} else if strings.HasPrefix(a.outputLine, "You take the ") {
				item := a.outputLine[13 : len(a.outputLine)-1]
				a.currentRoom.items = a.currentRoom.items.remove(item)
				a.items = a.items.add(item)
			} else if a.outputLine == "Command?" {
				a.state = StateCommand
				if a.currentRoom.name != "Security Checkpoint" {
					var commands []string
					for _, item := range a.currentRoom.items {
						if !ignoreItems[item] {
							commands = append(commands, fmt.Sprintf("take %v\n", item))
						}
					}
					a.inputs = strings.Join(commands, "")
				}
			} else if a.state == StateDoors && a.outputLine[0:2] == "- " {
				door := a.outputLine[2:]
				if a.currentRoom.doors[door] == nil {
					var reverseDoor string
					nx, ny := a.currentRoom.x, a.currentRoom.y
					switch door {
					case "north":
						ny--
						reverseDoor = "south"
					case "south":
						ny++
						reverseDoor = "north"
					case "east":
						nx++
						reverseDoor = "west"
					case "west":
						nx--
						reverseDoor = "east"
					}
					newRoom := a.newRoom(nx, ny)
					a.currentRoom.doors[door] = newRoom
					newRoom.doors[reverseDoor] = a.currentRoom
				}
			} else if a.state == StateItems && a.outputLine[0:2] == "- " {
				var item = a.outputLine[2:]
				a.currentRoom.items = a.currentRoom.items.add(item)
			}
		}
		a.outputLine = ""
	} else {
		a.outputLine = fmt.Sprintf("%v%c", a.outputLine, r)
	}
}

func (a *ASCII) Run() error {
	return a.ic.run()
}

func (r *Room) String() string {
	var doors []string
	for dir, r := range r.doors {
		var name = r.name
		if r.name == "" {
			name = "UNEXPLORED"
		}
		doors = append(doors, fmt.Sprintf("%v:%q", dir, name))
	}

	return fmt.Sprintf("Room %v.  Items:%v  Doors:%v", r.name, r.items, doors)
}

func (i Items) add(item string) (ret Items) {
	for _, item2 := range i {
		if item2 == item {
			return i
		}
		ret = append(ret, item2)
	}
	ret = append(ret, item)
	return ret
}

func (i Items) remove(item string) (ret Items) {
	for _, item2 := range i {
		if item2 != item {
			ret = append(ret, item2)
		}
	}
	return ret
}
