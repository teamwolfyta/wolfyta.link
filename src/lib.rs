use worker::{Context, Env, Request, Response, Result, Url, event};

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
  let original = req.url()?;

  let mut segments = match original.path_segments() {
    Some(segments) => segments,
    None => {
      return Response::error(
        "Oh no! The URL path seems to have gone missing in action. Please try again!",
        500,
      );
    }
  };

  let url = match resolve_url(&mut segments) {
    Some(url) => url,
    None => {
      return Response::error(
        "Gomenasai! I couldn't find the link you're looking for. Please check again!",
        404,
      );
    }
  };

  Response::redirect_with_status(Url::parse(url)?, 307)
}

fn resolve_url<'a>(segments: &mut impl Iterator<Item = &'a str>) -> Option<&'static str> {
  let service = segments.next()?;

  match service {
    "anilist" => Some("https://anilist.co/user/TeamWolfyta"),
    "bungie" => Some("https://www.bungie.net/7/en/User/Profile/1/4611686018451601257"),
    "github" => Some("https://github.com/TeamWolfyta"),
    "kitsu" => Some("https://kitsu.app/users/1449320"),
    "myanimelist" => Some("https://myanimelist.net/profile/TeamWolfyta"),
    "reddit" => Some("https://www.reddit.com/user/TEAMGAM3R"),
    "spotify" => Some("https://open.spotify.com/user/kieran0203"),
    "steam" => Some("https://steamcommunity.com/id/TeamWolfyta"),
    "twitch" => Some("https://www.twitch.tv/teamwolfyta"),
    "x" => Some("https://x.com/TeamWolfyta"),
    "youtube" => Some("https://www.youtube.com/channel/UCwSrKBEtN_sktfZM1bIVfoA"),

    "osu" => resolve_osu_links(segments),
    "discourse" => resolve_discourse_links(segments),
    "discord" => resolve_discord_links(segments),

    _ => None,
  }
}

fn resolve_osu_links<'a>(segments: &mut impl Iterator<Item = &'a str>) -> Option<&'static str> {
  match segments.next() {
    None => Some("https://osu.ppy.sh/users/17322353"),
    Some("wolfyta") => Some("https://osu.ppy.sh/teams/27171"),
    _ => None,
  }
}

fn resolve_discourse_links<'a>(
  segments: &mut impl Iterator<Item = &'a str>,
) -> Option<&'static str> {
  match segments.next() {
    Some("nixos") => Some("https://discourse.nixos.org/u/teamwolfyta"),
    _ => None,
  }
}

fn resolve_discord_links<'a>(segments: &mut impl Iterator<Item = &'a str>) -> Option<&'static str> {
  match segments.next() {
    None => Some("https://discord.com/users/193657209660375040"),
    Some("wolfyta") => Some("https://discord.gg/j4KmJ26ygZ"),
    Some("wolfyta-info-hub") => Some("https://discord.gg/w3t3kBkNpq"),
    Some("wolfyta-private") => Some("https://discord.gg/bJ6N7Jgybv"),
    Some("wolfyta-testing-hub") => Some("https://discord.gg/v9CuEPJARq"),
    _ => None,
  }
}
