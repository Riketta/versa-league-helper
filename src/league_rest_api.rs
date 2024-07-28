const USER_AGENT: &str = "LeagueOfLegendsClient/14.5.563.9790 (CEF 91)";

struct RiotClientAPICredentials {}

struct LeagueClientAPICredentials {}

struct LeagueAPI {
    riot_client_api_credentials: RiotClientAPICredentials,
    league_client_api_credentials: LeagueClientAPICredentials,
}

impl LeagueAPI {
    fn new(
        riot_client_api_credentials: RiotClientAPICredentials,
        league_client_api_credentials: LeagueClientAPICredentials,
    ) -> Self {
        Self {
            riot_client_api_credentials,
            league_client_api_credentials,
        }
    }

    // fn obtain_from_procesess() -> Self {}
}

pub struct LeagueRestAPI {}
