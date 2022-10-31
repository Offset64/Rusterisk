# Rusterisk 
An Asterisk Manager Interface implementation in Rust

# TODO:
* Networking - Setup multithreading.
    * We want to be able to send messages to the server and continue processing without blocking.
    * When the server sends us an event or response, we want to continue to read from the network without blocking.
        * To test this, lets do a non-multithreaded-implementation which waits briefly then prints. This should look like "msg -> wait -> print -> msg -> wait -> print"
        * After this, we implement it with multithreading. This should like closer to "msg ->  msg -> print -> msg -> print -> print"