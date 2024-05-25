#!/usr/bin/env bash

echo "Pick a cluster to work on:"
CLUSTER=`echo "kube-dev-01,kube-dev-02,kube-sre-01,kube-staging-01" | pick -d ','`
echo "Working on $(echo $CLUSTER)"

# do whatever you need with saved selection
# ie kubectl --cluster $(echo $CLUSTER) get pods