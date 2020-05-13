CONFIG_DIR=$HOME/.config/battery_notifier_s
SYSTEM_BIN="`systemd-path user-binaries`"

mkdir -p $HOME/.config
mkdir -p $CONFIG_DIR
mkdir -p $SYSTEM_BIN

cd $CONFIG_DIR
wget https://sanket143.github.io/battery-notifier/config.json
wget https://github.com/sanket143/battery-notifier/releases/download/v0.1.1/battery_notifier_s.tar.gz

tar -xvf battery_notifier_s.tar.gz
mv battery_notifier $SYSTEM_BIN

rm battery_notifier_s.tar.gz
