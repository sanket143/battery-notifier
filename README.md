# Battery-Notifier

To Install
```sh
$ curl -S https://sanket143.github.io/battery-notifier/install.sh | bash -E
```

Run daemon
```sh
$ battery_notifier
```

You'll have to trigger it everytime the OS boots up.

When you update your `config.json`, restart the daemon.

```sh
$ killall battery_notifier
$ battery_notifier
```
