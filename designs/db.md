# DB

## Overview

A yaml file will work as the "Database"

## Design and Implementation

### load db

### write db

### schema

```yaml
- Acronym
    - description
- GDB
    - Gnu De-Bugger is a cli debugging program used primarily with c and and c++ on linux.
- TLA
    - Three Letter Acronym
```

## Risk Analysis

- What rust package to use to read and write yaml files? serde_yaml
- Is it possible to modify the file in line with out read the entire contents and then overwriting back the entire contents?
