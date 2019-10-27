# environ-str

Simple conversion between a Rust value and a string with similar semantics to IEEE Std 1003.1-2008's `environ`.

## Examples

Parsing an `environ` string into `EnvVar`s.

```rust
use environ_str::{EnvVar, parse_environ};

let mut env_vars = parse_environ("PATH=/usr/local/bin HOME=/home/name");

assert_eq!(
    env_vars.next().unwrap(),
    Ok(EnvVar { name: "PATH".to_string(), value: "/usr/local/bin".to_string() }),
);
assert_eq!(
    env_vars.next().unwrap(),
    Ok(EnvVar { name: "HOME".to_string(), value: "/home/name".to_string() }),
);
```

Converting an `EnvVar` into a string and vice versa.

```rust
use environ_str::EnvVar;

let env_var = EnvVar { name: "PATH".to_string(), value: "/usr/local/bin".to_string() };
let environ_str = env_var.to_string();

assert_eq!(environ_str, "PATH=/usr/local/bin");
assert_eq!(environ_str.parse::<EnvVar>().unwrap(), env_var);
```

With `serde` feature enabled (default), `EnvVar` also implements `Serialize` and `Deserialize`.

```rust
use environ_str::EnvVar;

let env_var = EnvVar { name: "PATH".to_string(), value: "/usr/local/bin".to_string() };
assert_eq!(serde_json::to_string(&env_var).unwrap(), r#""PATH=/usr/local/bin""#);

let env_var: EnvVar = serde_json::from_str(r#""PATH=/usr/local/bin""#).unwrap();
assert_eq!(
    env_var,
    EnvVar { name: "PATH".to_string(), value: "/usr/local/bin".to_string() }
);
```
