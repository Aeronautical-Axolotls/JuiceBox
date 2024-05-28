# JuiceBox Developer Set up

This document serves as a guide for developers on JuiceBox to set up the GitHub Repo, start project on their local machine, and how to build releases. This guide assumes intermediate Rust and Git knowledge, and will not be covering those as such.
**Note for grading: Professor Cameron should already have access to all things (we set it up before hand). All cargo commands are CLI.**

# Prereqs

 - Make sure Rust is installed: https://www.rust-lang.org
 - Have an active, SSH verified GitHub account: https://github.com
 - Request Access to Organization: https://github.com/Aeronautical-Axolotls 
 - Request access to Jira: Give email to Scrum Master.

## Development Setup

 1. Pull JuiceBox repo to local: https://github.com/Aeronautical-Axolotls/JuiceBox
 2. Use following command to set-up local version of project :`cargo build`
 3. Project set-up is finished, use prefered devleopment tools for engineering.

## Release Building

 Go to `Cargo.toml` and make the following change: 
	 `bevy = { version = "0.12.0", features = ["dynamic_linking"] }`
	 To
    `bevy = { version = "0.12.0"}`
Then, run the following command: `cargo build --release`
This will create a folder named `target`. Navigate to the `target/release` and replace the `assets` folder there with the `assets` folder from the repo parent folder (or you can copy the contents). Then click on `juice_box.exe` to test that it compiles with all assets. If so, then add release to the GitHub release build as most recent release. 
 

