FROM rust:1.66

RUN apt-get update
RUN apt-get install -y bluez libbluetooth-dev dbus libdbus-1-dev
RUN sed 's/^\(ExecStart.*\)/\1 --enable-testing/' /etc/systemd/system/bluetooth.target.wants/bluetooth.service

COPY . .
