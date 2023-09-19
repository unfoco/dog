package cmd

import (
	"github.com/disgoorg/disgo/bot"
	"github.com/disgoorg/disgo/discord"
	"github.com/disgoorg/snowflake/v2"
)

type Context struct {
	Client    bot.Client
	GuildID   *snowflake.ID
	ChannelID snowflake.ID
	MessageID snowflake.ID
	UserID    snowflake.ID
	remove    bool
}

func (c *Context) Direct() bool {
	return c.GuildID == nil
}

func (c *Context) Remove() bool {
	return c.remove
}

func (c *Context) SetRemove(n bool) {
	c.remove = n
}

func (c *Context) HasPermUser(perm discord.Permissions) bool {
	return c.HasPerm(perm, false)
}

func (c *Context) HasPermChannel(perm discord.Permissions) bool {
	return c.HasPerm(perm, true)
}

func (c *Context) HasPerm(perm discord.Permissions, channel bool) bool {
	caches := c.Client.Caches()

	mem, ok := caches.Member(*c.GuildID, c.UserID)
	if !ok {
		return false
	}

	if channel {
		ch, ok := caches.Channel(c.ChannelID)
		if !ok {
			return false
		}

		return caches.
			MemberPermissionsInChannel(ch, mem).
			Has(perm)
	} else {
		return caches.
			MemberPermissions(mem).
			Has(perm)
	}
}
