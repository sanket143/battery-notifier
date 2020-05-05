CONFIG_DIR=$HOME/.config/battery_notifier_s
SYSTEM_BIN="`systemd-path user-binaries`"

mkdir -p $HOME/.config
mkdir -p $CONFIG_DIR
mkdir $SYSTEM_BIN

cd $CONFIG_DIR
wget https://sanket143.github.io/battery-notifier/battery_notifier_s.service
wget https://sanket143.github.io/battery-notifier/config.json
wget https://github.com/sanket143/battery-notifier/releases/download/v0.1.0/battery_notifier_s.tar.gz

tar -xvf battery_notifier_s.tar.gz
cp battery_notifier $SYSTEM_BIN

