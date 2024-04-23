# TV Series RenAImer
AI powered CLI tool for renaming TV series files for better recognition by media players such as Plex, Jellyfin, etc.

> [!CAUTION]
> These tool is still in development and may not work as expected. Use at your own risk.

## Usage
| Option          | Description                                                      | Default Value |
| --------------- | ---------------------------------------------------------------- | ------------- |
| -k, --key       | REQUIRED: GPT API key (get it from https://platform.openai.com/) | None          |
| -p, --path      | Path to the directory that will be scanned                       | "./"          |
| -m, --mode      | Mode to run the program in (recursive or single)                 | "recursive"   |
| -g, --gtp_model | GPT model to use (see https://platform.openai.com/docs/models)   | "gpt-4-turbo" |

## Development
1. Run the rust code
```bash
    cargo run -- -k <API_KEY> 
```

## License
[![License: MPL-2.0](https://img.shields.io/badge/License-MPL--2.0-blue.svg)](LICENSE)