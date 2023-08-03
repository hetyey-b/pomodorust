# Pomodorust
Console based pomodoro timer written in Rust

## Requirements
- Unix operating system
- cargo for compiling the Rust code
- a ~/Music/pomodoro_notification.wav sound file
- playerctl command
- aplay command

## Installation & Usage
Clone the repository. Run `cargo build` in the /src directory. It should result in the creation of an executable named `pomodorust` in the `src/target/debug` directory. You can run this file from the command line, passing in the number of work/rest cycles you want. E.g.:

```
pomodorust 4
```

You need to have a .wav file for aplay to play when a work cycle ends. The program searches for it in ~/Music/pomodoro_notification.wav

## Features
- If any media is playing when the work cycle finishes, pomodorust will automatically pause it, then resume it when you start the next work cycle
- You can specify how many cycles you want when starting the program
