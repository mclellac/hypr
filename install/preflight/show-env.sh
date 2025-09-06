#!/bin/bash

echo "Installation ENV:"
env | grep -E "^(hypr_CHROOT_INSTALL|hypr_USER_NAME|hypr_USER_EMAIL|USER|HOME|hypr_REPO|hypr_REF)="
