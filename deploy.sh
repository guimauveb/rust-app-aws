#!/bin/bash
ssh -i ~/.ssh/my-key.pem ec2-user@ec2-instance.compute.amazonaws.com '
    cd ~/rust-app/deploy &&
    ./run.sh > ~/logs/rust-app-deploy 2>&1
'
