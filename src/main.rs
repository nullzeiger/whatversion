// Copyright (c) 2024 Ivan Guerreschi. All rights reserved.
// Licensed under the MIT License. See License.txt in the project root for license information.

use whatversion::{csv, fetch, local_version};

#[tokio::main]
async fn main() {
    const FILE: &str = "/.apps.csv";

    if let Ok(apps) = csv::read_lines(FILE).await {
        for app in apps.map_while(Result::ok) {
            let app_values: Vec<&str> = app.split(',').collect();
            if let Ok(output) = local_version::command(app_values[0]).await {
                if let Ok(release) = fetch::release(app_values[1], app_values[2]).await {
                    println!(
                        "Local version of {}last version in GitHub repo is {}\nurl {}\n",
                        String::from_utf8_lossy(&output.stdout),
                        release.tag_name,
                        release.html_url
                    );
                }
            }
        }
    }
}
