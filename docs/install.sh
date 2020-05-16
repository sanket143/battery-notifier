CONFIG_DIR=$HOME/.config/battery_notifier_s
SYSTEM_BIN="`systemd-path user-binaries`"

mkdir -p $HOME/.config
mkdir -p $CONFIG_DIR
mkdir -p $SYSTEM_BIN

cd $CONFIG_DIR
wget https://sanket143.github.io/battery-notifier/config.json -O config.json
wget https://github.com/sanket143/battery-notifier/releases/download/v0.1.3/battery_notifier -O battery_notifier

chmod +x battery_notifier
mv battery_notifier $SYSTEM_BIN
