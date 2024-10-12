# Zix - API Schema Manager

**Zix** is a command-line tool designed for managing API schemas. It enables users to create, list, and generate documentation for APIs in a structured and efficient way. Utilizing the `clap` library for command-line argument parsing and `serde` for serialization, Zix simplifies the process of defining and documenting APIs.

## Features

- **Create API Schema**: Easily create a new API schema by specifying its name, version, and endpoints. Each endpoint can define its path, method, request format, and response format.
- **List Schemas**: View all created API schemas in your current directory, helping you keep track of your API documentation.
- **Generate Documentation**: Automatically generate Markdown documentation for any existing API schema, making it simple to share API details with your team or stakeholders.

## Usage

To get started with Zix, use the following commands:

- **Create a New API Schema**:
  ```
  zix create --name <schema_name> --version <schema_version> [--endpoints <path,method,request_format,response_format>...]
  ```

- **List All API Schemas**:
  ```
  zix list
  ```

- **Generate Documentation**:
  ```
  zix generate-docs --name <schema_name>
  ```

## Example

To create a new API schema with an endpoint:
```
zix create --name UserAPI --version 1.0 --endpoints "/users,GET,,application/json"
```

This command creates a JSON file named `UserAPI.json` containing the defined schema.

## File Structure

The project uses JSON files to store API schemas and generates Markdown files for documentation. Each schema file contains the schema's name, version, and a list of endpoints.

## Dependencies

- `clap`: For command-line argument parsing.
- `serde`: For serialization and deserialization of JSON data.

## Conclusion

Zix is an efficient tool for developers looking to manage their API schemas effortlessly. With its easy-to-use interface and powerful features, Zix enhances the API development workflow.