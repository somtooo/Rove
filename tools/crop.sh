#!/usr/bin/env bash
# Side-by-side crops: reference (left) vs Rove (right), magnified.
# usage: tools/crop.sh SCENE X Y W H [ZOOM] OUT.png   (window-relative logical px)
set -euo pipefail
scene=$1; x=$2; y=$3; w=$4; h=$5; zoom=${6:-400}; out=${7:-out/crop.png}
root=$(cd "$(dirname "$0")/.." && pwd)
ref="$root/.plan/reference/screens/grid-$scene@2x.png"
mine="$root/out/cmp-$scene.png"
geo="$((w*2))x$((h*2))+$(((x+40)*2))+$(((y+40)*2))"
magick \( "$ref" -crop "$geo" +repage -filter point -resize "$zoom%" \) \
       \( "$mine" -crop "$geo" +repage -filter point -resize "$zoom%" \) \
       -background '#FF00FF' -splice 8x0 +append "$out"
