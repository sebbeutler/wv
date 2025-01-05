cargo install create-tauri-app --locked
cargo create-tauri-app

brew install deno
curl -fsSL https://deno.land/install.sh | sh
cargo install deno --locked

deno install
deno task tauri dev

## PYO3

export DYLD_LIBRARY_PATH=/python/path/lib:$DYLD_LIBRARY_PATH

## Logging


### Rust to console

[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"

```{rust}
use tracing::{info, warn, error};
use tracing_subscriber::{fmt, EnvFilter};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
```

### Front-end to console/file/..

```{TypeScript}
import * as log from "https://deno.land/std@0.195.0/log/mod.ts";

await log.setup({
  handlers: {
    console: new log.handlers.ConsoleHandler("DEBUG"),
    file: new log.handlers.FileHandler("DEBUG", {
      filename: "./deno.log",
      formatter: "{datetime} [{level}] {msg}",
    }),
  },
  loggers: {
    default: {
      level: "DEBUG",
      handlers: ["console", "file"],
    },
  },
});

const logger = log.getLogger();
logger.info("This is an info message.");
logger.error("This is an error message.");
```

### Rust to Deno

```{rust}
#[tauri::command]
fn log_message(level: String, message: String) {
    match level.as_str() {
        "info" => tracing::info!(target: "frontend", "{}", message),
        "warn" => tracing::warn!(target: "frontend", "{}", message),
        "error" => tracing::error!(target: "frontend", "{}", message),
        _ => tracing::debug!(target: "frontend", "{}", message),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![log_message])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
```

### Deno to Rust

```{TypeScript}
import { invoke } from "@tauri-apps/api/tauri";

async function logToBackend(level: string, message: string) {
  await invoke("log_message", { level, message });
}

logToBackend("info", "This is a log message from Deno");
```