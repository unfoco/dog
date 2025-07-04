use poise::serenity_prelude as serenity;
use regex::Regex;

use std::sync::LazyLock;

use crate::types;

const GITHUB_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"https://github\.com/(?P<user>[^/]+)/(?P<repo>[^/]+)/blob/(?P<path>.+?)/(?P<file>[^/]+)\.(?P<extension>[^#]+)#L(?P<start_line>\d+)(?:-L(?P<end_line>\d+))?").unwrap()
});

pub async fn handle(
    ctx: &serenity::Context,
    _framework: types::ContextFramework<'_>,
    _data: &types::Data,
    message: &serenity::Message,
) -> Result<(), types::Error> {
    if message.author.bot {
        return Ok(());
    }

    let Some(captures) = GITHUB_REGEX.captures(&message.content) else {
        return Ok(());
    };

    let user = &captures["user"];
    let repo = &captures["repo"];
    let path = &captures["path"];
    let file = &captures["file"];
    let extension = &captures["extension"];
    let start_line = &captures["start_line"];
    let end_line = captures
        .name("end_line")
        .map(|m| m.as_str())
        .unwrap_or(start_line);

    let url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}/{}.{}",
        user, repo, path, file, extension
    );

    let res = reqwest::get(&url).await?;
    let res_text = res.text().await?;

    let base = start_line.parse::<i32>().unwrap();
    let end = end_line.parse::<i32>().unwrap();

    if end == base {
        let mut start = base - 5;
        let end = base + 5;

        if start < 0 {
            start = 0;
        }

        let mut lines = res_text
            .lines()
            .enumerate()
            .filter(|(i, _)| {
                *i >= start as usize && *i <= end as usize
            });

        let mut result = String::new();
        while let Some((i, line)) = lines.next() {
            result.push_str(&format!("{}: {}\n", i+1, line));
        }

        message.reply(
            ctx, format!("```{}\n{}\n```", extension, result)
        ).await?;
    } else {
        let mut lines = res_text
            .lines()
            .enumerate()
            .filter(|(i, _)| {
                *i + 1 >= base as usize && *i + 1 <= end as usize
            });

        let mut result = String::new();
        while let Some((i, line)) = lines.next() {
            result.push_str(&format!("{}: {}\n", i+1, line));
        }

        message.reply(
            ctx, format!("```{}\n{}\n```", extension, result)
        ).await?;
    }

    Ok(())
}
