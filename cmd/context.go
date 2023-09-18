package cmd

import (
	"github.com/disgoorg/disgo/bot"
	"github.com/disgoorg/snowflake/v2"
)

type Context struct {
	Client    bot.Client
	GuildID   snowflake.ID
	ChannelID snowflake.ID
	remove    bool
}

func (c *Context) Remove() bool {
	return c.remove
}

func (c *Context) SetRemove(n bool) {
	c.remove = n
}
