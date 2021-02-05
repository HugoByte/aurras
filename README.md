## Event Receiver

### Description
This action receives the events from the event feed services

### Installation
1. Install the action

```
    wsk action create EventReceiver actions/event-receiver/index.js
```
2. Create Trigger

```
    wsk trigger create ReceiveEvent
```
3. Create Rule to link between the Action and trigger events
```
 wsk rule create EventReceiver_ReceiveEvent_Rule ReceiveEvent EventReceiver
```