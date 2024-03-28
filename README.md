<p align="center">
  <picture>
      <img src="./logo.png" height="75">
    </picture>
    <h1 align="center">valid-env</h1>
</p>

<p align="center">
     <img src="https://github.com/ieedan/valid-env/actions/workflows/continuous-integration.yml/badge.svg?branch=main">
</p>

A decorator enhanced **.env** syntax for validating your environment variables.

> Note: This is not stable for production use

## Examples

Validate a list of phone numbers:
```ruby
@matches("[0-9]{3}-[0-9]{3}-[0-9]{4}")
CALL_LIST=["333-333-3333", "111-111-1111"]
```

Use a different API for development or production:
```ruby
@dev
@min(9)
API_URL="http://localhost:7301"

@dev
@min(10)
@startsWith("https://")
@doesNotMatch("localhost")
API_URL="https://my-domain.dev"
```

## Getting Started

1. Download the source code and compile the binary. 

2. Set your system path variable to point to the binary.

3. Check you have it correctly configured with `vnv --version`

4. Download the VS Code extension for syntax highlighting [here](https://marketplace.visualstudio.com/items?itemName=AidanBleser.valid-env).

5. Run `vnv init` to setup the config file and optional template files.

6. Run `vnv check` to validate your environment variables

7. Run `vnv build` to build your `.vnv` file into a `.env` file

## Variable Types
Currently valid-env supports 4 different types of environment variables.

- String
- Number
- String[]
- Number[]

For simplicity to the end user all numbers are 64 bit floating point integers.

## Decorator Enhanced
valid-env extends the .env syntax with decorators that allow you to validate and scope your environment variables.

### Decorators
- [@public](#public)
- [@private](#private)
- [@dev](#dev)
- [@prod](#prod)
- [@min](#min)
- [@max](#max)
- [@startsWith](#startsWith)
- [@endsWith](#endsWith)
- [@matches](#matches)
- [@doesNotMatch](#doesNotMatch)

### @public
Changes the scope of the environment variable to **public**;

Usage:
```ruby
@public
PORT=3000
```

#### Allowed Variable Types
- String
- Number
- String[]
- Number[]

### @private
This is not necessary as all variables by default are **private** but may be useful to annotate the importance for a variables privacy. 

Usage:
```ruby
@private
SECRET="this is a super private secret"
```

#### Allowed Variable Types
- String
- Number
- String[]
- Number[]

### @dev
Changes the environment of the environment variable to **dev**;

Usage:
```ruby
@dev
PORT=3000
```

#### Allowed Variable Types
- String
- Number
- String[]
- Number[]

### @prod
Changes the environment of the environment variable to **prod**;

Usage:
```ruby
@prod
PORT=3000
```

#### Allowed Variable Types
- String
- Number
- String[]
- Number[]

### @min
Allows you to validate the minimum length or size of a variable. For number types it will validate the size of the number. For string types it will validate the length.

Usage:
```ruby
@min(1000)
POLLING_INTERVAL=5000
@min(10)
DOMAIN="https://google.com"
@min(1024)
MICROSERVICE_PORTS=[8080, 8081, 8082]
@min(5)
ADMIN_EMAILS=["aidan@gmail.com", "john@yahoo.com"]
```

#### Allowed Variable Types
- String
- Number
- String[]
- Number[]

### @max
Allows you to validate the maximum length or size of a variable. For number types it will validate the size of the number. For string types it will validate the length.

Usage:
```ruby
@max(45000)
POLLING_INTERVAL=5000
@max(25)
APP_NAME="super-powered-app"
@max(49151)
MICROSERVICE_PORTS=[8080, 8081, 8082]
@max(254)
ADMIN_EMAILS=["aidan@gmail.com", "john@yahoo.com"]
```

#### Allowed Variable Types
- String
- Number
- String[]
- Number[]

### @startsWith
Allows you to validate the start of a string variable.

Usage:
```ruby
@startsWith("https://")
DOMAIN="https://google.com"
@startsWith("https://")
ALLOWED_ORIGINS=["https://google.com", "https://github.com"]
```

#### Allowed Variable Types
- String
- String[]

### @endsWith
Allows you to validate the end of a string variable.

Usage:
```ruby
@endsWith("@gmail.com")
EMAIL="john.doe@gmail.com"
@endsWith("@gmail.com")
ALLOWED_ORIGINS=["john.doe@gmail.com", "jane.doe@gmail.com"]
```

#### Allowed Variable Types
- String
- String[]

### @matches
Enables regex based pattern matching to validate string variables. Will return an error when the pattern doesn't match. 

Usage:
```ruby
# Digits only regex
@matches("^\d+$")
PHONE_NUMBER="4427211223"
@matches("^\d+$")
PHONE_NUMBERS=["4427211223", "4427511227", "4428211213"]
```

#### Allowed Variable Types
- String
- String[]

### @doesNotMatch
Enables regex based pattern matching to validate string variables. Will return an error when the pattern matches.

Usage:
```ruby
# Special characters regex
@doesNotMatch("[^\w.]")
SUPER_USER="admin"
@doesNotMatch("[^\w.]")
ADMIN_USERNAMES=["johnothy", "jimnothy"]
```

#### Allowed Variable Types
- String
- String[]

## Public and Private
Some environment variable handlers allow you to scope your variables to be public or private. (For example [SvelteKit](https://learn.svelte.dev/tutorial/env-static-private)). This allows you to separate privileges to use environment variables between server and client code. By default all variables are scoped as **private** but can be marked public using the `@public` decorator.

> Note: While the `@private` decorator is valid syntax and listed as a decorator it does not change the scope of the variable. However it can be useful for annotating something that should be treated as sensitive and should not be changed to public.

## Environments 
Sometimes you want to use different values for your variables for different environments or even different variables entirely. This is made possible with the [@dev](#dev) and [@prod](#prod) decorators.

Here are a few examples:

Different Value for same variable
```ruby
@dev
PORT=3000
@prod
PORT=8080
```

Omit a variable from prod
```ruby
@dev
KEY="..."
```

Omit a variable from dev
```ruby
@prod
KEY="..."
```

> Keep in mind any keys not marked with `@prod` or `@dev` will be included in all environments.

### How to specify the environment
Pass the `--prod` or `--dev` flag to the check/build command. By default the environment is set to `dev` so theres no need to supply the `--dev` flag.

.vnv file:
```ruby
@dev
PORT=3000
@prod
PORT=8080
```

Output:
```bash
C:\Users\aidan\project> vnv build --dev
Checking '.vnv'...
PORT ✔️
PORT ⏭️
Completed in 1.24ms
Completed build wrote output to .env.
```

Generated .env file: 
```ruby
# This file was generated from '.vnv' by vnv.

# @dev
PORT=3000
```