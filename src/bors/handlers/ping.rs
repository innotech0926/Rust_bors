use crate::bors::RepositoryClient;
use crate::bors::RepositoryState;
use crate::github::PullRequest;

pub async fn command_ping<Client: RepositoryClient>(
    repo: &mut RepositoryState<Client>,
    pr: &PullRequest,
) -> anyhow::Result<()> {
    log::debug!("Executing ping on {}", repo.client.repository());
    repo.client
        .post_comment(pr.number.into(), "Pong 🏓!")
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::tests::state::RepoBuilder;

    #[tokio::test]
    async fn test_ping() {
        let mut state = RepoBuilder::default().create_state().await;
        state.comment("@bors ping").await;
        state.client().check_comments(1, &["Pong 🏓!"]);
    }
}
