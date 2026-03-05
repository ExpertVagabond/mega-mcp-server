#![recursion_limit = "512"]
use serde::Deserialize;
use serde_json::{json, Value};
use std::io::BufRead;
use std::process::Command;

#[derive(Deserialize)]
struct JsonRpcRequest { #[allow(dead_code)] jsonrpc: String, id: Option<Value>, method: String, params: Option<Value> }

fn mega_cmd_path() -> String {
    std::env::var("MEGA_CMD_PATH").unwrap_or_else(|_| "/Applications/MEGAcmd.app/Contents/MacOS".into())
}

fn run_mega(cmd: &str, args: &[&str]) -> Result<String, String> {
    let path = mega_cmd_path();
    let mega_exec = format!("{}/mega-exec", path);
    let mut all_args = vec![cmd];
    all_args.extend_from_slice(args);
    let output = Command::new(&mega_exec).args(&all_args).output()
        .map_err(|e| format!("mega-exec failed: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let combined: String = format!("{stdout}\n{stderr}").lines()
        .filter(|l| !l.contains("Shell cwd was reset"))
        .collect::<Vec<_>>().join("\n").trim().to_string();
    if combined.is_empty() { Ok("Command completed successfully".into()) } else { Ok(combined) }
}

fn tool_definitions() -> Value {
    json!([
        {"name":"mega_whoami","description":"Get current MEGA account info","inputSchema":{"type":"object","properties":{}}},
        {"name":"mega_ls","description":"List files and folders in MEGA cloud","inputSchema":{"type":"object","properties":{"path":{"type":"string","default":"/"},"long":{"type":"boolean","default":false},"recursive":{"type":"boolean","default":false}}}},
        {"name":"mega_cd","description":"Change directory in MEGA cloud","inputSchema":{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}},
        {"name":"mega_pwd","description":"Print current directory in MEGA cloud","inputSchema":{"type":"object","properties":{}}},
        {"name":"mega_mkdir","description":"Create a directory in MEGA","inputSchema":{"type":"object","properties":{"path":{"type":"string"},"parents":{"type":"boolean","default":false}},"required":["path"]}},
        {"name":"mega_rm","description":"Remove files/folders from MEGA","inputSchema":{"type":"object","properties":{"path":{"type":"string"},"recursive":{"type":"boolean","default":false},"force":{"type":"boolean","default":false}},"required":["path"]}},
        {"name":"mega_mv","description":"Move/rename files in MEGA","inputSchema":{"type":"object","properties":{"source":{"type":"string"},"destination":{"type":"string"}},"required":["source","destination"]}},
        {"name":"mega_cp","description":"Copy files within MEGA","inputSchema":{"type":"object","properties":{"source":{"type":"string"},"destination":{"type":"string"}},"required":["source","destination"]}},
        {"name":"mega_get","description":"Download files from MEGA to local","inputSchema":{"type":"object","properties":{"remote_path":{"type":"string"},"local_path":{"type":"string"}},"required":["remote_path"]}},
        {"name":"mega_put","description":"Upload files to MEGA from local","inputSchema":{"type":"object","properties":{"local_path":{"type":"string"},"remote_path":{"type":"string"}},"required":["local_path"]}},
        {"name":"mega_df","description":"Show MEGA storage space usage","inputSchema":{"type":"object","properties":{"human":{"type":"boolean","default":true}}}},
        {"name":"mega_du","description":"Show disk usage of remote path","inputSchema":{"type":"object","properties":{"path":{"type":"string","default":"/"},"human":{"type":"boolean","default":true}}}},
        {"name":"mega_find","description":"Search for files in MEGA","inputSchema":{"type":"object","properties":{"pattern":{"type":"string"},"path":{"type":"string","default":"/"}},"required":["pattern"]}},
        {"name":"mega_export","description":"Create a public link for a file/folder","inputSchema":{"type":"object","properties":{"path":{"type":"string"},"expire":{"type":"string"},"password":{"type":"string"}},"required":["path"]}},
        {"name":"mega_share","description":"Share a folder with another MEGA user","inputSchema":{"type":"object","properties":{"path":{"type":"string"},"email":{"type":"string"},"access_level":{"type":"string","default":"r"}},"required":["path","email"]}},
        {"name":"mega_transfers","description":"Show current transfers","inputSchema":{"type":"object","properties":{"show_completed":{"type":"boolean","default":false}}}},
        {"name":"mega_sync","description":"Set up sync between local and remote","inputSchema":{"type":"object","properties":{"local_path":{"type":"string"},"remote_path":{"type":"string"},"list_only":{"type":"boolean","default":false}}}},
        {"name":"mega_tree","description":"Show directory tree","inputSchema":{"type":"object","properties":{"path":{"type":"string","default":"/"}}}},
        {"name":"mega_cat","description":"Display contents of a remote file","inputSchema":{"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}},
        {"name":"mega_import","description":"Import a public MEGA link","inputSchema":{"type":"object","properties":{"link":{"type":"string"},"remote_path":{"type":"string","default":"/"}},"required":["link"]}}
    ])
}

fn call_tool(name: &str, args: &Value) -> Value {
    let s = |k: &str| args[k].as_str().unwrap_or("").to_string();
    let b = |k: &str| args[k].as_bool().unwrap_or(false);
    let text = match name {
        "mega_whoami" => run_mega("whoami", &[]),
        "mega_ls" => {
            let mut a = Vec::new();
            if b("long") { a.push("-l"); }
            if b("recursive") { a.push("-R"); }
            let path = s("path"); let p = if path.is_empty() { "/".to_string() } else { path }; a.push(&p);
            run_mega("ls", &a.iter().map(|s| s.as_ref()).collect::<Vec<_>>())
        }
        "mega_cd" => run_mega("cd", &[&s("path")]),
        "mega_pwd" => run_mega("pwd", &[]),
        "mega_mkdir" => {
            let mut a = Vec::new();
            if b("parents") { a.push("-p"); }
            let path = s("path"); a.push(&path);
            run_mega("mkdir", &a.iter().map(|s| s.as_ref()).collect::<Vec<_>>())
        }
        "mega_rm" => {
            let mut a = Vec::new();
            if b("recursive") { a.push("-r"); }
            if b("force") { a.push("-f"); }
            let path = s("path"); a.push(&path);
            run_mega("rm", &a.iter().map(|s| s.as_ref()).collect::<Vec<_>>())
        }
        "mega_mv" => run_mega("mv", &[&s("source"), &s("destination")]),
        "mega_cp" => run_mega("cp", &[&s("source"), &s("destination")]),
        "mega_get" => {
            let rp = s("remote_path"); let lp = s("local_path");
            if lp.is_empty() { run_mega("get", &[&rp]) } else { run_mega("get", &[&rp, &lp]) }
        }
        "mega_put" => {
            let lp = s("local_path"); let rp = s("remote_path");
            if rp.is_empty() { run_mega("put", &[&lp]) } else { run_mega("put", &[&lp, &rp]) }
        }
        "mega_df" => {
            let mut a = Vec::new();
            if b("human") { a.push("-h"); }
            run_mega("df", &a.iter().map(|s| s.as_ref()).collect::<Vec<_>>())
        }
        "mega_du" => {
            let mut a = Vec::new();
            if b("human") { a.push("-h"); }
            let path = s("path"); if !path.is_empty() { a.push(&path); }
            run_mega("du", &a.iter().map(|s| s.as_ref()).collect::<Vec<_>>())
        }
        "mega_find" => {
            let path = s("path"); let pattern = format!("--pattern={}", s("pattern"));
            let mut a = Vec::new();
            if !path.is_empty() { a.push(path.as_str()); }
            a.push(&pattern);
            run_mega("find", &a)
        }
        "mega_export" => {
            let path = s("path");
            let mut a = vec!["-a".to_string(), path];
            let expire = s("expire"); if !expire.is_empty() { a.push(format!("--expire={expire}")); }
            let pw = s("password"); if !pw.is_empty() { a.push(format!("--password={pw}")); }
            run_mega("export", &a.iter().map(|s| s.as_ref()).collect::<Vec<_>>())
        }
        "mega_share" => {
            let path = s("path"); let email = format!("--with={}", s("email"));
            let level = format!("--level={}", if s("access_level").is_empty() { "r".to_string() } else { s("access_level") });
            run_mega("share", &["-a", &email, &level, &path])
        }
        "mega_transfers" => {
            let mut a = Vec::new();
            if b("show_completed") { a.push("-c"); }
            run_mega("transfers", &a.iter().map(|s| s.as_ref()).collect::<Vec<_>>())
        }
        "mega_sync" => {
            if b("list_only") { run_mega("sync", &[]) }
            else {
                let lp = s("local_path"); let rp = s("remote_path");
                if lp.is_empty() || rp.is_empty() { run_mega("sync", &[]) }
                else { run_mega("sync", &[&lp, &rp]) }
            }
        }
        "mega_tree" => {
            let path = s("path");
            if path.is_empty() { run_mega("tree", &[]) } else { run_mega("tree", &[&path]) }
        }
        "mega_cat" => run_mega("cat", &[&s("path")]),
        "mega_import" => {
            let link = s("link"); let rp = s("remote_path");
            if rp.is_empty() { run_mega("import", &[&link]) } else { run_mega("import", &[&link, &rp]) }
        }
        _ => Err(format!("Unknown tool: {name}")),
    };
    match text {
        Ok(t) => json!({"content":[{"type":"text","text":t}]}),
        Err(e) => json!({"content":[{"type":"text","text":format!("Error: {e}")}],"isError":true}),
    }
}

fn main() {
    tracing_subscriber::fmt().with_env_filter("info").with_writer(std::io::stderr).init();
    eprintln!("[mega-mcp] Starting with 20 tools");
    let stdin = std::io::stdin();
    let mut line = String::new();
    loop {
        line.clear();
        if stdin.lock().read_line(&mut line).unwrap_or(0) == 0 { break; }
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        let req: JsonRpcRequest = match serde_json::from_str(trimmed) { Ok(r) => r, Err(_) => continue };
        let resp = match req.method.as_str() {
            "initialize" => json!({"jsonrpc":"2.0","id":req.id,"result":{"protocolVersion":"2024-11-05","capabilities":{"tools":{}},"serverInfo":{"name":"mega-mcp","version":"1.0.0"}}}),
            "notifications/initialized" => continue,
            "tools/list" => json!({"jsonrpc":"2.0","id":req.id,"result":{"tools":tool_definitions()}}),
            "tools/call" => {
                let params = req.params.unwrap_or(json!({}));
                let name = params["name"].as_str().unwrap_or("");
                let args = params.get("arguments").cloned().unwrap_or(json!({}));
                json!({"jsonrpc":"2.0","id":req.id,"result":call_tool(name, &args)})
            }
            _ => json!({"jsonrpc":"2.0","id":req.id,"error":{"code":-32601,"message":"Method not found"}}),
        };
        println!("{}", serde_json::to_string(&resp).unwrap());
    }
}
