package main

import (
	"github.com/disgoorg/disgo"
	"github.com/disgoorg/disgo/bot"
	"github.com/disgoorg/disgo/gateway"
	"github.com/disgoorg/log"
	"github.com/unfoco/dog/dog"
)

func main() {
	log.SetLevel(log.LevelInfo)

	cfg, err := dog.ReadConfig()
	if err != nil {
		log.Fatal("error while reading config: ", err)
	}

	cli, err := disgo.New(cfg.Token,
		bot.WithGatewayConfigOpts(
			gateway.WithIntents(
				gateway.IntentGuildMessages,
				gateway.IntentMessageContent,
				gateway.IntentGuildMessageReactions,
			),
		),
	)
	if err != nil {
		log.Fatal("error while building bot: ", err)
	}

	d := dog.New(cfg, cli)
	d.Start()
}
