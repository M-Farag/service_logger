### Service logger
Service logger is a simple service that logs messages to a file. It is a simple example of a service that can be used as a part of other services.

### Usage
To use the service you need to build the binary and run it.
The service accepts two arguments:
* filepath - to the file where the logs will be written
* message - the message that will be logged

Example:
```bash
./service-logger '/tmp/log.txt' 'Hello world!'
```
