# Tray File Format
The Tray file format is a derivative of [`kdl`](https://kdl.dev/).

# Capture
Denotes any simple capture, with an attached header. Can contain a body and be sleeping.
## Syntax
```
capture <header> {
  [sleeping <until=DateTimeString>]
  [body <content>]
}
```

# Task
Denotes any general task. Requires a header describing the task. Can also contain a body, be sleeping, have a scheduled date, have a deadline, or be waiting.
```
task <header> {
  [sleeping <until=DateTimeString>]
  [body <content>]
  [scheduled <DateTimeString>]
  [deadline <DateTimeString>]
  [waiting <since=DateTimeString>]
}
```

# Next

# Done

# Event

# Project



# See Also:
- https://kdl.dev/
