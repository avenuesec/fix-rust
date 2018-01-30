# fix-rust
FIX (Financial Information Exchange) client in Rust


### Todo items

* Currently only works as client (initator), some adjustments are needed to make it work as server (acceptor)

* During message sync, any message that is not filling the gap should go to a queue instead and only come out of that once the gap is filled

* It does not ask the handler if a message should be resent, this may be an issue since the handler may prevent staled messages from being resend (like new_order_single)

* Instead of mio, using proper future-rs/tokio would have been better, and would allow the message store to be async (allowing using an impl persisting to DynamoDB for example)

