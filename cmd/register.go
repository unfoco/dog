package cmd

import (
	"sync"

	"github.com/disgoorg/disgo/discord"
)

// commands holds a list of registered commands indexed by their name.
var commands sync.Map

// Register registers a command with its name and all aliases that it has. Any command with the same name or
// aliases will be overwritten.
func Register(command Command) {
	commands.Store(command.name, command)
	for _, alias := range command.aliases {
		commands.Store(alias, command)
	}
}

// ByAlias looks up a command by an alias. If found, the command and true are returned. If not, the returned
// command is nil and the bool is false.
func ByAlias(alias string) (Command, bool) {
	command, ok := commands.Load(alias)
	if !ok {
		return Command{}, false
	}
	return command.(Command), ok
}

// Commands returns a map of all registered commands indexed by the alias they were registered with.
func Commands() map[string]Command {
	cmd := make(map[string]Command)
	commands.Range(func(key, value any) bool {
		cmd[key.(string)] = value.(Command)
		return true
	})
	return cmd
}

func CommandsData() (list []discord.ApplicationCommandCreate) {
	for _, cmd := range Commands() {
		cmdData := discord.SlashCommandCreate{
			Name:        cmd.name,
			Description: cmd.description,
		}

		for _, pl := range cmd.Params() {
			options := []discord.ApplicationCommandOption{}
			sub := ""

			for _, p := range pl {
				var paramData discord.ApplicationCommandOption

				switch p.Value.(type) {
				case int, int8, int16, int32, int64:
					paramData = discord.ApplicationCommandOptionInt{
						Name:        p.Name,
						Description: p.Name,
						Required:    !p.Optional,
					}
				case uint, uint8, uint16, uint32, uint64:
					min := 0
					paramData = discord.ApplicationCommandOptionInt{
						Name:        p.Name,
						Description: p.Name,
						Required:    !p.Optional,
						MinValue:    &min,
					}
				case float32, float64:
					paramData = discord.ApplicationCommandOptionFloat{
						Name:        p.Name,
						Description: p.Name,
						Required:    !p.Optional,
					}
				case string:
					paramData = discord.ApplicationCommandOptionString{
						Name:        p.Name,
						Description: p.Name,
						Required:    !p.Optional,
					}
				case bool:
					paramData = discord.ApplicationCommandOptionBool{
						Name:        p.Name,
						Description: p.Name,
						Required:    !p.Optional,
					}
				case Varargs:
					paramData = discord.ApplicationCommandOptionString{
						Name:        p.Name,
						Description: p.Name,
						Required:    !p.Optional,
					}
				case []Target:
					paramData = discord.ApplicationCommandOptionUser{
						Name:        p.Name,
						Description: p.Name,
						Required:    !p.Optional,
					}
				case SubCommand:
					sub = p.Name
					continue
				}

				options = append(options, paramData)
			}

			if sub != "" {
				cmdData.Options = append(cmdData.Options, discord.ApplicationCommandOptionSubCommand{
					Name:        sub,
					Description: sub,
					Options:     options,
				})
			} else {
				//cmdData.Options = append(cmdData.Options, options...)
				cmdData.Options = append(cmdData.Options, discord.ApplicationCommandOptionSubCommand{
					Name:        "-",
					Description: "-",
					Options:     options,
				})
			}
		}

		list = append(list, cmdData)
	}
	return
}
