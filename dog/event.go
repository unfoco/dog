package dog

import (
	"github.com/disgoorg/disgo/discord"
	"github.com/disgoorg/disgo/events"
	"github.com/disgoorg/snowflake/v2"
)

func (d *Dog) onReactionAdd(event *events.GuildMessageReactionAdd) {
	boards := d.config.Boards

	for emoji, channel := range boards {
		if event.Emoji.Reaction() != emoji {
			continue
		}

		msg, err := event.Client().Rest().GetMessage(event.ChannelID, event.MessageID)
		if err != nil {
			return
		}

		embed := discord.NewEmbedBuilder().
			SetAuthor(msg.Author.Username, "", *msg.Author.AvatarURL()).
			SetDescription(msg.Content).
			AddField("source", discord.MessageURL(event.GuildID, event.ChannelID, event.MessageID), false)

		_, _ = event.Client().Rest().CreateMessage(
			snowflake.MustParse(channel),
			discord.
				NewMessageCreateBuilder().
				AddEmbeds(embed.Build()).
				Build(),
		)
	}
}
