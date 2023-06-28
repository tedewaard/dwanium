# dwanium

### Setup Environment
- Create an API key in Tanium
- Make sure you have a Dell Techdirect account and API Key so you can access the API
- Within Tanium under the Asset module -> Iventory management -> Sources create an 'Import API' source and name it "Dell Warranty End Date".
    - Enable Reconciliation and match on the serial number. Add the following mapping:
    ```
    {
        "keys": [
            "serial_number"
        ],
        "fieldMaps": [
        {
            "source": "serial",
            "destination": "serial_number"
        },
        {
            "source": "end_date",
            "destination": "dell_warranty_expiration"
        }
        ]
    }
    ```
- Also under Inventory Management create a new asset custom attribute called "Dell Warranty Expiration"

#### Create a .env file at the root of the project and add the following to include your Tanium API token, Dell ID and your Dell Secret key.  
```
`TOKEN=<Add Token Here>`  
`DELL_ID=<Add Dell ID Here>`  
`DELL_SECRET=<Add Dell Secret Key Here`
```
### Run the service
Run the following docker compose command which will build the docker image for dwanium
and spin up the container as well as the Postgres Database. 
```
docker compose up --build
```
