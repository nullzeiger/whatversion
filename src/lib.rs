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
    use octocrab::models::repos::Release;
    use octocrab::Result;

    pub async fn release(owner: &str, repo: &str) -> Result<Release> {
        let release = octocrab::instance()
            .repos(owner, repo)
            .releases()
            .get_latest()
            .await?;
        Ok(release)
    }
}
