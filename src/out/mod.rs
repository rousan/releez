use crate::prelude::*;
use tokio::io::{self, BufReader};

pub async fn read_line() -> crate::Result<String> {
    let mut stdin = BufReader::new(io::stdin());

    let mut line = String::with_capacity(10);
    stdin
        .read_line(&mut line)
        .await
        .context("Couldn't read line from stdin")?;

    Ok(line.trim().to_owned())
}

pub async fn print(text: impl AsRef<str>) -> crate::Result<()> {
    let mut stdout = io::stdout();

    stdout
        .write_all(text.as_ref().as_bytes())
        .await
        .context("Couldn't print text to stdout")?;

    stdout.flush().await.context("Couldn't flush the stdout")?;

    Ok(())
}

pub async fn print_err(text: impl AsRef<str>) -> crate::Result<()> {
    let mut stderr = io::stderr();

    stderr
        .write_all(text.as_ref().as_bytes())
        .await
        .context("Couldn't print text to stderr")?;

    stderr.flush().await.context("Couldn't flush the stderr")?;

    Ok(())
}
