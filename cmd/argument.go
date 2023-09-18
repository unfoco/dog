package cmd

import (
	"errors"
	"fmt"
	"reflect"
	"strconv"
	"strings"
)

// Line represents a command line holding command arguments that were passed upon the execution of the
// command. It is a convenience wrapper around a string slice.
type Line struct {
	args []string
}

// Next reads the next argument from the command line and returns it. If there were no more arguments to
// consume, false is returned.
func (line *Line) Next() (string, bool) {
	v, ok := line.NextN(1)
	if !ok {
		return "", false
	}
	return v[0], true
}

// NextN reads the next N arguments from the command line and returns them. If there were not enough arguments
// (n arguments), false is returned.
func (line *Line) NextN(n int) ([]string, bool) {
	if len(line.args) < n {
		return nil, false
	}
	v := line.args[:n]
	return v, true
}

// RemoveNext consumes the next argument from the command line.
func (line *Line) RemoveNext() {
	line.RemoveN(1)
}

// RemoveN consumes the next N arguments from the command line.
func (line *Line) RemoveN(n int) {
	if len(line.args) < n {
		line.args = nil
		return
	}
	line.args = line.args[n:]
}

// Leftover takes the leftover arguments from the command line.
func (line *Line) Leftover() []string {
	v := line.args
	line.args = nil
	return v
}

// Len returns the leftover length of the arguments in the command line.
func (line *Line) Len() int {
	return len(line.args)
}

// parser manages the parsing of a Line, turning the raw arguments into values which are then stored in the
// struct fields.
type parser struct {
	currentField string
}

// parseArgument parses the next argument from the command line passed and sets it to value v passed. If
// parsing was not successful, an error is returned.
func (p parser) parseArgument(line *Line, v reflect.Value, optional bool, name string) (error, bool) {
	var err error
	i := v.Interface()
	switch i.(type) {
	case int, int8, int16, int32, int64:
		err = p.int(line, v, name)
	case uint, uint8, uint16, uint32, uint64:
		err = p.uint(line, v, name)
	case float32, float64:
		err = p.float(line, v, name)
	case string:
		err = p.string(line, v)
	case bool:
		err = p.bool(line, v, name)
	case Varargs:
		err = p.varargs(line, v)
	case SubCommand:
		err = p.sub(line, name)
	default:
		if param, ok := i.(Parameter); ok {
			err = param.Parse(line, v)
			break
		}
		if enum, ok := i.(Enum); ok {
			err = p.enum(line, v, enum)
			break
		}
		panic(fmt.Sprintf("non-command parameter type %T in command structure", i))
	}
	if err == nil {
		// The argument was parsed successfully, so it needs to be removed from the command line.
		line.RemoveNext()
	} else if err == ErrInsufficientArgs && optional {
		// The command ran didn't have enough arguments for this parameter, but it was optional, so it does
		// not matter. Make sure to clear the value though.
		v.Set(reflect.Zero(v.Type()))
		return nil, false
	}
	return err, err == nil
}

// ErrInsufficientArgs is returned by argument parsing functions if it does not have sufficient arguments
// passed and is not optional.
var ErrInsufficientArgs = errors.New("not enough arguments for command")

// int ...
func (p parser) int(line *Line, v reflect.Value, name string) error {
	arg, ok := line.Next()
	if !ok {
		return ErrInsufficientArgs
	}
	value, err := strconv.ParseInt(arg, 10, v.Type().Bits())
	if err != nil {
		return fmt.Errorf(`cannot parse argument "%v" as type %v for argument "%v"`, arg, v.Kind(), name)
	}
	v.SetInt(value)
	return nil
}

// uint ...
func (p parser) uint(line *Line, v reflect.Value, name string) error {
	arg, ok := line.Next()
	if !ok {
		return ErrInsufficientArgs
	}
	value, err := strconv.ParseUint(arg, 10, v.Type().Bits())
	if err != nil {
		return fmt.Errorf(`cannot parse argument "%v" as type %v for argument "%v"`, arg, v.Kind(), name)
	}
	v.SetUint(value)
	return nil
}

// float ...
func (p parser) float(line *Line, v reflect.Value, name string) error {
	arg, ok := line.Next()
	if !ok {
		return ErrInsufficientArgs
	}
	value, err := strconv.ParseFloat(arg, v.Type().Bits())
	if err != nil {
		return fmt.Errorf(`cannot parse argument "%v" as type %v for argument "%v"`, arg, v.Kind(), name)
	}
	v.SetFloat(value)
	return nil
}

// string ...
func (p parser) string(line *Line, v reflect.Value) error {
	arg, ok := line.Next()
	if !ok {
		return ErrInsufficientArgs
	}
	v.SetString(arg)
	return nil
}

// bool ...
func (p parser) bool(line *Line, v reflect.Value, name string) error {
	arg, ok := line.Next()
	if !ok {
		return ErrInsufficientArgs
	}
	value, err := strconv.ParseBool(arg)
	if err != nil {
		return fmt.Errorf(`cannot parse argument "%v" as type bool for argument "%v"`, arg, name)
	}
	v.SetBool(value)
	return nil
}

// enum ...
func (p parser) enum(line *Line, val reflect.Value, v Enum) error {
	arg, ok := line.Next()
	if !ok {
		return ErrInsufficientArgs
	}
	found := ""
	for _, option := range v.Options() {
		if strings.EqualFold(option, arg) {
			found = option
		}
	}
	if found == "" {
		return fmt.Errorf(`invalid argument "%v" for enum parameter "%v"`, arg, v.Type())
	}
	val.SetString(found)
	return nil
}

// sub reads verifies a SubCommand against the next argument.
func (p parser) sub(line *Line, name string) error {
	arg, ok := line.Next()
	if !ok {
		return ErrInsufficientArgs
	}
	if strings.EqualFold(name, arg) {
		return nil
	}
	return fmt.Errorf(`invalid argument "%v" for sub command "%v"`, arg, name)
}

// varargs ...
func (p parser) varargs(line *Line, v reflect.Value) error {
	v.SetString(strings.Join(line.Leftover(), " "))
	return nil
}
