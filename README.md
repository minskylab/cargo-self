# Cargo Self

This project is a modular and extensible computation framework designed to manage, process, and execute computational plans. Leveraging the power of OpenAI's GPT-3.5-turbo, it can generate chat completions based on structured input, following a set of specified rules defined in a constitution. The software is organized into well-defined modules each handling specific aspects of the system, like state management, JSON persistence, planning, and processing of computational units.

## Features

- Computation Planning: Define and manage computational plans with a specified root path and persistence mechanism.
- Dynamic Chat Completion: Utilize OpenAI's GPT-3.5-turbo to generate chat completions based on structured input.
- JSON Persistence: Store and retrieve the state of the computation in a JSON format.
- Modular Design: Extensible architecture with clear separation of concerns among different modules such as planning, state management, and JSON persistence.
- CLI Tool: A command-line interface for interacting with the computation engine and executing plans.
- Constitution-based Processing: Define a constitution to guide the computation process, with the ability to retrieve system and input data for constitution-based processing.

## Modules

- src/engine: Core engine modules handling state management, JSON persistence, planning, and constitution-based processing.
- src/cli: Command-line interface for interacting with the computation engine.
- src/lib.rs: Library root, tying together the CLI and engine modules.
- src/main.rs: Entry point for initializing and executing the computation engine.

## Dependencies

- async-openai, cargo, clap, dotenv, handlebars, ignore, serde, serde_json, sha2, tokio, walkdir, and others.

## Usage

Refer to the README.md for detailed instructions on how to use this software, including setting up, configuring, and executing computational plans.

## Contribution

Feel free to fork this repository, submit issues, or open pull requests to improve the project.
