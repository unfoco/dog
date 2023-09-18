package cmd

import "github.com/disgoorg/snowflake/v2"

type Context struct {
	GuildID   snowflake.ID
	ChannelID snowflake.ID
}
