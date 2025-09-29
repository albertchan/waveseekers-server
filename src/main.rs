use std::fmt::{Debug, Display};
use tokio::task::JoinError;
use waveseekers::{application::Application, configuration::get_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let config = get_config().expect("Failed to read config");
    let application = Application::build(&config).await?;
    let application_task = tokio::spawn(application.run_until_stopped());

    tokio::select! {
        val = application_task => report_exit("API", val),
    }

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{} failed",
                task_name
            )
        }
        Err(e) => {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "{}' task failed to complete",
                task_name
            )
        }
    }
}
