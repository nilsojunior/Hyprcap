#!/usr/bin/env bash

hyprctl clients -j | jq -r '.[] | select(.workspace.id != null) | .at,.size' | jq -s 'add' | jq '_nwise(4)' | jq -r '"\(.[0]),\(.[1]) \(.[2])x\(.[3])"' | slurp
