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
Denotes any next task. It is exactly the same as `Task`, but semantically denotes a different state. Requires a header describing the task. Can also contain a body, be sleeping, have a scheduled date, have a deadline, or be waiting.
```
next <header> {
  [sleeping <until=DateTimeString>]
  [body <content>]
  [scheduled <DateTimeString>]
  [deadline <DateTimeString>]
  [waiting <since=DateTimeString>]
}
```

# Done
Denotes any completed Task. It is almost the same as `Task`, but includes a `completed` property, as well as semantically denoting a different state. Requires a header describing the task. Can also contain a body, be sleeping, have a scheduled date, have a deadline, or be waiting.
```
done <header> <completed=DateTimeString> {
  [sleeping <until=DateTimeString>]
  [body <content>]
  [scheduled <DateTimeString>]
  [deadline <DateTimeString>]
  [waiting <since=DateTimeString>]
}
```

# Event
Denotes an event. Requires a header describing the event, as well as a datetime string. Can also contain a body, a duration, and a location.
```
event <header> <on=DateTimeString> {
  [duration <DateTimeString>]
  [body <content>]
  [location <content>]
}
```

# Project
Denotes a project. Requires a header describing the project. Can contain any set of `Task`, `Done`, `Next`, or other `Project`s. A project is considered completed if every entry is `Done`. A project is in warning state if there is no `Next` entry, either top level or in child `Project`s.
```
project <header> {
  <children>
}
```


# See Also:
- https://kdl.dev/
