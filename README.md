# Node Stats
Simple project for investigating/learning a bit of svelte and rust.

The idea is to run various commands on each node exposing the results using a daemon set running a web end point.  Two simple backend implementations exist, one using node and the other rust (tide).  See `backend-node` and `backend-rust` respectively.

A simple svelte frontend is provided to show results collected from each node.  See `frontend`.

# Commands to run
These are specifed in an input json file containing a list of key -> command entries; commands are run to produce the value to be displayed in the frontend.  See `commands.json`.

One of the commands should have the key `name`, it's value will be used as the `node name` when unpacking on the frontend.

# Kubernetes/Docker (in progress)
Dockerfiles are provided for the rust backend and the frontend.

DaemonSet, Deployment and Service definitions are provided for the following:
- `backend-rust`: DaemonSet to run the backend on each node, Service of type None to get the k8 DNS to provide an A record containing all members in the DaemonSet
- `frontend`: Deployment to run the frontend on a single node, Service of type LoadBalancer set up to register with metallb (see my other projects https://github.com/georgenicoll/ansible for setup details and https://github.com/georgenicoll/coredns/tree/master/plugin/k8s_ext_dhcp for k8 aware local network DNS)
