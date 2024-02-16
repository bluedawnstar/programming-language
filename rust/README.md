# Rust

## 설치하기
  - Linux와 Mac에 설치
    - Rustup 설치
      ```bash
      $ curl https://sh.rustup.rs -sSf | sh
      ```
      ```bash
      $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      ```
    - Mac에 rust 설치
      ```bash
      $ brew install rust
      ```
    - Ubuntu에 rust 설치
      ```bash
      $ sudo apt install rustc cargo
      ```
    - 환경변수 설정
      ```bash
      $ source $HOME/.cargo/env
      ```
      shell profile(예: ~/.bash_profile)에 path 추가
      ```bash
      $ export PATH="$HOME/.cargo/bin:$PATH"
      ```

  - Windows에 설치
    - ```https://www.rust-lang.org/tools/install``` 에서 다운로드 후 설치
    - 설정 안되어 있을 경우, path에 ```$HOME\.cargo\bin``` 추가

  - 업데이트
    ```bash
    $ rustup update stable
    ```

  - 버전 확인
    ```bash
    $ rustc --version
    ```


    

## examples
  - Hello, world!
    - hello.rs
      ```Rust
      // hello.rs
      fn main() {
          println!("Hello, world!");
      }
      ```
    - build
      ```bash
      $ rustc hello.rs
      ```
