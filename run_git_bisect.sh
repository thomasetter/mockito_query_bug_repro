#!/bin/bash
git submodule update --init
cd mockito
git bisect start 0.21.0 0.17.0
git bisect run ../bisect_helper.sh