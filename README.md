# Netherite
A Minecraft server implementation in Rust

## Motivation

The main motivation for this project can be summarized in 3 points:
1. The author(s) have great interest in the Rust programming language, and the advantages it proposes over Java and other system languages such as C and C++
2. The bloated nature of Mojang's server implementation makes it unsuitable for servers that only utilize certain features of Minecraft (in which unneeded features consume otherwise valuable system resources)
3. An open source project with a permissive license that benefits everybody, from server owners, to developers, to system administrators.

This project has been heavily inspired by [Minestom](https://github.com/Minestom/Minestom), and aims to provide something similar in the Rust ecosystem.

## Status
Netherite implements the protocol of **Minecraft 1.19.2**, and is still under heavy development.

- [x] Basic Minecraft protocol
  - [x] Reading packets
  - [x] Writing packets
  - [x] Encryption
  - [x] Compression
  - [ ] Handling of invalid packets or out-of-order ones.
- [-] Region system
  - [x] Chunks
  - [ ] World data
- [ ] Entity component system
- [ ] A suitable NBT framework
- [ ] Plugin API
- [ ] A powerful, user-friendly CLI for controlling the server, installing extensions, etc.

## Goals
1. A stable, powerful, possibly cross-version implementation of Minecraft's protocol, NBT system, chunks, and other game-related formats
2. An API for extending gameplay
3. Security and performance.

### Why don't you just contribute to [insert an existing implementation here]?
We are aware that there are ongoing attempts in building a Minecraft server implementation in Rust already, however:
1. **We have different goals**: Some implementations aim on providing a vanilla-like experience. Others have been made to replace a certain niche of servers, and work alongside the Mojang ones. We believe in providing an extensible, bare-minimum implementation for minigame servers.
2. **They _may_ be outdated**: Minecraft's protocol changes a lot on each major version bump, and most of the community prefers running on the latest version. We believe we have an edge as we've started working on netherite only a little after 1.19.2 was released.
3. **We love to learn**: The best way to master a difficult systems programming language is by building with it. We've decided that a project with the magnitude of a Minecraft server ought to provide an excellent experience to the underlying concepts behind building this fantastic game
