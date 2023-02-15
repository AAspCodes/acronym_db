# Text User Interface

## Overview

The layout of the user interface to allow the user to add, modify, and delete acronyms. As well as get the man page and see usage examples. And lastly search the database for acronyms.

### usage

- commands to display usage info

```text
adb
adb <something invalid>
```

### help

- command to display help/man page

```text
adb help
adb -h
adb --h
adb -help
adb --help
```

### add acronym

```bash
adb add <acronym>
adb add <acronym> -d <description>
```

### modify acronym

```text
adb modify <acronym>
adb -m <acronym>
```

### delete acronym

```text
adb delete <acronym>
adb -d <acronym>
```

### search for acronym

```text
adb search <acronym> <additional acronyms>
adb -s <acronym> <additional acronyms>
```

## Design and Implementation

## Risk Analysis

- What rust package to use to create a TUI?
- Can I have an add with out a description then open a vim text editor where the description can be entered?
