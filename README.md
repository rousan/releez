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

We all document a checklist prior to an application release to avoid any mistakes. Somebody writes to a file called `release-checklist.txt` or someone documents it somewhere else.
So, two types of tasks can be involved during a release:

1. **Automated**: these tasks are automated though running scripts.
2. **Manual**: these tasks require manual effort as these can't be automated or very difficult to automate.

This way, we have to remember which step we just finished, and also if some automated tasks breaks somewhere, it is difficult to find the fault point.
We make mistakes and it's normal. But if some tool runs those automated and manual tasks instead, and if it tracks the release process so that if some tasks
failed to completion it stores the state and if it allows to resume the release process from where THE FAULT happened, it would be a life saver.

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

## Documentation



## Contributing

Your PRs and stars are always welcome.