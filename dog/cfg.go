package dog

import (
	"encoding/json"
	"fmt"
	"os"
)

type Config struct {
	Token  string            `json:"token"`
	Admin  string            `json:"admin"`
	Boards map[string]string `json:"boards"`
}

func DefaultConfig() Config {
	return Config{
		Token:  "your-token-here",
		Boards: map[string]string{},
	}
}

func ReadConfig() (Config, error) {
	c := DefaultConfig()
	var zero Config
	if _, err := os.Stat("config.json"); os.IsNotExist(err) {
		data, err := json.Marshal(c)
		if err != nil {
			return zero, fmt.Errorf("encode default config: %v", err)
		}
		if err := os.WriteFile("config.json", data, 0644); err != nil {
			return zero, fmt.Errorf("create default config: %v", err)
		}
		return c, nil
	}
	data, err := os.ReadFile("config.json")
	if err != nil {
		return zero, fmt.Errorf("read config: %v", err)
	}
	if err := json.Unmarshal(data, &c); err != nil {
		return zero, fmt.Errorf("decode config: %v", err)
	}
	return c, nil
}
