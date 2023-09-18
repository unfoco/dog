package cmd

import (
	"strconv"
	"strings"

	"github.com/disgoorg/disgo/bot"
	"github.com/disgoorg/disgo/discord"
	"github.com/disgoorg/disgo/events"
)

var pfx string

func Init(cli bot.Client, prefix string) {
	pfx = prefix

	cli.AddEventListeners(
		bot.NewListenerFunc(onMessageCommand),
		bot.NewListenerFunc(onSlashCommand),
	)
}

func onMessageCommand(event *events.MessageCreate) {
	msg := event.Message

	if msg.Author.Bot || !strings.HasPrefix(msg.Content, pfx) {
		return
	}

	ctx := Context{
		Client:    event.Client(),
		GuildID:   *event.GuildID,
		ChannelID: event.ChannelID,
	}

	args := strings.Split(msg.Content, " ")
	resp := onCommand(args[0][len(pfx):], args[1:], &ctx)

	if ctx.remove {
		event.Client().Rest().
			DeleteMessage(event.ChannelID, event.MessageID)
	}

	event.Client().Rest().
		CreateMessage(event.ChannelID, resp)
}

func onSlashCommand(event *events.ApplicationCommandInteractionCreate) {
	args := slashToArgs(event.SlashCommandInteractionData())
	resp := onCommand(args[0], args[1:], &Context{
		Client:    event.Client(),
		GuildID:   *event.GuildID(),
		ChannelID: event.Channel().ID(),
	})

	event.CreateMessage(resp)
}

func onCommand(name string, args []string, ctx *Context) (resp discord.MessageCreate) {
	cmd, ok := ByAlias(name)

	if ok {
		if out, err := cmd.Execute(strings.Join(args, " "), ctx); err != nil {
			resp = discord.NewMessageCreateBuilder().
				SetContentf(err.Error()).
				Build()
		} else {
			resp = out.Build()
		}
	} else {
		resp = discord.NewMessageCreateBuilder().
			SetContent("Please check that the command exists and that you have permission to use it.").
			Build()
	}
	return
}

func slashToArgs(data discord.SlashCommandInteractionData) (args []string) {
	args = append(args, data.CommandName())

	if n := data.SubCommandGroupName; n != nil && *n != "-" {
		args = append(args, *n)
	}
	if n := data.SubCommandName; n != nil && *n != "-" {
		args = append(args, *n)
	}
	for _, o := range data.Options {
		switch o.Type {
		case discord.ApplicationCommandOptionTypeString:
			args = append(args, data.String(o.Name))
		case discord.ApplicationCommandOptionTypeInt:
			args = append(args, strconv.Itoa(data.Int(o.Name)))
		case discord.ApplicationCommandOptionTypeBool:
			args = append(args, strconv.FormatBool(data.Bool(o.Name)))
		case discord.ApplicationCommandOptionTypeUser:
			args = append(args, data.User(o.Name).Mention())
		case discord.ApplicationCommandOptionTypeChannel:
			args = append(args, data.Channel(o.Name).ID.String())
		case discord.ApplicationCommandOptionTypeRole:
			args = append(args, data.Role(o.Name).Mention())
		case discord.ApplicationCommandOptionTypeFloat:
			args = append(args, strconv.FormatFloat(data.Float(o.Name), 'f', 2, 64))
		case discord.ApplicationCommandOptionTypeAttachment:
			args = append(args, data.Attachment(o.Name).ID.String())
		}
	}

	return
}
