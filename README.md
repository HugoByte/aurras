# Event Manager

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](http://www.apache.org/licenses/LICENSE-2.0)

### Introduction
Aurras is a middleware that acts as an event processor and a low code workflow orchestration platform. Aurras is being pitched as a next-generation system for enabling decentralized push notification. This middleware solution listens to events from blockchain applications and propagates them to a registered pool of MQTT brokers. The broader architecture consist of parachain from which the middleware listens for the events.

The Event Manager as a core component of aurras system is composed of multiple actions including using a database to store trigger URLs and their respective auth, and Kafka provider, consumer, and producer. Once the event manager receives an event from the event feed, this data is produced to a topic. The feed action in the manager lets the user hook into the system. That is, once an event is indexed to a particular topic, it can invoke a particular action. While creating the workflow, users can choose the event trigger as feed and provide necessary parameters from which chain it should be listening to.

### Architecture
![](https://storage.googleapis.com/hugobyte-2.appspot.com/aurras.png)

#### Components
1. **Event Sources**  
Event Feeds sources events from numerous sources to aurras system. Events can come from different blockchains or IoT devices or external services for which workflow need to executed. Event Manager registers Event feeds after which the aurras system will be listening for events from the registered sources. Below are the available event sources

    * [Substrate Event Feed](https://github.com/HugoByte/aurras-event-feed-substrate-js)

2. **Event Trigger Manager**  
Event trigger manager is composed of multiple actions including using a database to store trigger URLs and their respective auths, and Kafka provider, consumer and producer. Once the event manager receives an event from the event feed, this data is produced to a topic. The feed action in the trigger manager lets the user hook into the system. That is,  once an event is indexed to a particular topic, it can invoke a particular action. While creating the workflow, users can choose the event trigger as feed and provide necessary parameters from which chain it should be listening to and from which block it should start listening. Under the hood, a feed action is invoked with create lifecycle, which accepts the mandatory parameters the lifecycle, auth, trigger name, and other optional parameters of the event source. The feed action invokes the related actions of creating the entry in the database, adding to the Kafka consumer group, etc. The next component in the event trigger manager is a persistent connection to Kafka where it is used to produce and consume the stream of data. Once data is received in Kafka, the event trigger manager invokes the action to check the consumer groups for that particular topic and if found any, the trigger for the users under that particular group is invoked, which in turn invokes the workflow action.  

3. **Workflow Composer**  
Workflow composer consists of an async Rust library to compose multiple triggers, deployment configuration generator, and whisk deployment tool. For creating the workflow, the only input is the configuration file which is a YAML file. The workflow composition is laid out in the YAML which in turn takes care of the deployment and composing the triggers. Once a workflow is deployed to a namespace it creates a specific topic unique workflow id in Kafka. Workflow configuration comprises the input URL of workflow tasks, primarily GitHub repo, the sequence of processing tasks, and argument structure. Arguments must match the task input parameters.  

4. **Web API Gateway and Backend Service**   
This is the end user facing component to utilize the workflow. This component comprises of a backend application which is responsible for user registration, selecting the workflow, managing / creating workflow using friendly APIs, providing input parameters. API gateway / Machine gateway where the external world can connect to the Aurras system.

### Prerequisites

1. [Openwhisk](http://openwhisk.apache.org/)

### Components

#### Actions

* Event Receiver

This action receives events from the event feed and propagates to the system.

### Installation

Assuming basic dependency such as [git](https://git-scm.com/) and [yarn](https://yarnpkg.com/) already installed.

1. Clone the repository

```text
git clone https://github.com/HugoByte/aurras-event-manager.git
```

  2. Navigate to the cloned directory

```text
cd aurras-event-manager
```

  3. Deploy the actions using the deploy script. The script supports optional parameters which can be found [here](./docs/configuration.md).

```text
./deploy.sh
```
### Testing

Run Unit test suites

#### Actions

* Event Receiver

```text
cd actions/event-receiver
yarn test
```

### License

Licensed under [Apache-2.0](https://github.com/HugoByte/aurras-documentation/tree/f07f6727f0cb01cccf04f15ec446e2d310ca1cb9/components/event-feed/substrate-event-feed/LICENSE/README.md)
