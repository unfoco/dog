package dog

import (
	"context"
	"fmt"
	"os"
	"os/signal"
	"syscall"

	"github.com/disgoorg/disgo/bot"
	"github.com/disgoorg/disgo/discord"
	"github.com/disgoorg/disgo/handler"
	"github.com/disgoorg/json"
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
		bot.NewListenerFunc(d.onMessageCommand),
	)
}

func (d *Dog) initCommands() {
	cmd.Register(cmd.New("purge", "purges messages", nil, PurgeCommand{}))
	cmd.Init(d.client, "!")

	data := cmd.CommandsData()
	for name, _ := range d.config.Boards {
		data = append(data, discord.MessageCommandCreate{
			Name:                     fmt.Sprintf("Pin to %v", name),
			DefaultMemberPermissions: json.NewNullablePtr[discord.Permissions](discord.PermissionManageChannels),
		})
	}

	if err := handler.SyncCommands(d.client, data, []snowflake.ID{snowflake.MustParse("1153049076644991047")}); err != nil {
		log.Fatal("error while syncing commands: ", err)
	}
}
