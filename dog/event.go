package dog

import (
	"strings"

	"github.com/disgoorg/disgo/discord"
	"github.com/disgoorg/disgo/events"
	"github.com/disgoorg/snowflake/v2"
	"golang.org/x/exp/slices"
)

func (d *Dog) onMessageCommand(event *events.ApplicationCommandInteractionCreate) {
	data, ok := event.Data.(discord.MessageCommandInteractionData)
	if !ok {
		return
	}

	admin := snowflake.MustParse(d.config.Admin)

	ok = slices.Contains(event.Member().RoleIDs, admin)
	if !ok {
		return
	}

	for name, board := range d.config.Boards {
		if !strings.HasSuffix(data.CommandName(), name) {
			continue
		}

		msg, err := event.Client().Rest().GetMessage(event.Channel().ID(), data.TargetID())
		if err != nil {
			return
		}

		embed := discord.NewEmbedBuilder().
			SetAuthor(msg.Author.Username, "", *msg.Author.AvatarURL()).
			SetDescription(msg.Content).
			AddField("source", discord.MessageURL(*event.GuildID(), event.Channel().ID(), data.TargetID()), false)

		_, _ = event.Client().Rest().CreateMessage(
			snowflake.MustParse(board),
			discord.
				NewMessageCreateBuilder().
				AddEmbeds(embed.Build()).
				Build(),
		)

		embed = discord.NewEmbedBuilder().
			SetDescriptionf("message pinned to board %v", name)

		event.CreateMessage(
			discord.NewMessageCreateBuilder().
				AddEmbeds(embed.Build()).
				Build(),
		)
	}
}
