#!/bin/bash

while true; do

 P="$1"

 shift || break;

 mkdir -p "$(dirname "$P")" && touch "$P"

done

