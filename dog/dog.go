package dog

import (
	"context"
	"os"
	"os/signal"
	"syscall"

	"github.com/disgoorg/disgo/bot"
	"github.com/disgoorg/disgo/handler"
	"github.com/disgoorg/log"
	"github.com/disgoorg/snowflake/v2"
	"github.com/unfoco/dog/cmd"
)

type Dog struct {
	config Config
	client bot.Client
}

func New(cfg Config, cli bot.Client) *Dog {
	return &Dog{
		config: cfg,
		client: cli,
	}
}

func (d *Dog) Start() {
	d.initEvents()
	d.initCommands()

	if err := d.client.OpenGateway(context.Background()); err != nil {
		log.Fatal("error while connecting to gateway: ", err)
	}
	defer d.client.Close(context.Background())

	log.Info("dog is ready for mission")

	s := make(chan os.Signal, 1)
	signal.Notify(s, syscall.SIGINT, syscall.SIGTERM, os.Interrupt)
	<-s
}

func (d *Dog) initEvents() {
	d.client.AddEventListeners(
		bot.NewListenerFunc(d.onReactionAdd),
	)
}

func (d *Dog) initCommands() {
	cmd.Register(cmd.New("test", "test command", nil, TestCommand{}, TestSubCommand{}))

	if err := handler.SyncCommands(d.client, cmd.CommandsData(), []snowflake.ID{snowflake.MustParse("1153049076644991047")}); err != nil {
		log.Fatal("error while syncing commands: ", err)
	}

	cmd.Init(d.client, "!")
}
