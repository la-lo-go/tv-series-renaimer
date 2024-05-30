# TV Series RenAImer

GPT powered CLI tool for renaming TV series files for better recognition by media players such as Plex, Jellyfin, etc.

## Download

1. Download the latest release from the [releases page](https://github.com/la-lo-go/TV-Series-RenAImer/releases)
1. Extract the downloaded file
1. Run the executable in the desired directory
    - On Windows:
  
        ```bash
        ./tv-series-renaimer.exe -k <API_KEY> [OPTIONS]
        ```

    - On Linux:
  
        ```bash
        ./tv-series-renaimer -k <API_KEY> [OPTIONS]
        ```

## Usage

| Option          | Description                                                                                                           | Default Value   |
| --------------- | --------------------------------------------------------------------------------------------------------------------- | --------------- |
| -k, --key       | REQUIRED: GPT API key (get it from <https://platform.openai.com/>)                                                    | None            |
| -p, --path      | Path to the directory that will be scanned                                                                            | "./"            |
| -m, --mode      | Mode to run the program in (recursive or single)                                                                      | "recursive"     |
| -g, --gtp_model | Model to use, must be one with JSON mode enable (<https://platform.openai.com/docs/guides/text-generation/json-mode>) | "gpt-4o" |

## Development

- Run the rust code

    ```bash
    cargo run -- -k <API_KEY> 
    ```

- Run unit tests

    ```bash
    cargo test
    ```

## License

[![License: MPL-2.0](https://img.shields.io/badge/License-MPL--2.0-blue.svg)](LICENSE)
