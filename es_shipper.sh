#!/bin/sh -x
#
# $FreeBSD:
#

# PROVIDE: es_shipper
# REQUIRE: LOGIN
# KEYWORD: shutdown

# Add the following line to /etc/rc.conf to enable `blog':
#
#es_shipper_enable="YES"
#

. /etc/rc.subr

name="es_shipper"
rcvar=`set_rcvar`

# this should probably be changed to /usr/local/etc/es_shipper
# (Your config.yaml should be placed here.)
es_shipper_chdir="/etc/es_shipper"

# read configuration and set defaults
load_rc_config "$name"

es_shipper_enable=${es_shipper_enable:-"NO"}

pidfile="/var/run/${name}.pid"
command="/usr/sbin/daemon"
command_args="-f -p ${pidfile} /usr/local/bin/es_shipper"

run_rc_command "$1"