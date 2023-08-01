#!/bin/sh
STAGED=$(git diff --name-only --cached | grep '.*\.rs')
if ! [ "$STAGED" = '' ]; then
    rustfmt --check "$STAGED" || {
        echo -e "\e[31mPlease run rusftmt on all staged files before committing\e[0m" ; exit 1
    }
fi