# Darkness Check

A little CLI, built in Rust that lets you find out how many hours of darkness you're in for today. 

## Usage 


## Usage

There are a handful of ways to use the `darkness-check` app, balanced between looking up lat-long for locations yourself or getting an API key so the app can look it up for you.

### No Config: Bring Your Own Lat/Long Points 

```
darkness-check <LAT> <LONG>
```

NOTE: If you have provided an API key, this will fail and you should look up by city and country OR you can overwrite it by using the env variable

```
DARK_POINT="52.520008 13.404954" darkness-check 
```

### A Little Config

Put the latitude and longitude points in your `config.toml` or `config.json`. Then call it like this: 

```
darkness-check
```

### More Config, More Flexibility

If you want to look up locations by name, you will need to create a free account at [API Ninjas](https://api-ninjas.com/) and add the `key` field to `config.toml`.

```toml
# config.toml
key = "API_KEY_FROM_API_NINJAS"
```

Then look up any city and country.

```
darkness-check -- berlin germany 
```

Put cities with multiple words in quotes.

```
darkness-check -- "mexico city" mexico
```

The API key can also be provided as an environment variable.

```
DARK_KEY="jfhkaljkrhfjrhuljkhs" darkness-check -- berlin germany 
```

## Coming Soon

Add multiple points in the config.


## Installation 

Go to the releases page, which I will link when it's real.

**Built as an @axodotdev onboarding project.**
