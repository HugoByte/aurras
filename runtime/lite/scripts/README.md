# Setup the SSB-Network For Test
This is a sample network for ssb for testing purpose. 

## Pre-requisite
- docker compose

## Steps for setup
- Start the Pub,consumer, and Producers

        ./ssb-up start 
- Create invite for pub

        ./ssb-up create-invite`
- Accept the invite

        ./ssb-up accept-invite`

- For Stop and clean

        ./ssb-up stop

- Start a specifi service like consumer or producer

        ./ssb-up start-service consumer
        or 
        ./ssb-up start-service producer

>[!NOTE]
> If you need more than one consumer or producer, you may need to change the port number in docker compose file