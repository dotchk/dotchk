#!/bin/bash
# Domain Monitoring Script - Check if a domain becomes available

DOMAIN="${1:-example.com}"
CHECK_INTERVAL="${2:-3600}"  # Default: 1 hour
NOTIFICATION_EMAIL="${3:-}"   # Optional email for notifications

echo "=== Domain Availability Monitor ==="
echo "Monitoring: $DOMAIN"
echo "Check interval: $CHECK_INTERVAL seconds"
echo "Press Ctrl+C to stop"
echo

# Create log file
LOG_FILE="domain-monitor-$(echo $DOMAIN | tr '.' '-').log"
echo "Log file: $LOG_FILE"
echo

# Function to send notification
send_notification() {
    local status=$1
    local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
    
    echo "[$timestamp] Domain $DOMAIN is $status" | tee -a "$LOG_FILE"
    
    # Send email notification if configured
    if [ -n "$NOTIFICATION_EMAIL" ] && command -v mail &> /dev/null; then
        echo "Domain $DOMAIN is now $status!" | mail -s "Domain Alert: $DOMAIN" "$NOTIFICATION_EMAIL"
    fi
    
    # macOS notification
    if command -v osascript &> /dev/null; then
        osascript -e "display notification \"Domain $DOMAIN is $status\" with title \"Domain Monitor\""
    fi
    
    # Linux desktop notification
    if command -v notify-send &> /dev/null; then
        notify-send "Domain Monitor" "Domain $DOMAIN is $status"
    fi
}

# Initial check
LAST_STATUS=""
while true; do
    # Check domain
    OUTPUT=$(dotchk check "$DOMAIN" 2>&1)
    
    # Parse status
    if echo "$OUTPUT" | grep -q "AVAILABLE"; then
        CURRENT_STATUS="AVAILABLE"
    elif echo "$OUTPUT" | grep -q "TAKEN"; then
        CURRENT_STATUS="TAKEN"
    else
        CURRENT_STATUS="ERROR"
    fi
    
    # Check if status changed
    if [ "$CURRENT_STATUS" != "$LAST_STATUS" ]; then
        send_notification "$CURRENT_STATUS"
        
        # If domain became available, also check related TLDs
        if [ "$CURRENT_STATUS" = "AVAILABLE" ]; then
            echo "Checking related TLDs..."
            BASE_DOMAIN=$(echo "$DOMAIN" | cut -d'.' -f1)
            dotchk tld "$BASE_DOMAIN" --popular --available-only | tee -a "$LOG_FILE"
        fi
    else
        echo "$(date '+%Y-%m-%d %H:%M:%S') - Status unchanged: $CURRENT_STATUS"
    fi
    
    LAST_STATUS="$CURRENT_STATUS"
    
    # Wait before next check
    sleep "$CHECK_INTERVAL"
done