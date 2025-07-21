# TLD Update Script

This script helps you keep the dotchk's TLD list up-to-date with the latest IANA root zone data.

## Script

### `update_tlds.py`
Python script that:
- Downloads the latest root zone file from IANA
- Parses all TLDs and their authoritative nameservers  
- Generates an updated `src/tld.rs` file
- Assigns appropriate timeouts based on TLD type and geographic location
- Creates a backup of the existing file before overwriting

## Usage

### Using Make (Recommended)
```bash
make update-tlds
```

### Direct Python
```bash
python3 scripts/update_tlds.py
```

### Make Executable and Run
```bash
chmod +x scripts/update_tlds.py
./scripts/update_tlds.py
```

## When to Update

Run these scripts when:
- IANA releases new TLDs (check https://www.iana.org/domains/root/db)
- Nameservers for existing TLDs change
- You want to ensure you have the latest data
- Building the project for production deployment

## How It Works

1. **Downloads Root Zone**: Fetches the official root zone file from https://www.internic.net/domain/root.zone
2. **Parses NS Records**: Extracts all TLD nameserver records
3. **Categorizes TLDs**: Groups TLDs by type for timeout optimization:
   - Common gTLDs: 1000ms timeout
   - European/North American ccTLDs: 1000ms timeout
   - Asia-Pacific ccTLDs: 1500ms timeout
   - Other regions: 2000ms timeout
4. **Generates Rust Code**: Creates a properly formatted `phf_map!` with all TLDs
5. **Backs Up Existing File**: Saves the current tld.rs as tld.rs.backup
6. **Writes New File**: Replaces src/tld.rs with updated data

## Example Output

```
TLD Update Script - 2024-01-15 10:30:45
==================================================
Downloading root zone from https://www.internic.net/domain/root.zone...
Downloaded 2451632 bytes
Parsing root zone file...
Generating Rust code for 1440 TLDs...
Writing to ../src/tld.rs...
Backed up existing file to ../src/tld.rs.backup
Successfully wrote 237854 bytes to ../src/tld.rs

=== TLD Statistics ===
Total TLDs: 1440

TLDs by nameserver count:
  13 nameservers: 3 TLDs
  12 nameservers: 1 TLDs
  10 nameservers: 5 TLDs
  9 nameservers: 8 TLDs
  ...

TLD categories:
  Common gTLDs: 45
  EU/NA ccTLDs: 35
  Asia-Pacific ccTLDs: 32
  Other: 1328

TLD update completed successfully!
```

## Requirements

- Python 3.6+
- Internet connection to download root zone
- Write permission to src/tld.rs

## Notes

- The script automatically backs up the existing tld.rs file
- If the update fails, the backup is restored
- The generated code includes proper formatting and imports
- All nameservers are included for each TLD
- The output is deterministic (sorted) for clean git diffs