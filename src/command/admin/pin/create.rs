use poise::serenity_prelude as serenity;
use serenity::Mentionable;
use serenity::json;

pub async fn handle(
    ctx: types::ContextApp<'_>,
    msg: serenity::Message,
) -> Result<(), types::Error> {
    let Some(result) = util::interactions::send_dropdown(
        ctx,
        "mesajı pinlemek istediğiniz panoyu seçiniz",
        ctx.data().config.boards.keys().collect(),
    )
    .await?
    else {
        return Ok(());
    };

    let board = ctx.data().config.boards.get(&result).cloned().unwrap();

    let webhook = board.webhook(ctx.http()).await?;

    let member = ctx
        .guild()
        .unwrap()
        .member(ctx.http(), msg.author.id)
        .await?;

    let name = member.display_name().clone();
    let avatar = member
        .avatar_url()
        .unwrap_or_else(|| msg.author.avatar_url().unwrap());

    webhook
        .execute(ctx.http(), true, |w| {
            w.username(&name)
                .avatar_url(&avatar)
                .content(msg.content.clone());

            for attachment in &msg.attachments {
                w.add_file(serenity::AttachmentType::Image(
                    url::Url::parse(&attachment.url).unwrap(),
                ));
            }

            let mut embeds: Vec<json::Value> = msg
                .embeds
                .iter()
                .map(|e| super::embed_to_json(e.clone()))
                .collect();

            embeds.push(serenity::Embed::fake(|e| {
                e.field("kaynak", msg.link(), true)
            }));

            w.embeds(embeds)
        })
        .await?;

    msg.reply(
        ctx.http(),
        format!("mesaj {} panosuna pinlendi", board.channel.mention(),),
    )
    .await?;

    log_sys!(
        ctx,
        "{} {} mesajını {} panosuna pinledi",
        ctx.author(),
        msg.link(),
        board.channel.mention()
    );
    Ok(())
}
