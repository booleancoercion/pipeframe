# pipeframe
Rust library for constructing videos from individual frames, using an already installed instance of ffmpeg.

This library exposes a convenient API for creating videos, to save the pain of working with raw stdin.
Note that due to the nature of pipeframe, it shells out to `ffmpeg`, and as such you will also need to have it installed on your system.
