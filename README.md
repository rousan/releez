<div align="center">
  <a href="https://github.com/rousan/releez">
    <img width="200" height="200" src="https://avatars3.githubusercontent.com/u/63495711?s=200&v=4">
  </a>
  <br />
  <br />
</div>

[![Crate](https://img.shields.io/crates/v/releez.svg)](https://crates.io/crates/releez)
[![Contributors](https://img.shields.io/github/contributors/rousan/releez.svg)](https://github.com/rousan/releez/graphs/contributors)
[![MIT](https://img.shields.io/crates/l/releez.svg)](./LICENSE)


# releez

An utility tool to run application release-checklist safely.

## Why to use releez?

We all document a checklist prior to an application release to avoid any mistakes. Somebody writes them to a file called `release-checklist.txt` or someone documents them somewhere else.
So, two types of tasks can be involved during a release:

1. **Automated**: these tasks are automated though running scripts.
2. **Manual**: these tasks require manual efforts as these can't be automated or very difficult to automate.

This way, we have to remember which step we just finished, and also if some automated tasks breaks somewhere, it is difficult to find the fault point.
We make mistakes and it's normal. But if some tool runs those automated and manual tasks instead, and if it tracks the release process so that if some tasks
failed to completion it stores the state and it allows to resume the release process from where THE FAULT happened, then it would be a life saver.

Here, comes the `releez` tool which does exactly what it means. It requires a config file named `releez.yml` containing the release checklist. This config file is kind of alternative
to our `release-checklis.txt` file or whatever we use to document the checklist. You have to just mention the task name and the task type (`auto` or `manual`), if it's an automated task then
you have to mention the commands to run or if it's a manual task, then you have to write the instructions or guide to do that task manually. That's it, it will run you checklist and tracks
the progress and it can also resume the release if you want.

## Install

### macOS

```sh
 $ bash -c "$(curl -fsSL https://raw.githubusercontent.com/rousan/releez/master/install.sh)"
```

### Linux

```sh
 $ bash -c "$(curl -fsSL https://raw.githubusercontent.com/rousan/releez/master/install.sh)"
```

### Windows

Please download it from [releases](https://github.com/rousan/releez/releases) page.

## Getting Started

After installation, you can follow the steps below to get started with `releez` tool.

First, you need to create a file `releez.yml` at the root directory of the project and write the checklist in following format:

An example `releez.yml` file
```yaml
version: 1.0.0
checklist:
  - name: "First Manual Task"
    type: "manual"
    instructions:
      - "Write the instructions or guide to do this manual task"
      - "Another instruction"
      - "You can access next release version as $VERSION here"
      - "You can also  embed system environment variables in here e.g. $USER or $PWD"
  - name: "You can also write instructions based on operating systems"
    type: "manual"
    instructions:
      macos + linux:
        - "This instruction will be shown on linux and macOS system"
      windows:
        - "This instruction will be shown on Windows system only"
  - name: "An automated task e.g. Building the project"
    type: "auto"
    run:
      - echo "Build the project"
      - echo "You can also access release version as $VERSION"
      - npm build
  - name: "You can also write different commands for different Operating Systems"
    type: "auto"
    run:
      macos + linux:
        - echo "commands for macOS and linux"
        - echo "commands for macOS and linux"
      windows:
        - echo "commands for Windows only"
  - name: "Another auto task, but it will ask to confirm before executing commands"
    confirm: "Do you want to proceed this task to be run?"
    type: "auto"
    run:
      - echo "Releasing..."
      - npm publish
```

Then, when you're ready to release the app, you need to run the following command at the root directory of the project with the next release version e.g. `v1.0.2` :

```sh
$ releez v1.0.2
```

That's it. It will now run these checklist and it will make sure to store the release status and if a task failed to run, it can also resume the release process from that breaking point.

## Contributing

Your PRs and stars are always welcome.