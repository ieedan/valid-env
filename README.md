@ -1,30 +0,0 @@
# valid-env
A .env file parser that supports decorators to allow for data validation and other functions.

> **Note:** This is just for fun obviously not for use in real projects.

### Decorator enhanced
Supports decorators like 
- `@public` Will put variables under the public object
- `@private` Will put variables under the private object (default)
- `@max()` You can provide this decorator with a number to validate the length of a string or size of a number
- `@min()` You can provide this decorator with a number to validate the length of a string or size of a number

Examples:

```ruby
# goes to private
SOMETHING="nothing"
# goes to public
@public
NOTHING="something"
```

```ruby
# valid
@min(5)
SOMETHING=10
# invalid
@min(5)
SOMETHING=4
```