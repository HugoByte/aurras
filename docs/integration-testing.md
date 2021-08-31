# Integration test

## Steps
1. While the Aurras system is up and running
2. Make sure [wsk cli](https://github.com/apache/openwhisk-cli) is added to the path
3. Install the [actions](../../../#installation)
4. Register event source using the below command with name as param eg: --name polkadot-balance if connecting to polkadot

```
./register_event_source.sh --name polkadot-balance
```
5. Get the generated uuid and add as an environment variable to the substrate [event feed](../../../../?q=aurras-event-feed)
6. Connect event feed with a Substrate based chain
7. Using [examples/susbtrate-push-notification](../examples/susbtrate-push-notification) Register for balance notification
8. Perform an amount transfer transaction to the registered wallet.
9. Verify the push notification received.
