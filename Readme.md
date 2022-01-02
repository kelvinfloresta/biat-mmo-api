# Biat API

## Watching your tests:
  Install dependency
  ```sh
  cargo install cargo-watch
  ```

  Run watch
  ```sh
  cargo watch -x 'test --tests'
  ```

  If you want to see stdout
  ```sh
  cargo watch -x 'test --tests -- --nocapture'
  ```