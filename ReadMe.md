# Guide

This setup contains three servers which are bin crates

- app: This this the api gateway of the application
- notify: This is the notification service
- core: This houses the task, chat and notes section of the app

- The underscore `_` is used to discard a return value.
- `is_ok()` is associated with results. Used to check if a result doesn't return an error;

---

- `rm -rf ~/.cargo/registry/index/* ~/.cargo/.package-cache`: Cargo Crate
