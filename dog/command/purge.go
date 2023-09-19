package command

import (
	"github.com/disgoorg/disgo/discord"
	"github.com/disgoorg/snowflake/v2"
	"github.com/unfoco/dog/cmd"
)

type Purge struct {
	Count uint `cmd:"count"`
}

func (p Purge) Allow(ctx *cmd.Context) bool {
	return true //ctx.HasPermUser(discord.PermissionManageChannels)
}

func (p Purge) Run(ctx *cmd.Context, b *discord.MessageCreateBuilder) {
	if p.Count > 100 {
		b.SetContent("you can't purge more than 100 message at once")
		return
	}

	ids, err := p.GetMessageIds(ctx, p.Count)
	if err != nil {
		b.SetContent("unable to retrieve messages")
		return
	}

	err = ctx.Client.Rest().BulkDeleteMessages(ctx.ChannelID, ids)
	if err != nil {
		b.SetContent("unable to purge messages")
	} else {
		b.SetContentf("purged %v messages", len(ids))
	}
}

func (Purge) GetMessageIds(ctx *cmd.Context, count uint) ([]snowflake.ID, error) {
	list, err := ctx.Client.Rest().GetMessages(
		ctx.ChannelID,
		snowflake.ID(0),
		ctx.MessageID,
		snowflake.ID(0),
		int(count)+1,
	)
	if err != nil {
		return nil, err
	}

	var ids []snowflake.ID
	for _, v := range list {
		ids = append(ids, v.ID)
	}
	return ids, nil
}
