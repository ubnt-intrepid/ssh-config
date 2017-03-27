extern crate ssh_config;

#[test]
fn simple_case() {
  let input = include_str!("ssh_config");
  let result = ssh_config::parse(input).expect("failed to parse");

  let host = result.get("server1").expect("no such entry: server1");
  assert_eq!(host.hostname, "192.168.24.2");
  assert_eq!(host.user, "user1");
  assert_eq!(host.port, 49001);
  assert_eq!(host.identity_file, "~/.ssh/id_ecdsa");

  let host = result.get("server2").expect("no such entry: server2");
  assert_eq!(host.hostname, "192.168.24.2");
  assert_eq!(host.user, "user2");
  assert_eq!(host.port, 49005);
  assert_eq!(host.identity_file, "~/.ssh/id_ecdsa2");
}
