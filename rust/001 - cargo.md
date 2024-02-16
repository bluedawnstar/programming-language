# Rust

## Cargo 사용법
  - 버전 확인
    ```bash
    $ cargo --version
    ```

  - 실행 가능 application 프로젝트
    - 생성
      ```bash
      $ cargo new hello-cargo --bin
      $ cd hello-cargo
      ```
    - 컴파일 되는지만 확인
      ```bash
      $ cargo check
      ```
    - 빌드 & 실행 (debug)
      ```bash
      $ cargo build
      ```
      ```bash
      $ ./target/debug/hello-cargo
      ```
      ```bash
      $ cargo run
      ```
    - 빌드 & 실행 (release)
      ```bash
      $ cargo build --release
      ```
      ```bash
      $ ./target/release/hello-cargo
      ```
      ```bash
      $ cargo run --release
      ```

  - lib 프로젝트
    - ...