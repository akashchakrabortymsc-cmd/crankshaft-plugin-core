# crankshaft-plugin-core

Shared types and contracts for the Crankshaft plugin system.

This crate is the foundation that every other plugin crate depends on.


---

## What is inside this crate

### `JobId`

A unique identifier for a submitted job.

```rust
let id = JobId::new("job-abc-123".to_string());
println!("{}", id); // job-abc-123
```

---

### `JobStatus`

The current state of a job.

```rust
pub enum JobStatus {
    Pending,        // submitted, not yet running
    Running,        // actively executing
    Completed,      // finished successfully
    Failed(String), // finished with error — carries the error message
    Cancelled,      // stopped before completion
}
```

---

### `Job`

The unit of work sent to a plugin for execution.

```rust
let job = Job::new(
    JobId::new("job-001".to_string()),
    "echo hello".to_string(),
);
```


---

### `PluginBackend` trait

The contract every plugin must implement.

```rust
pub trait PluginBackend {
    fn submit(&self, job: Job) -> PluginResult;
    fn status(&self, id: &JobId) -> PluginResult;
    fn cancel(&self, id: &JobId) -> PluginResult;
}
```

If you are writing a new backend plugin, you implement this trait.

---

### `PluginError`

All the ways a plugin interaction can fail.

```rust
pub enum PluginError {
    ConnectionFailed(String),  // could not reach plugin process
    JobNotFound(String),       // no job with this ID
    InvalidResponse(String),   // plugin sent unexpected data
    Timeout,                   // plugin did not respond in time
    Unknown(String),           // anything else
}
```


## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE))

