# Configuration

Configuration values below are passed as parameters to the script.

### Openwhisk API Host

Openwhisk API Endpoint to where the actions to be deployed

| Parameter | Default Value |
| :--- | :--- |
| --openwhiskApiHost | `https://localhost:31001` |

#### usage

```text
./deploy.sh --openwhiskApiHost https://localhost:31001
```

### 

### Openwhisk API Key

Openwhisk authentication key.

| Parameter | Default Value |
| :--- | :--- |
| --openwhiskApiKey | 23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP |

#### usage

```text
./deploy.sh --openwhiskApiKey 23bc46b1-71f6-4ed5-8c54-816aa4f8c502:123zO3xZCLrMN6v2BKK1dXYFpXlPkccOFqm12CdAsMgRU4VrNZ9lyGVCGuMDGIwP
```

### 

### Openwhisk Namespace

Organization space where the actions to be deployed

| Parameter | Default Value |
| :--- | :--- |
| --openwhiskNamespace | guest |

#### usage

```text
./deploy.sh --openwhiskNamespace guest
```

