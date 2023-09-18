package dog

import (
	"github.com/disgoorg/disgo/discord"
	"github.com/unfoco/dog/cmd"
)

type TestCommand struct {
	Yay bool `cmd:"yay"`
}

func (t TestCommand) Run(ctx *cmd.Context, b *discord.MessageCreateBuilder) {
	embed := discord.NewEmbedBuilder().
		SetDescriptionf("%v", t.Yay).
		Build()
	b.AddEmbeds(embed)
	ctx.SetRemove(true)
}

type TestSubCommand struct {
	Test  cmd.SubCommand `cmd:"test"`
	Third int            `cmd:"third"`
}

func (t TestSubCommand) Run(ctx *cmd.Context, b *discord.MessageCreateBuilder) {
	embed := discord.NewEmbedBuilder().
		SetDescriptionf("%#v", cmd.CommandsData()).
		Build()
	b.AddEmbeds(embed)
}
