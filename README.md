# dwanium

### Setup Environment
Create a .env file at the root of the project and add the following to include your Tanium API token, Dell ID and your Dell Secret key.  
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
