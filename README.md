# Task Orchestrator

A Rust-based task orchestration service that executes multi-step background tasks concurrently.

## Usage

```bash
cargo run -- tasks.csv > results.csv
```

## Requirements

- Rust stable (tested on Linux)
- Network access to `https://httpbin.org/get`

## How It Works

The orchestrator reads task definitions from a CSV file, executes each unique task with the following steps:
1. **fetch_data**: HTTP GET request to `https://httpbin.org/get`
2. **long_delay**: 5-second pause
3. **emit_event**: Prints confirmation message to stderr

All tasks are executed concurrently using `tokio::spawn` for improved performance.

## Assumptions

- Duplicate `task_id` values in the input CSV are automatically deduplicated (only unique task IDs are processed)
- The `task_type` column in the input CSV is read but ignored (as specified in requirements)
- HTTP requests use default `reqwest::Client` settings (no custom timeouts or retries)
- Tasks execute concurrently rather than sequentially for better performance
- CSV output row order is not guaranteed (as per requirements)
- Error messages are captured as strings in the `error_info` field

## Trade-offs Considered

- **Concurrent vs Sequential Execution**: Chose concurrent execution using `tokio::spawn` to improve performance. With N tasks each taking ~5 seconds, concurrent execution completes in ~5 seconds total vs N×5 seconds sequentially.
  
- **Error Handling**: Used `String` for error messages rather than custom error types. Simpler and sufficient for this use case, though less structured than a full error type hierarchy.

- **Custom Serialization**: Implemented custom `Serialize` for `TaskStatus` enum to ensure exact string output ("Completed"/"Failed") rather than using a plain `String` type. Provides type safety while meeting output format requirements.

- **No HTTP Timeouts/Retries**: Kept implementation simple without retry logic or custom timeouts. For production use, these would be valuable additions.

- **tokio::spawn vs futures::join_all**: Chose `tokio::spawn` since tokio was already a dependency, avoiding an additional crate.

## Dependencies

- `tokio`: Async runtime for concurrent task execution
- `reqwest`: HTTP client for fetch_data step
- `csv`: CSV file reading and writing
- `serde`: Serialization/deserialization for CSV handling

## AI-Assisted Development

This project was developed with AI-assisted tools (Cursor AI) for code generation, explanation, and guidance throughout the implementation process.

