# Readme
This command line tool is built to provide a missing step from the BDK language bindings where one is required to build the funding transaction given a output script provided by ldk-java in raw bytes (`byte[]` in Java, `ByteArray` in Kotlin).

Given the workflow outlined here, we should be able to ensure this functionality is available to users in the bdk-ffi language bindings, which is why I want to make sure it's done correctly and in the best way (I don't think it currently is, which is why the tool needs work).

To create the funding transaction, use the tool like so:
```shell
./txcreator --descriptor <yourdescriptor> --channel-value-satoshis <number in satoshis> --network testnet --output-script <the output script given by LDK in hex format>
```
