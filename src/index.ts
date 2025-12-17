#!/usr/bin/env node

import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { exec } from "child_process";
import { promisify } from "util";

const execAsync = promisify(exec);

const MEGA_CMD_PATH = "/Applications/MEGAcmd.app/Contents/MacOS";

async function runMegaCommand(command: string, args: string[] = []): Promise<string> {
  const escapedArgs = args.map(arg => `'${arg.replace(/'/g, "'\\''")}'`).join(" ");
  const fullCommand = `cd "${MEGA_CMD_PATH}" && ./mega-exec ${command} ${escapedArgs}`;

  try {
    const { stdout, stderr } = await execAsync(fullCommand, { timeout: 60000 });
    // Filter out the shell cwd reset message
    const output = (stdout + stderr)
      .split('\n')
      .filter(line => !line.includes('Shell cwd was reset'))
      .join('\n')
      .trim();
    return output || "Command completed successfully";
  } catch (error: any) {
    if (error.stdout || error.stderr) {
      const output = (error.stdout + error.stderr)
        .split('\n')
        .filter((line: string) => !line.includes('Shell cwd was reset'))
        .join('\n')
        .trim();
      if (output) return output;
    }
    throw new Error(`Command failed: ${error.message}`);
  }
}

const server = new Server(
  {
    name: "mega-mcp",
    version: "1.0.0",
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

server.setRequestHandler(ListToolsRequestSchema, async () => {
  return {
    tools: [
      {
        name: "mega_whoami",
        description: "Get current logged-in MEGA account information",
        inputSchema: {
          type: "object",
          properties: {},
          required: [],
        },
      },
      {
        name: "mega_ls",
        description: "List files and folders in MEGA cloud storage",
        inputSchema: {
          type: "object",
          properties: {
            path: {
              type: "string",
              description: "Remote path to list (default: current directory)",
              default: "/",
            },
            long: {
              type: "boolean",
              description: "Show detailed listing with sizes and dates",
              default: false,
            },
            recursive: {
              type: "boolean",
              description: "List recursively",
              default: false,
            },
          },
          required: [],
        },
      },
      {
        name: "mega_cd",
        description: "Change current working directory in MEGA cloud",
        inputSchema: {
          type: "object",
          properties: {
            path: {
              type: "string",
              description: "Remote path to change to",
            },
          },
          required: ["path"],
        },
      },
      {
        name: "mega_pwd",
        description: "Print current working directory in MEGA cloud",
        inputSchema: {
          type: "object",
          properties: {},
          required: [],
        },
      },
      {
        name: "mega_mkdir",
        description: "Create a directory in MEGA cloud storage",
        inputSchema: {
          type: "object",
          properties: {
            path: {
              type: "string",
              description: "Remote path for new directory",
            },
            parents: {
              type: "boolean",
              description: "Create parent directories as needed",
              default: false,
            },
          },
          required: ["path"],
        },
      },
      {
        name: "mega_rm",
        description: "Remove files or folders from MEGA cloud storage",
        inputSchema: {
          type: "object",
          properties: {
            path: {
              type: "string",
              description: "Remote path to remove",
            },
            recursive: {
              type: "boolean",
              description: "Remove directories recursively",
              default: false,
            },
            force: {
              type: "boolean",
              description: "Force removal without confirmation",
              default: false,
            },
          },
          required: ["path"],
        },
      },
      {
        name: "mega_mv",
        description: "Move or rename files/folders in MEGA cloud storage",
        inputSchema: {
          type: "object",
          properties: {
            source: {
              type: "string",
              description: "Source remote path",
            },
            destination: {
              type: "string",
              description: "Destination remote path",
            },
          },
          required: ["source", "destination"],
        },
      },
      {
        name: "mega_cp",
        description: "Copy files/folders within MEGA cloud storage",
        inputSchema: {
          type: "object",
          properties: {
            source: {
              type: "string",
              description: "Source remote path",
            },
            destination: {
              type: "string",
              description: "Destination remote path",
            },
          },
          required: ["source", "destination"],
        },
      },
      {
        name: "mega_get",
        description: "Download files from MEGA cloud to local filesystem",
        inputSchema: {
          type: "object",
          properties: {
            remote_path: {
              type: "string",
              description: "Remote path in MEGA to download",
            },
            local_path: {
              type: "string",
              description: "Local destination path (default: current directory)",
            },
          },
          required: ["remote_path"],
        },
      },
      {
        name: "mega_put",
        description: "Upload files from local filesystem to MEGA cloud",
        inputSchema: {
          type: "object",
          properties: {
            local_path: {
              type: "string",
              description: "Local file/folder path to upload",
            },
            remote_path: {
              type: "string",
              description: "Remote destination path in MEGA",
            },
          },
          required: ["local_path"],
        },
      },
      {
        name: "mega_df",
        description: "Show MEGA cloud storage space usage",
        inputSchema: {
          type: "object",
          properties: {
            human: {
              type: "boolean",
              description: "Show sizes in human-readable format",
              default: true,
            },
          },
          required: [],
        },
      },
      {
        name: "mega_du",
        description: "Show disk usage of remote path",
        inputSchema: {
          type: "object",
          properties: {
            path: {
              type: "string",
              description: "Remote path to check",
              default: "/",
            },
            human: {
              type: "boolean",
              description: "Show sizes in human-readable format",
              default: true,
            },
          },
          required: [],
        },
      },
      {
        name: "mega_find",
        description: "Search for files/folders in MEGA cloud storage",
        inputSchema: {
          type: "object",
          properties: {
            pattern: {
              type: "string",
              description: "Search pattern (supports wildcards)",
            },
            path: {
              type: "string",
              description: "Path to search in (default: /)",
              default: "/",
            },
          },
          required: ["pattern"],
        },
      },
      {
        name: "mega_export",
        description: "Create a public link for a file/folder",
        inputSchema: {
          type: "object",
          properties: {
            path: {
              type: "string",
              description: "Remote path to export",
            },
            expire: {
              type: "string",
              description: "Expiration time (e.g., '1d', '1w', '1m')",
            },
            password: {
              type: "string",
              description: "Password protect the link",
            },
          },
          required: ["path"],
        },
      },
      {
        name: "mega_share",
        description: "Share a folder with another MEGA user",
        inputSchema: {
          type: "object",
          properties: {
            path: {
              type: "string",
              description: "Remote folder path to share",
            },
            email: {
              type: "string",
              description: "Email of user to share with",
            },
            access_level: {
              type: "string",
              description: "Access level: 'r' (read), 'rw' (read-write), 'full' (full access)",
              enum: ["r", "rw", "full"],
              default: "r",
            },
          },
          required: ["path", "email"],
        },
      },
      {
        name: "mega_transfers",
        description: "Show current transfers (uploads/downloads)",
        inputSchema: {
          type: "object",
          properties: {
            show_completed: {
              type: "boolean",
              description: "Include completed transfers",
              default: false,
            },
          },
          required: [],
        },
      },
      {
        name: "mega_sync",
        description: "Set up sync between local and remote folders",
        inputSchema: {
          type: "object",
          properties: {
            local_path: {
              type: "string",
              description: "Local folder path",
            },
            remote_path: {
              type: "string",
              description: "Remote MEGA folder path",
            },
            list_only: {
              type: "boolean",
              description: "Just list current syncs instead of creating new one",
              default: false,
            },
          },
          required: [],
        },
      },
      {
        name: "mega_tree",
        description: "Show directory tree structure",
        inputSchema: {
          type: "object",
          properties: {
            path: {
              type: "string",
              description: "Remote path to show tree for",
              default: "/",
            },
          },
          required: [],
        },
      },
      {
        name: "mega_cat",
        description: "Display contents of a remote file",
        inputSchema: {
          type: "object",
          properties: {
            path: {
              type: "string",
              description: "Remote file path to display",
            },
          },
          required: ["path"],
        },
      },
      {
        name: "mega_import",
        description: "Import a public MEGA link to your account",
        inputSchema: {
          type: "object",
          properties: {
            link: {
              type: "string",
              description: "MEGA public link to import",
            },
            remote_path: {
              type: "string",
              description: "Destination path in your MEGA",
              default: "/",
            },
          },
          required: ["link"],
        },
      },
    ],
  };
});

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;

  try {
    let result: string;

    switch (name) {
      case "mega_whoami":
        result = await runMegaCommand("whoami");
        break;

      case "mega_ls": {
        const cmdArgs: string[] = [];
        if (args?.long) cmdArgs.push("-l");
        if (args?.recursive) cmdArgs.push("-R");
        if (args?.path) cmdArgs.push(args.path as string);
        else cmdArgs.push("/");
        result = await runMegaCommand("ls", cmdArgs);
        break;
      }

      case "mega_cd":
        result = await runMegaCommand("cd", [args?.path as string]);
        break;

      case "mega_pwd":
        result = await runMegaCommand("pwd");
        break;

      case "mega_mkdir": {
        const cmdArgs: string[] = [];
        if (args?.parents) cmdArgs.push("-p");
        cmdArgs.push(args?.path as string);
        result = await runMegaCommand("mkdir", cmdArgs);
        break;
      }

      case "mega_rm": {
        const cmdArgs: string[] = [];
        if (args?.recursive) cmdArgs.push("-r");
        if (args?.force) cmdArgs.push("-f");
        cmdArgs.push(args?.path as string);
        result = await runMegaCommand("rm", cmdArgs);
        break;
      }

      case "mega_mv":
        result = await runMegaCommand("mv", [
          args?.source as string,
          args?.destination as string,
        ]);
        break;

      case "mega_cp":
        result = await runMegaCommand("cp", [
          args?.source as string,
          args?.destination as string,
        ]);
        break;

      case "mega_get": {
        const cmdArgs: string[] = [args?.remote_path as string];
        if (args?.local_path) cmdArgs.push(args.local_path as string);
        result = await runMegaCommand("get", cmdArgs);
        break;
      }

      case "mega_put": {
        const cmdArgs: string[] = [args?.local_path as string];
        if (args?.remote_path) cmdArgs.push(args.remote_path as string);
        result = await runMegaCommand("put", cmdArgs);
        break;
      }

      case "mega_df": {
        const cmdArgs: string[] = [];
        if (args?.human) cmdArgs.push("-h");
        result = await runMegaCommand("df", cmdArgs);
        break;
      }

      case "mega_du": {
        const cmdArgs: string[] = [];
        if (args?.human) cmdArgs.push("-h");
        if (args?.path) cmdArgs.push(args.path as string);
        result = await runMegaCommand("du", cmdArgs);
        break;
      }

      case "mega_find": {
        const cmdArgs: string[] = [];
        if (args?.path) cmdArgs.push(args.path as string);
        cmdArgs.push("--pattern=" + (args?.pattern as string));
        result = await runMegaCommand("find", cmdArgs);
        break;
      }

      case "mega_export": {
        const cmdArgs: string[] = ["-a", args?.path as string];
        if (args?.expire) cmdArgs.push("--expire=" + args.expire);
        if (args?.password) cmdArgs.push("--password=" + args.password);
        result = await runMegaCommand("export", cmdArgs);
        break;
      }

      case "mega_share": {
        const cmdArgs: string[] = [
          "-a",
          "--with=" + (args?.email as string),
          "--level=" + ((args?.access_level as string) || "r"),
          args?.path as string,
        ];
        result = await runMegaCommand("share", cmdArgs);
        break;
      }

      case "mega_transfers": {
        const cmdArgs: string[] = [];
        if (args?.show_completed) cmdArgs.push("-c");
        result = await runMegaCommand("transfers", cmdArgs);
        break;
      }

      case "mega_sync": {
        if (args?.list_only) {
          result = await runMegaCommand("sync");
        } else if (args?.local_path && args?.remote_path) {
          result = await runMegaCommand("sync", [
            args.local_path as string,
            args.remote_path as string,
          ]);
        } else {
          result = await runMegaCommand("sync");
        }
        break;
      }

      case "mega_tree": {
        const cmdArgs: string[] = [];
        if (args?.path) cmdArgs.push(args.path as string);
        result = await runMegaCommand("tree", cmdArgs);
        break;
      }

      case "mega_cat":
        result = await runMegaCommand("cat", [args?.path as string]);
        break;

      case "mega_import": {
        const cmdArgs: string[] = [args?.link as string];
        if (args?.remote_path) cmdArgs.push(args.remote_path as string);
        result = await runMegaCommand("import", cmdArgs);
        break;
      }

      default:
        throw new Error(`Unknown tool: ${name}`);
    }

    return {
      content: [
        {
          type: "text",
          text: result,
        },
      ],
    };
  } catch (error: any) {
    return {
      content: [
        {
          type: "text",
          text: `Error: ${error.message}`,
        },
      ],
      isError: true,
    };
  }
});

async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("MEGA MCP server running on stdio");
}

main().catch(console.error);
