#!/bin/sh

agg_present() {
	agg -v --theme asciinema --font-family "Iosevka Term Slab,Apple Color Emoji" --font-size 20 --speed 2 --cols 120 $1 $2
}

agg_present hi.cast hi.gif &
agg_present demo1.cast demo1.gif &
agg_present demo2.cast demo2.gif &

wait
