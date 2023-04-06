use crate::github::PullRequest;
use crate::handlers::RepositoryClient;

pub async fn command_ping<Client: RepositoryClient>(
    client: &mut Client,
    pr: &PullRequest,
) -> anyhow::Result<()> {
    log::debug!("Executing ping on {}", client.repository());
    client.post_comment(pr, "Pong 🏓!").await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::handlers::ping::command_ping;
    use crate::tests::client::test_client;
    use crate::tests::github::PRBuilder;

    #[tokio::test]
    async fn test_ping() {
        let mut client = test_client();
        command_ping(&mut client, &PRBuilder::default().number(1).create())
            .await
            .unwrap();
        client.check_comments(1, &["Pong 🏓!"]);
    }
}
