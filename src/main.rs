use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::io::{self, BufRead, Write};
use std::process::Command;
use std::env;

const MEGA_CMD_PATH: &str = "/Applications/MEGAcmd.app/Contents/MacOS";

fn run_mega(cmd: &str, args: &[&str]) -> Result<String, String> {
    let mega_path = env::var("MEGA_CMD_PATH").unwrap_or_else(|_| MEGA_CMD_PATH.into());
    let binary = format!("{}/mega-exec", mega_path);
    let mut command = Command::new(&binary);
    command.arg(cmd);
    for a in args { command.arg(a); }

    let output = command.output().map_err(|e| format!("Failed to run mega-exec: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined: String = format!("{}{}", stdout, stderr)
        .lines()
        .filter(|l| !l.contains("Shell cwd was reset"))
        .collect::<Vec<_>>()
        .join("\n");

    if combined.trim().is_empty() {
        Ok("Command completed successfully".into())
    } else {
        Ok(combined.trim().to_string())
    }
}

fn call_tool(name: &str, args: &Value) -> Value {
    let s = |key: &str| args.get(key).and_then(|v| v.as_str()).unwrap_or("");
    let b = |key: &str| args.get(key).and_then(|v| v.as_bool()).unwrap_or(false);

    let result = match name {
        "mega_whoami" => run_mega("whoami", &[]),
        "mega_ls" => {
            let path = s("path"); if path.is_empty() { "/"  } else { path };
            let mut a = vec![path];
            if b("long") { a.push("-l"); }
            if b("recursive") { a.push("-R"); }
            run_mega("ls", &a)
        }
        "mega_cd" => run_mega("cd", &[s("path")]),
        "mega_pwd" => run_mega("pwd", &[]),
        "mega_mkdir" => {
            let mut a = vec![s("path")];
            if b("parents") { a.push("-p"); }
            run_mega("mkdir", &a)
        }
        "mega_rm" => {
            let mut a = vec![s("path")];
            if b("recursive") { a.push("-r"); }
            if b("force") { a.push("-f"); }
            run_mega("rm", &a)
        }
        "mega_cp" => run_mega("cp", &[s("source"), s("destination")]),
        "mega_mv" => run_mega("mv", &[s("source"), s("destination")]),
        "mega_get" => run_mega("get", &[s("remote_path"), s("local_path")]),
        "mega_put" => run_mega("put", &[s("local_path"), s("remote_path")]),
        "mega_cat" => run_mega("cat", &[s("path")]),
        "mega_find" => {
            let mut a = vec![s("path")];
            let pattern = s("pattern");
            if !pattern.is_empty() { a.push("--pattern"); a.push(pattern); }
            run_mega("find", &a)
        }
        "mega_df" => run_mega("df", &[]),
        "mega_du" => run_mega("du", &[s("path")]),
        "mega_export" => run_mega("export", &[s("path")]),
        "mega_import" => run_mega("import", &[s("url"), s("destination")]),
        "mega_share" => run_mega("share", &[s("path"), s("email"), &format!("--level={}", s("level"))]),
        "mega_sync" => run_mega("sync", &[s("local_path"), s("remote_path")]),
        "mega_transfers" => run_mega("transfers", &[]),
        "mega_tree" => run_mega("tree", &[s("path")]),
        _ => Err(format!("Unknown tool: {name}")),
    };

    match result {
        Ok(output) => json!({"content":[{"type":"text","text":output}]}),
        Err(e) => json!({"content":[{"type":"text","text":format!("Error: {e}")}],"isError":true}),
    }
}

fn tool_definitions() -> Value {
    json!([
        {"name":"mega_whoami","description":"Get current MEGA account info","inputSchema":{"type":"object","properties":{}}},
        {"name":"mega_ls","description":"List files in MEGA cloud","inputSchema":{"type":"object","properties":{"path":{"type":"string","default":"/"},"long":{"type":"boolean","default":false},"recursive":{"type":"boolean","default":false}}}},
        {"name":"mega_cd","description":"Change directory in MEGA","inputSchema":{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}},
        {"name":"mega_pwd","description":"Print current MEGA directory","inputSchema":{"type":"object","properties":{}}},
        {"name":"mega_mkdir","description":"Create directory in MEGA","inputSchema":{"type":"object","properties":{"path":{"type":"string"},"parents":{"type":"boolean","default":false}},"required":["path"]}},
        {"name":"mega_rm","description":"Remove files/folders from MEGA","inputSchema":{"type":"object","properties":{"path":{"type":"string"},"recursive":{"type":"boolean","default":false},"force":{"type":"boolean","default":false}},"required":["path"]}},
        {"name":"mega_cp","description":"Copy files within MEGA","inputSchema":{"type":"object","properties":{"source":{"type":"string"},"destination":{"type":"string"}},"required":["source","destination"]}},
        {"name":"mega_mv","description":"Move/rename files in MEGA","inputSchema":{"type":"object","properties":{"source":{"type":"string"},"destination":{"type":"string"}},"required":["source","destination"]}},
        {"name":"mega_get","description":"Download file from MEGA to local","inputSchema":{"type":"object","properties":{"remote_path":{"type":"string"},"local_path":{"type":"string","default":"."}},"required":["remote_path"]}},
        {"name":"mega_put","description":"Upload local file to MEGA","inputSchema":{"type":"object","properties":{"local_path":{"type":"string"},"remote_path":{"type":"string","default":"/"}},"required":["local_path"]}},
        {"name":"mega_cat","description":"Print file contents from MEGA","inputSchema":{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}},
        {"name":"mega_find","description":"Find files in MEGA by pattern","inputSchema":{"type":"object","properties":{"path":{"type":"string","default":"/"},"pattern":{"type":"string"}}}},
        {"name":"mega_df","description":"Show MEGA storage usage","inputSchema":{"type":"object","properties":{}}},
        {"name":"mega_du","description":"Show disk usage of path","inputSchema":{"type":"object","properties":{"path":{"type":"string","default":"/"}}}},
        {"name":"mega_export","description":"Export file/folder link","inputSchema":{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}},
        {"name":"mega_import","description":"Import from MEGA link","inputSchema":{"type":"object","properties":{"url":{"type":"string"},"destination":{"type":"string","default":"/"}},"required":["url"]}},
        {"name":"mega_share","description":"Share folder with user","inputSchema":{"type":"object","properties":{"path":{"type":"string"},"email":{"type":"string"},"level":{"type":"string","enum":["FULL","READ","READWRITE"],"default":"READ"}},"required":["path","email"]}},
        {"name":"mega_sync","description":"Sync local folder with MEGA","inputSchema":{"type":"object","properties":{"local_path":{"type":"string"},"remote_path":{"type":"string"}},"required":["local_path","remote_path"]}},
        {"name":"mega_transfers","description":"List active transfers","inputSchema":{"type":"object","properties":{}}},
        {"name":"mega_tree","description":"Show directory tree","inputSchema":{"type":"object","properties":{"path":{"type":"string","default":"/"}}}}
    ])
}

#[derive(Deserialize)] struct JsonRpcRequest { #[allow(dead_code)] jsonrpc: String, id: Option<Value>, method: String, #[serde(default)] params: Value }
#[derive(Serialize)] struct JsonRpcResponse { jsonrpc: String, id: Value, #[serde(skip_serializing_if = "Option::is_none")] result: Option<Value>, #[serde(skip_serializing_if = "Option::is_none")] error: Option<Value> }

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter(tracing_subscriber::EnvFilter::from_default_env()).with_writer(std::io::stderr).init();
    tracing::info!("mega-mcp-server starting");
    let stdin = io::stdin(); let stdout = io::stdout();
    for line in stdin.lock().lines() {
        let line = match line { Ok(l) => l, Err(_) => break };
        if line.trim().is_empty() { continue; }
        let req: JsonRpcRequest = match serde_json::from_str(&line) { Ok(r) => r, Err(_) => continue };
        let id = req.id.clone().unwrap_or(Value::Null);
        let response = match req.method.as_str() {
            "initialize" => Some(JsonRpcResponse { jsonrpc:"2.0".into(), id, result: Some(json!({"protocolVersion":"2024-11-05","capabilities":{"tools":{}},"serverInfo":{"name":"mega-mcp-server","version":env!("CARGO_PKG_VERSION")}})), error: None }),
            "notifications/initialized" => None,
            "tools/list" => Some(JsonRpcResponse { jsonrpc:"2.0".into(), id, result: Some(json!({"tools": tool_definitions()})), error: None }),
            "tools/call" => {
                let name = req.params.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let args = req.params.get("arguments").cloned().unwrap_or(json!({}));
                Some(JsonRpcResponse { jsonrpc:"2.0".into(), id, result: Some(call_tool(name, &args)), error: None })
            }
            other => Some(JsonRpcResponse { jsonrpc:"2.0".into(), id, result: None, error: Some(json!({"code":-32601,"message":format!("method not found: {other}")})) }),
        };
        if let Some(resp) = response {
            let mut out = stdout.lock();
            let _ = serde_json::to_writer(&mut out, &resp);
            let _ = out.write_all(b"\n"); let _ = out.flush();
        }
    }
}
