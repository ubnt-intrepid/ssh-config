use std::collections::BTreeMap;

#[derive(Default, Debug)]
pub struct Host {
  pub hostname: String,
  pub user: String,
  pub port: u16,
  pub identity_file: String,
}

impl Host {
  fn new(entries: Vec<(String, String)>) -> Result<Host, String> {
    let mut host = Host::default();
    for entry in entries {
      match entry.0.as_str() {
        "HostName" => host.hostname = entry.1,
        "User" => host.user = entry.1,
        "Port" => {
          host.port = entry.1
            .parse()
            .map_err(|_| "failed to parse")?
        }
        "IdentityFile" => host.identity_file = entry.1,
        _ => (),
      }
    }
    Ok(host)
  }
}

pub fn parse(input: &str) -> Result<BTreeMap<String, Host>, String> {
  // Step1. parse lines.
  let mut lines = Vec::new();
  for line in input.split("\n") {
    let line: &str = line.trim_left();
    if let Some(c) = line.chars().next() {
      if c != '#' {
        let a: Vec<_> = line.split_whitespace().collect();
        // TODO: split by space
        lines.push((a[0].to_owned(), (&a[1..]).join(" ")));
      }
    }
  }

  // 2. Split by sections
  let mut entries = Vec::new();
  {
    let mut iter = lines.into_iter();
    let mut ent = None;
    while let Some((a, b)) = iter.next() {
      if a == "Host" {
        if let Some(ent) = ent {
          entries.push(ent);
        }
        ent = Some((b, Vec::new()));
      } else {
        ent.as_mut()
          .unwrap()
          .1
          .push((a, b));
      }
    }
    if let Some(ent) = ent {
      entries.push(ent);
    }
  }

  // Step3. parse sections
  let mut hosts = BTreeMap::new();
  for entry in entries {
    let host = Host::new(entry.1)?;
    hosts.insert(entry.0, host);
  }

  Ok(hosts)
}
