Features

    Concurrent Scanning: Checks the status of all your saved services at the exact same time instead of waiting for them one-by-one.

    Automatic Timeouts: Smart connection management that drops dead or hanging network targets automatically so your application never freezes.

    Permanent Storage: Saves your services locally in a clean text file inside your home directory so your data is never lost when you close the app.

    Resource Efficient: Uses minimal system resources and memory by leveraging Rust's fast compilation and execution model.

Architecture Overview

Traditional monitoring tools check hosts sequentially, meaning one dead or lagging IP will cause the entire application to stutter or hang.

Simple Service uses a cooperative scheduling model. The moment a connection request is sent out to a server, the application shifts its focus to fire off the next check immediately. This ensures that fast responders report back instantly, while slower, offline hosts are handled gracefully in the background.
How It Works

The application operates through a straightforward command pipeline split into two main sections:
1. Registry Management (CRUD)

    Add: Input an IP address, connection port, and a custom nickname to register a new service.

    List: View all currently monitored endpoints, their saved network configurations, and assigned names.

    Update: Modify any existing parameter (such as a changing IP address or port number) without needing to delete the entry.

    Delete: Permanently remove an old or unneeded service profile from your local database.

2. Network Testing Engine

    Single Test: Targets a specific service by name to determine if it is responsive.

    Test All: Triggers the core asynchronous engine to ping every single registered device at the exact same moment, printing a clean list of live and dead connections as they report back.

Data Storage

All your configuration data is stored locally on your machine inside your user profile folder in a file named service_registry.json. This keeps your network configurations completely private, accessible offline, and easy to back up.