// Copyright (c) 2024 Ivan Guerreschi. All rights reserved.
// Licensed under the MIT License. See LICENSE in the project root for license information.

pub mod csv {
    use std::env;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Lines, Result};

    pub async fn read_lines(filename: &str) -> Result<Lines<BufReader<File>>> {
        const KEY: &str = "HOME";
        let home = env::var(KEY).expect("$HOME is not set");
        let file = File::open(home + filename)?;
        Ok(BufReader::new(file).lines())
    }
}

pub mod local_version {
    use std::io::Result;
    use std::process::{Command, Output};

    pub async fn command(name: &str) -> Result<Output> {
        let output = Command::new(name).args(["--version"]).output()?;
        Ok(output)
    }
}

pub mod fetch {

    use reqwest::Error;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    pub struct Release {
        pub tag_name: String,
        pub html_url: String,
    }

    pub async fn release(owner: &str, repo: &str) -> Result<Release, Error> {
        let url = format!(
            "https://api.github.com/repos/{}/{}/releases/latest",
            owner, repo
        );

        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .header("User-Agent", "whatversion")
            .send()
            .await?;

        let release: Release = response.json().await?;

        Ok(release)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_version() {
        let rustc = "rustc";
        let result = local_version::command(rustc).await;
        let expected = false;
        match result {
            Ok(output) => assert_eq!(expected, output.stdout.is_empty()),
            Err(error) => panic!("Test error {}", error),
        };
    }
}
