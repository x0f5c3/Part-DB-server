# Part-DB

Part-DB is an Open-Source inventory management system for your electronic components. It is installed on a web server and can be accessed with any browser without the need to install additional software.

## About This Version

This version of Part-DB has been fully rewritten with a modern technology stack:

- **Backend**: Rust with Axum framework and SQLX for database access
- **Frontend**: React with Next.js 14, Tailwind CSS, and shadcn/ui components
- **Database**: PostgreSQL, MySQL, or SQLite support

The new architecture provides:

- Improved performance and type safety
- Better developer experience
- Modern, responsive UI
- 100% API compatibility with the original PHP application

## Features

- **Inventory Management**: Track your electronic parts with categories, footprints, manufacturers, storage locations, and price information
- **Multi-language Support**: Currently supports German, English, Russian, Japanese, French, Czech, Danish, and Chinese
- **Barcode/Label System**: Generate barcodes and labels for parts and storage locations, scan via webcam
- **User Management**: Fine-grained permissions, two-factor authentication, and optional SSO via SAML
- **Project Management**: Create projects with bills of material (BOM) and track build capabilities
- **Event Logging**: Track changes to your inventory and revert to older versions
- **Responsive Design**: Works on desktop, tablet, and mobile devices
- **API Access**: Full REST API for integration with other applications
- **KiCad Integration**: Use Part-DB as a central datasource in KiCad

## Demo

You can try Part-DB without installing it using our demo instance:

- [Demo (English)](https://demo.part-db.de/)
- [Demo (German)](https://demo.part-db.de/de/)

Login with username: **user** and password: **user**

## License

Part-DB is licensed under the GNU Affero General Public License v3.0 (or any later version). This means you can use Part-DB for any purpose (including commercial use) as long as you publish the source code for any modifications under the AGPL.

## Getting Help

- **Documentation**: You're reading it!
- **GitHub Issues**: [Report bugs or request features](https://github.com/x0f5c3/Part-DB-server/issues)
- **Contributing**: See our [Contributing Guide](./development/contributing.md)
