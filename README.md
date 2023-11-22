# rust_intro
Intro to Rust using a container image


### Set up local container for development  

I'm using podman so it is slightly different but nearly the same as docker. 

`podman pull rust:alpine3.17`  

Then run a container to test that I got a working environment. Checking rust first, then cargo. 

`podman run --rm rust:alpine3.17 rustc --version`  

`podman run --rm rust:alpine3.17 cargo --version`