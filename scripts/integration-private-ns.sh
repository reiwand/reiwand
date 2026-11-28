#!/usr/bin/env bash
set -euo pipefail
declare ROOT WORK OUTER_UID OUTER_GID
ip link set lo up
ip link add out0 type dummy
ip link set out0 addrgenmode none
ip link set out0 up
ip addr add 10.77.0.1/24 dev out0
ip -6 addr add fd77::1/64 dev out0 nodad
ip route add default via 10.77.0.2 dev out0
ip -6 route add default via fd77::2 dev out0
printf 'nameserver 10.77.0.53\nnameserver fd77::53\n' >"$WORK/private/resolv.conf"
printf '127.0.0.1 localhost\n' >"$WORK/private/hosts"
mount --bind "$WORK/private/resolv.conf" /etc/resolv.conf
mount --bind "$WORK/private/hosts" /etc/hosts
exec unshare --user --map-user="$OUTER_UID" --map-group="$OUTER_GID" bash "$ROOT/scripts/integration-private-run.sh"
