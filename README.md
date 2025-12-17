# MEGA Cloud Storage MCP Server

MCP server for MEGA cloud storage integration with Claude Code. Provides comprehensive file management, synchronization, and sharing capabilities using MEGAcmd.

## Features

- **File Management** - List, copy, move, delete files and folders
- **Upload/Download** - Transfer files between local and cloud
- **Sharing** - Create public links and share with other users
- **Sync** - Set up folder synchronization
- **Storage Info** - View usage and quotas

## Available Tools (18 total)

### Account & Navigation
| Tool | Description |
|------|-------------|
| `mega_whoami` | Get current logged-in account info |
| `mega_pwd` | Print current working directory |
| `mega_cd` | Change current directory |
| `mega_df` | Show storage space usage |
| `mega_du` | Show disk usage of remote path |

### File Operations
| Tool | Description |
|------|-------------|
| `mega_ls` | List files and folders |
| `mega_mkdir` | Create a directory |
| `mega_rm` | Remove files or folders |
| `mega_mv` | Move or rename files/folders |
| `mega_cp` | Copy files/folders |
| `mega_cat` | Display contents of a remote file |
| `mega_tree` | Show directory tree structure |
| `mega_find` | Search for files/folders |

### Transfer
| Tool | Description |
|------|-------------|
| `mega_get` | Download files to local filesystem |
| `mega_put` | Upload files to MEGA cloud |
| `mega_transfers` | Show current transfers |
| `mega_sync` | Set up folder synchronization |

### Sharing
| Tool | Description |
|------|-------------|
| `mega_export` | Create a public link |
| `mega_share` | Share folder with another user |
| `mega_import` | Import a public MEGA link |

## Setup

### 1. Prerequisites

Install MEGAcmd from https://mega.io/cmd

On macOS:
```bash
brew install --cask megacmd
```

Login to your MEGA account:
```bash
mega-login your@email.com password
```

### 2. Install Dependencies

```bash
cd ~/mcp-servers/mega-mcp
npm install
npm run build
```

### 3. Add to Claude Code

Add to `~/.claude.json`:

```json
{
  "mcpServers": {
    "mega": {
      "type": "stdio",
      "command": "node",
      "args": ["/Users/matthewkarsten/mcp-servers/mega-mcp/dist/index.js"]
    }
  }
}
```

## Architecture

```
Claude Code (Opus 4.5)
         │
         └──▶ MEGA MCP Server
                    │
                    └──▶ MEGAcmd CLI
                              │
                              └──▶ MEGA Cloud Storage
                                        │
                                        ├── 20 GB Free Storage
                                        ├── End-to-End Encryption
                                        ├── File Versioning
                                        └── Sharing & Sync
```

## MEGA Features

- **End-to-End Encryption** - All files encrypted client-side
- **20 GB Free Storage** - Free tier included
- **File Versioning** - Automatic version history
- **Cross-Platform Sync** - Desktop and mobile apps
- **Public Links** - Share files with anyone
- **Folder Sharing** - Collaborate with other MEGA users

## Usage Examples

```
User: Show my MEGA storage usage

Claude: [Uses mega_df tool]
Result:
Account: purplesquirrelmedia@icloud.com
Used: 2.3 GB of 20 GB (11.5%)
Available: 17.7 GB

User: List files in my Documents folder

Claude: [Uses mega_ls tool with path=/Documents]
Result:
- project-files/
- presentations/
- backup.zip (156 MB)
- notes.txt (2.3 KB)

User: Create a public link for backup.zip

Claude: [Uses mega_export tool]
Result: https://mega.nz/file/xxxxx#yyyyy
```

## Transfer Features

### Upload
```
User: Upload ~/reports/ to MEGA

Claude: [Uses mega_put tool]
Uploading 15 files...
Completed: ~/reports/ -> /reports/
```

### Download
```
User: Download /Documents/backup.zip

Claude: [Uses mega_get tool]
Downloaded: /Documents/backup.zip -> ~/Downloads/backup.zip
```

### Sync
```
User: Set up sync between ~/Projects and /Projects

Claude: [Uses mega_sync tool]
Sync established: ~/Projects <-> /Projects
```

## Files

```
mega-mcp/
├── src/
│   └── index.ts    # MCP server implementation
├── dist/           # Compiled JavaScript
├── package.json
├── tsconfig.json
└── README.md
```

## Dependencies

- `@modelcontextprotocol/sdk` - MCP SDK
- MEGAcmd - MEGA command-line client

## Security

- All data encrypted with your account key
- Zero-knowledge encryption (MEGA can't read your files)
- Two-factor authentication supported
- Session-based authentication via MEGAcmd

## Author

Matthew Karsten

## License

MIT
