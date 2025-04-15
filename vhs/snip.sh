#!/usr/bin/env bash

cargo install --path .

vhs vhs/mot.tape

cp target/mot.gif ../

git checkout images

mv ../mot.gif images/

git add .
git commit -m "Update mot.gif"
git push

git checkout main

xdg-open https://raw.githubusercontent.com/Tuurlijk/apisnip/refs/heads/images/images/mot.gif