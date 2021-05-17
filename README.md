### Environment Setup
1. Install Rust from https://rustup.rs/
2. Install Solana v1.6.2 or later from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

### Build and test for program compiled natively
```
$ cargo build
$ cargo test
```

### Build and test the program compiled for BPF
```
$ cargo build-bpf
$ cargo test-bpf
```

### Testing locally

```
# 1. Start validator
solana-test-validator

# 2. Generate an account to play with
solana-keygen new -o test.json

# 3. Give your account some SOL
solana airdrop --url http://127.0.0.1:8899 --keypair test.json 100

# 4. Deploy
solana program deploy --url http://127.0.0.1:8899 --keypair test.json target/deploy/half_baked.so

> Program Id: 6EgWgFtrCcFyhsGLmpQ7sQPCXp4sY3CXEUhSkLjwpGCh
```