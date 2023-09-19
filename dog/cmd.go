package dog

import (
	"github.com/disgoorg/disgo/discord"
	"github.com/disgoorg/snowflake/v2"
	"github.com/unfoco/dog/cmd"
)

type PurgeCommand struct {
	Count uint `cmd:"count"`
}

func (t PurgeCommand) Allow(ctx *cmd.Context) bool {
	return true //ctx.HasPermUser(discord.PermissionManageChannels)
}

func (t PurgeCommand) Run(ctx *cmd.Context, b *discord.MessageCreateBuilder) {
	if t.Count > 100 {
		b.SetContent("you can't purge more than 100 message at once")
		return
	}

	list, err := ctx.Client.Rest().GetMessages(
		ctx.ChannelID,
		snowflake.ID(0),
		ctx.MessageID,
		snowflake.ID(0),
		int(t.Count)+1,
	)
	if err != nil {
		b.SetContent("unable to retrieve messages")
		return
	}

	var ids []snowflake.ID
	for _, v := range list {
		ids = append(ids, v.ID)
	}

	err = ctx.Client.Rest().BulkDeleteMessages(ctx.ChannelID, ids)
	if err != nil {
		b.SetContent("unable to purge messages")
	} else {
		b.SetContentf("purged %v messages", len(ids))
	}
}
