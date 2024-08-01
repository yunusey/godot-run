# godot-run

Easily run and test Godot applications that you see on GitHub on your terminal.

## Usage 🛠️

`godot-run` is a simple CLI tool that can be used to run and test Godot applications that you see on GitHub quickly on your terminal. Using it is pretty simple:
```
Usage: godot-run [OPTIONS] <repository>

Arguments:
  <repository>  The repository to clone from GitHub (ex: "github.com/yunusey/gosu")

Options:
      --godot-path <GODOT_PATH>
          The path to the Godot executable (tries to find it atomatically, if not specified) (ex: --godot-path="/home/yunusey/.nix-profile/bin/godot4") [default: ]
  -e, --extra-arguments <EXTRA_ARGUMENTS>
          Extra arguments to pass to the Godot executable (learn more at https://docs.godotengine.org/en/stable/tutorials/editor/command_line_tutorial.html) (ex: --extra-arguments="--resolution 1920x1080") [default: ]
  -h, --help
          Print help
  -V, --version
          Print version

```

## Repository Cloning 🚚
Something nice about `godot-run` is that you can give it a subpath to the repository, and it will work flawlessly (hopefully :D). For instance, you can run `godot-run github.com/godotengine/godot-demo-projects/tree/master/gui/theming_override`. Just make sure that the subpath you give has the `project.godot` file. The format of the repository can be any of the following:
- https://www.github.com/<owner\>/\<repository\>
- https://www.github.com/<owner\>/\<repository\>/tree/\<branch\>/\<subpath1\>/\<subpath2\>/...
- www.github.com/<owner\>/\<repository\>/tree/\<branch\>/\<subpath1\>/\<subpath2\>/...
- \<owner\>/\<repository\>
- \<owner\>/\<repository\>/tree/\<branch\>/\<subpath1\>/\<subpath2\>/...

You got the point! Just write it somehow, and hopefully `godot-run` will handle the rest.

> [!warning]
> Currently, `godot-run` clones the **entire** repository regardless of whether or not you specified a subpath.

> [!warning]
> I haven't implemented a caching system yet, as I've just made the program for *very* basic use cases; but if you are interested, please let me know by [opening an issue](https://github.com/yunusey/godot-run/issues) or just implement it yourself and don't forget to [open up a PR](https://github.com/yunusey/godot-run/pulls)!

## Godot Executable 🤖

In order to run the program, we need the Godot executable, which you can give to the program by specifying the `--godot-path` argument. If you don't specify it, the program will try to find the executable by looking at the paths on your `PATH`. However, on the operating systems like Windows, you may have to pass it manually as it may not know where it is exactly at.

## Passing Extra Arguments to Godot 📝

Okay, it is pretty obvious that we are just spawning a subprocess that runs Godot from command-line, which you can find more information about [here](https://docs.godotengine.org/en/stable/tutorials/editor/command_line_tutorial.html). And you may want to, for instance, run the program in fullscreen mode. How do you do so? Pretty simple, actually! You run this:
```bash
godot-run github.com/yunusey/gosu --extra-arguments="--fullscreen"
```

## Future Plans 📚
- [ ] Add a caching system
    - [ ] Add the flag `--save-dir` to save the repositories to a specific directory
    - [ ] Add the flag `--no-cache` to disable the caching
    - [ ] Check the last commit in the remote repository, and if it's literally the same, don't clone it again
    - [ ] Add the flag `--update-cache` to update the cache no matter what the last commit is

- [ ] If possible, just clone the subpath instead of the entire repository.

## References
I would like to thank all these libraries and software that were used in the development of this project:
- [Godot Game Engine](https://godotengine.org/) for the most amazing game engine ever made
- [Clap](https://github.com/clap-rs/clap) for argument parsing
- [Git2](https://github.com/rust-lang/git2-rs) for libgit2 bindings in Rust
- [RegEx](https://github.com/rust-lang/regex) for parsing the repository path
- [Which](https://docs.rs/which/latest/which/) for finding the path to the Godot executable
