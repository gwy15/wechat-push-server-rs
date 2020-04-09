#!/bin/bash
gpg --quiet --batch --yes --decrypt --passphrase="$SECRET_PASSPHRASE" \
--output config/config.ci.local.toml config/config.ci.local.toml.gpg
