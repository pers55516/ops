use ops::{server, CheckResponse, Checker, NamedChecker, Result, StatusBuilder};

const APP_NAME: &str = "example";
const APP_DESC: &str = "An example app with an ops server";
const APP_SHA: &str = "12561012a04f945852cf0171da516a9ffc709e76";

const HOST: &str = "0.0.0.0:3000";

struct NoopChecker {}

impl Checker for NoopChecker {
    fn check(&self) -> CheckResponse {
        CheckResponse::healthy("noop is always healthy")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let noop = NoopChecker {};

    let healthchecks = StatusBuilder::healthchecks(APP_NAME, APP_DESC)
        .checker(NamedChecker::new("noop", noop))
        .revision(APP_SHA);

    let server = server(HOST.parse()?, healthchecks);

    println!("Serving http://{}", HOST);

    tokio::try_join!(server)?;

    Ok(())
}
