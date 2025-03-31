# Better Call Put
Application for trading on the MOEX via the T-Invest API.
## Prerequisites
* Create a `config.yaml` file
```
touch "config.yaml"
```
* Add in your `config.yaml` file
```yaml
api_token: <your_token>
telegram_token: <your_token>
class_code: TQBR
instrument_type: INSTRUMENT_TYPE_SHARE
scan_interval_seconds: 300
strategy:
  short_ema_length: 8
  long_ema_length: 21
  interval: INDICATOR_INTERVAL_4_HOUR
```