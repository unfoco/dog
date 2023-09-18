package dog

import (
	"github.com/disgoorg/disgo/discord"
	"github.com/unfoco/dog/cmd"
)

type TestCommand struct {
	First  int    `cmd:"first"`
	Second string `cmd:"second"`
}

func (t TestCommand) Run(ctx *cmd.Context, b *discord.MessageCreateBuilder) {
	embed := discord.NewEmbedBuilder().
		SetDescriptionf("%v %v", t.First, t.Second).
		Build()
	b.AddEmbeds(embed)
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
