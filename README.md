# Pig Farm Management System

This project is a Pig Farm Management System built with Rust, leveraging the power of the Internet Computer to manage various entities and operations related to pig farming. The system includes functionalities for managing pigs, feeds, health records, inventory, invoices, notifications, and breeding records.

## Key Features

1. **Pig Management**

   - **Create Pig:** Allows the creation of new pig records with attributes such as id, name, breed, birth_date, weight, health_status, and created_at.
   - **Get Pig:** Retrieves the details of a specific pig by its ID.
   - **Update Pig:** Updates the existing pig record with new information.
   - **Get All Pigs:** Retrieves all registered pig records.

2. **Feed Management**

   - **Create Feed Record:** Allows a user to register a new feed record for a pig.
   - **Get Feed Record:** Retrieves the details of a specific feed record by its ID.
   - **Get Feed Records by Pig ID:** Lists all feed records for a specific pig.

3. **Health Record Management**

   - **Create Health Record:** Allows the creation of new health records for pigs.
   - **Get Health Record:** Retrieves the details of a specific health record by its ID.
   - **Get Health Records by Pig ID:** Lists all health records for a specific pig.

4. **Inventory Management**

   - **Create Inventory Item:** Allows a user to add a new item to the inventory.
   - **Get Inventory Item:** Retrieves the details of a specific inventory item by its ID.
   - **Get All Inventory Items:** Lists all items in the inventory.

5. **Invoice Management**

   - **Create Invoice:** Allows the creation of new invoices.
   - **Get Invoice:** Retrieves the details of a specific invoice by its ID.
   - **Get Invoices by Customer ID:** Lists all invoices associated with a specific customer.

6. **Notification Management**

   - **Create Notification:** Allows the creation of new notifications for users.
   - **Get Notification:** Retrieves the details of a specific notification by its ID.
   - **Get Notifications by Customer ID:** Lists all notifications associated with a specific customer.

7. **Breeding Record Management**

   - **Create Breeding Record:** Allows the creation of new breeding records for pigs.
   - **Get Breeding Record:** Retrieves the details of a specific breeding record by its ID.
   - **Get Breeding Records by Pig ID:** Lists all breeding records for a specific pig.

8. **Error Handling**
   - **Not Found:** Returns an error if a requested resource (pig, feed record, health record, inventory item, invoice, notification, breeding record) is not found.
   - **Invalid Input:** Handles errors related to invalid input fields or missing required fields.

## Requirements

- rustc 1.64 or higher

```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```

- rust wasm32-unknown-unknown target

```bash
$ rustup target add wasm32-unknown-unknown
```

- candid-extractor

```bash
$ cargo install candid-extractor
```

- install `dfx`

```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:

```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:

```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:

```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:

```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```
