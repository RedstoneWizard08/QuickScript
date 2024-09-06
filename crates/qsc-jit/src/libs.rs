use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use regex::Regex;
use target_lexicon::Triple;

pub(crate) fn linux_triple(triple: Triple) -> String {
    format!(
        "{}-{}-{}",
        triple.architecture, triple.operating_system, triple.environment
    )
}

pub(crate) fn find_library(lib: &str) -> Result<Vec<PathBuf>> {
    let pc = format!("{}.pc", lib);
    let triple = linux_triple(Triple::host());

    let roots = &[
        PathBuf::from("/usr/lib/pkgconfig"),
        PathBuf::from(format!("/usr/lib/{}/pkgconfig", triple)),
    ];

    for root in roots {
        let path = root.join(&pc);

        if path.exists() {
            let data = fs::read_to_string(&path)?;
            let content = data.split("\n");
            let mut iter = content.into_iter();
            let libs = iter.find(|v| v.starts_with("Libs:"));

            if let Some(libs) = libs {
                let mut libs_iter = libs.split(" ").into_iter();

                let dir = libs_iter
                    .find(|v| v.starts_with("-L"))
                    .map(|v| v.replace("-L", ""));

                if let Some(dir) = dir {
                    let dir = PathBuf::from(dir);
                    let mut data = Vec::new();

                    for item in libs_iter
                        .filter(|v| v.starts_with("-l"))
                        .map(|v| dir.join(format!("{}.so", v.replace("-l", "lib"))))
                        .map(|v| v.canonicalize().unwrap())
                    {
                        let content = fs::read(&item)?;

                        if &content[0..5] == b"INPUT" {
                            let raw = fs::read_to_string(&item)?;
                            let re = Regex::new(r"INPUT\(([^\s]+)").unwrap();
                            let parent = item.parent().ok_or(anyhow!("Path has no parent!"))?;

                            let cap = re
                                .captures(&raw)
                                .ok_or(anyhow!("Regex capturing failed!"))?
                                .get(1)
                                .ok_or(anyhow!("Could not get first capture group from regex!"))?
                                .as_str();

                            let item = parent.join(cap);

                            data.push(item);
                        } else {
                            data.push(item);
                        }
                    }

                    return Ok(data);
                }
            }
        }
    }

    Err(anyhow!("Cannot find library: {}", lib))
}
