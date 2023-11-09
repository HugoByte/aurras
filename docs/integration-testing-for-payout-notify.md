# Integration test for Polkadot Payout Notification

## Steps

1. Make sure while the aurras is up and running
2. Make sure [wsk cli](https://github.com/apache/openwhisk-cli) is added to the path
3. Install the [actions](../../../#installation)
4. Workflow generation with composer:
   Once your YAML configuration is ready, hand it over to the composer. This composer leverages the configuration to produce a WebAssembly (wasm) file. This wasm file encapsulates the workflow based on your specified criteria. 
   Generating a wasm file `ex: output.wasm` from the [PayoutNotification.yaml](../workflow/examples/PayoutNotification.yaml)
   
   ```
   cat examples/PayoutNotification.yaml | docker run -i hugobyte/workflow-composer:v0.2 generate > output.wasm
   ```

5. Deployment to openwhisk environment: 
   After creating a wasm file, copy the wasm file from `workflow` directory to the `runtime/openwhisk` directory. However wasm file should undergo compilation into executable format, tailord for openwhisk depolyment. can be done by using the below command
   
   ```
   zip -r - Cargo.toml src output.wasm | docker run -e RELEASE=true -i --rm hugobyte/openwhisk-runtime-rust:v0.3 -compile main > output.zip
   ```

6. Creating the action `payout_notify`
   
   ```
   wsk -i action create payout_notify  output.zip --docker hugobyte/openwhisk-runtime-rust:v0.3 --timeout 300000 --web true --param allowed_hosts "<allowed_hosts>"
   ```

7. Register event source using the below command with name as param eg: --name invoker.
   
   ```
   ./register_event_source_polkadot.sh --name invoker --openwhiskApiHost <API_HOST>
   ```

8. After registering the event, the system generates a topic. Keep a record of this token.

9.  User registration should be done by giving the parameters name, email and password. 
    
    ```
    curl -X POST "<API_HOST>/api/v1/web/guest/default/user-registration.json?blocking=true&result=true" -H 'Content-Type: application/json' -u "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123z03xZCLrMN6v2BKK1dXYFpX1PkCcOFqm12CdAsMgRU4VrNZ9lyGVCGUMDGIWP" -d '{"name": "name", "email": "email@gmail.com", "password":"password"}' -k
    ```

10. User can login with the credentials mentioned during the registration, this will create the JWT token
    
    ```
    curl -X POST "<API_HOST>/api/v1/web/guest/default/user-login.json?blocking=true&result=true" -H 'Content-Type: application/json' -u "23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123z03xZCLrMN6v2BKK1dXYFpX1PkCcOFqm12CdAsMgRU4VrNZ9lyGVCGUMDGIWP" -d '{"email": "email@gmail.com", "password": "password"}' -k
    ```

11. Perform the action invoke `user-login` by giving the below command, this will create a user token
    
    ```
    wsk -i action invoke user-login --param email email@gmail.com --param password password -b -r
    ```

12. Navigate to examples/susbtrate-push-notification
13. Add [API configuration](../examples/substrate-push-notification/#api-configuration) and [Firebase Push Notification Configuration](../examples/substrate-push-notification/#push-notification-configuration)
14. Install Node Dependencies using `yarn install`
15. Start susbtrate-push-notification using `npx yarn start`
> For Brave brower enable `Use Google services for push messaging` using brave://settings/privacy
16. Select the account for which polkadot notification to be received
17. Click Register Balance Notification button
18. Select the event source, here in this case `invoker`. Copy the Push notification token(user device token).
19. Open postman and try to execute it there by pasting the copied user-device-token, adding owner key, adding topic and adding   auth-token.
    ![Allow Push Notification](../examples/substrate-push-notification/images/Screen-6.png)
    

20. Now the next step is to run [aurras-event-feed-substrate](https://github.com/HugoByte/aurras-event-feed-substrate-js).
21. Here, change the env variables according to your usage. 
22. Install the Node Dependencies using `yarn install` 
23. Start aurras-event-feed-substrate-js using `npx yarn serve`
24. Here 1 era = 6 hours. After 1 era we will get a notification saying, "era payout completed".
    
### Expected Output
- Payout notification
  ![Allow Push Notification](../examples/substrate-push-notification/images/Screen-7.png)

- Checking the verification process in chrome
  ![Allow Push Notification](../examples/substrate-push-notification/images/Screen-8.png)
