# scaffolder

A helpful scaffolder for any kind of project

Example `scaffolder.yml`

```yml
projects:
    ### Typescript Webpack starter
    - names: [rust-wasm-web]
      description: A Rust wasm starter project bundled using vite
      steps:
          - !command
            command: yarn
            args: [create, vite, ., --template, vanilla-ts]
          - !command
            command: yarn
            args: []
          - !template
            template: package.json
            file: package.json
            replacements:
                main: ""
                type: module
                scripts: |4
                        "dev": "wasm-pack build ./innards --target web && yarn build && tsc && yarn preview",
                        "build": "vite build",
                        "preview": "vite preview"
          - !command
            command: yarn
            args: [add, -D, vite, vite-plugin-wasm-pack, typescript]
          - !command
            command: cargo
            args: [init, --lib, innards]
          - !create
            file: Cargo.toml
            contents: |
                [workspace]
                members = ["innards"]
          - !copy
            from: rust-wasm-web/vite.config.ts
            to: vite.config.ts
          - !copy
            from: rust-wasm-web/main.ts
            to: src/main.ts
          - !copy
            from: rust-wasm-web/lib.rs
            to: innards/src/lib.rs
          - !append
            file: innards/Cargo.toml
            contents: |


                [lib]
                crate-type = ["cdylib"]
          - !append
            file: .gitignore
            contents: |

                target
          - !remove
            file: public/vite.svg
          - !remove
            file: src/typescript.svg
          - !remove
            file: src/counter.ts
          - !multicommand
            command: >
                cd innards
                && cargo add wasm-bindgen
                && cargo add console_error_panic_hook
```
