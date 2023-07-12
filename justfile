[private]
alias p := proj
[private]
alias c := clean-up

proj *args:
    cargo run -- {{ args }}

clean-up dir="sample":
    poetry -C "./{{ dir }}" env remove 3.11
    rm -rf {{ dir }}
