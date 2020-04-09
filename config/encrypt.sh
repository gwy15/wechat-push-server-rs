#!/bin/bash
gpg --yes --symmetric --cipher-algo AES256 config/config.ci.local.toml
