# Better Call Put
Application for trading on the MOEX via the T-Invest API.

## Prerequisites
* Copy `config.example.yaml` in `config.yaml` and add your tokens:
```bash
cp config.example.yaml config.yaml
```

## Docker Commands

### First Run
```bash
# Create .env file from example
./deploy.sh env

# Deploy the application
./deploy.sh
# or
./deploy.sh deploy
```

### Application Management
```bash
# Stop the application
./deploy.sh stop

# Start the application
./deploy.sh start

# Restart the application
./deploy.sh restart

# Stop and remove containers
./deploy.sh down

# Check application status
./deploy.sh status
```

### Logging
```bash
# Follow logs in real-time
./deploy.sh logs
```

### Help
```bash
# Show all available commands
./deploy.sh help
```

## Configuration

### Environment Variables
The application uses the following environment variables (defined in `.env`):

#### Application Settings
* `ENVIRONMENT` - Application environment (default: production)
* `VERSION` - Application version (default: 1.0)
* `RUST_LOG` - Logging levels (default: better_call_put=error,warn,info,debug,hyper=error,warn,info)

#### Resource Limits
* `CPU_LIMIT` - CPU limit (default: 1)
* `MEMORY_LIMIT` - Memory limit (default: 1G)
* `CPU_RESERVATION` - CPU reservation (default: 0.5)
* `MEMORY_RESERVATION` - Memory reservation (default: 512M)

#### Healthcheck Settings
* `HEALTHCHECK_INTERVAL` - Health check interval (default: 30s)
* `HEALTHCHECK_TIMEOUT` - Health check timeout (default: 10s)
* `HEALTHCHECK_RETRIES` - Health check retries (default: 3)

#### Logging Settings
* `LOG_MAX_SIZE` - Maximum log file size (default: 10m)
* `LOG_MAX_FILES` - Maximum number of log files (default: 3)

### Application Configuration
The application uses the following settings in `config.yaml`:

#### API Tokens
* `t_token` - Your Tinkoff Invest API token
* `telegram_token` - Your Telegram Bot API token

#### General Settings
* `scan_interval_seconds` - Interval between market scans (default: 300)

#### Filter Settings
* `filter.class_code` - Market class code (e.g., "TQBR" for shares)
* `filter.instrument_type` - Type of instruments to scan (e.g., "INSTRUMENT_TYPE_SHARE")

#### Strategy Settings
* `strategy.short_ema_length` - Length of short EMA (default: 8)
* `strategy.long_ema_length` - Length of long EMA (default: 21)
* `strategy.interval` - Time interval for indicators (e.g., "INDICATOR_INTERVAL_4_HOUR")
* `strategy.hysteresis_percentage` - Hysteresis threshold (default: 0.1)
* `strategy.hysteresis_periods` - Number of periods for hysteresis (default: 1)

#### Asset Settings
* `assets.instrument_type` - Type of instruments to trade (e.g., "INSTRUMENT_TYPE_SHARE")
* `assets.instrument_status` - Status of instruments (e.g., "INSTRUMENT_STATUS_BASE")

Example configuration:
```yaml
t_token: "your_tinkoff_token"
telegram_token: "your_telegram_bot_token"
scan_interval_seconds: 300

filter:
  class_code: "TQBR"
  instrument_type: "INSTRUMENT_TYPE_SHARE"

strategy:
  short_ema_length: 8
  long_ema_length: 21
  interval: "INDICATOR_INTERVAL_4_HOUR"
  hysteresis_percentage: 0.1
  hysteresis_periods: 1

assets:
  instrument_type: "INSTRUMENT_TYPE_SHARE"
  instrument_status: "INSTRUMENT_STATUS_BASE"
```