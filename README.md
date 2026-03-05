# mega-mcp-server

[\![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[\![MCP](https://img.shields.io/badge/MCP-Compatible-blue.svg)](https://modelcontextprotocol.io)
[\![TypeScript](https://img.shields.io/badge/TypeScript-5.x-blue.svg)](https://www.typescriptlang.org)

MCP server for MEGA encrypted cloud storage. Provides file management, uploads, downloads, sharing, folder sync, and search via MEGAcmd.

## Tools (18 total)

### Account and Navigation

| Tool | Description |
|------|-------------|
| `mega_whoami` | Get current logged-in account info |
| `mega_pwd` | Print current working directory |
| `mega_cd` | Change current directory |
| `mega_df` | Show storage space usage |
| `mega_du` | Show disk usage of a remote path |

### File Operations

| Tool | Description |
|------|-------------|
| `mega_ls` | List files and folders (supports -l and -R) |
| `mega_mkdir` | Create a directory (supports -p) |
| `mega_rm` | Remove files or folders |
| `mega_mv` | Move or rename files and folders |
| `mega_cp` | Copy files and folders |
| `mega_cat` | Display contents of a remote file |
| `mega_tree` | Show directory tree structure |
| `mega_find` | Search for files with wildcard patterns |

### Transfer

| Tool | Description |
|------|-------------|
| `mega_get` | Download files to local filesystem |
| `mega_put` | Upload files to MEGA cloud |
| `mega_transfers` | Show current upload/download progress |
| `mega_sync` | Set up bidirectional folder sync |

### Sharing

| Tool | Description |
|------|-------------|
| `mega_export` | Create a public link (with optional expiry and password) |
| `mega_share` | Share a folder with another MEGA user (r/rw/full) |
| `mega_import` | Import a public MEGA link to your account |

## Prerequisites

Install MEGAcmd:

```bash
brew install --cask megacmd
```

Log in to your MEGA account:

```bash
mega-login your@email.com password
```

## Install

```bash
npm install
npm run build
```

## Configuration

```json
{
  "mcpServers": {
    "mega": {
      "type": "stdio",
      "command": "node",
      "args": ["/path/to/mega-mcp/dist/index.js"]
    }
  }
}
```

## MEGA Features

- **End-to-end encryption** -- all files encrypted client-side with zero-knowledge architecture
- **20 GB free storage** -- included with every account
- **File versioning** -- automatic version history
- **Cross-platform sync** -- desktop and mobile clients

## Dependencies

- `@modelcontextprotocol/sdk` -- MCP protocol SDK
- MEGAcmd -- MEGA command-line client (must be installed separately)

## License

[MIT](LICENSE)
