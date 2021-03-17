# Integration test

## Steps
1. While the Aurras system is up and running
2. Make sure [wsk cli](https://github.com/apache/openwhisk-cli) is added to the path
3. Install the [event manager action](../#installation)
4. Perform a transaction to emit an event from the chain.
5. Navigate to [aurras-event-manager](../) source directory.
6. Use wsk cli to list activation ids.

```
wsk -i --apihost https://localhost:31001 --auth 23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP activation list
```
7. To get details of an activation, where ``<Activation ID>`` is a unique id of the activation which is executed as a result for an event emitted from the chain.

```
wsk -i --apihost https://localhost:31001 --auth 23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP activation get <Activation ID>
```

8. Verify if the activation result contains event data received from the chain matches **response.result.event.data**
