#!/usr/bin/env python3
"""
Update TLD list from IANA root zone database.

This script downloads the latest root zone file from IANA and generates
an updated src/tld.rs file with all current TLDs and their nameservers.

Usage:
    python scripts/update_tlds.py
    
Or make it executable:
    chmod +x scripts/update_tlds.py
    ./scripts/update_tlds.py
"""

import urllib.request
import sys
from collections import defaultdict
from datetime import datetime
import os

# URL for the IANA root zone file
ROOT_ZONE_URL = "https://www.internic.net/domain/root.zone"

# Output file path (relative to script location)
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
OUTPUT_FILE = os.path.join(SCRIPT_DIR, "..", "src", "tld.rs")

# TLD categories for timeout assignment
COMMON_GTLDS = {
    'com', 'net', 'org', 'info', 'biz', 'name', 'pro', 'aero', 'coop', 
    'museum', 'int', 'edu', 'gov', 'mil', 'app', 'dev', 'io', 'co', 'me',
    'tv', 'xyz', 'top', 'site', 'online', 'shop', 'store', 'blog', 'cloud',
    'tech', 'website', 'space', 'live', 'life', 'world', 'email', 'group',
    'ltd', 'digital', 'one', 'work', 'business', 'company', 'network', 'agency'
}

EU_NA_CCTLDS = {
    'us', 'ca', 'uk', 'de', 'fr', 'es', 'it', 'nl', 'be', 'ch', 'at', 'se',
    'no', 'dk', 'fi', 'ie', 'pt', 'gr', 'pl', 'cz', 'hu', 'ro', 'bg', 'hr',
    'si', 'sk', 'lt', 'lv', 'ee', 'is', 'lu', 'mt', 'cy'
}

ASIA_PACIFIC_CCTLDS = {
    'au', 'nz', 'jp', 'cn', 'kr', 'tw', 'hk', 'sg', 'my', 'th', 'id', 'ph',
    'vn', 'in', 'pk', 'bd', 'lk', 'np', 'mm', 'kh', 'la', 'bn', 'mv', 'mn',
    'kz', 'uz', 'tm', 'kg', 'tj', 'af', 'bt'
}

def download_root_zone():
    """Download the root zone file from IANA."""
    print(f"Downloading root zone from {ROOT_ZONE_URL}...")
    try:
        with urllib.request.urlopen(ROOT_ZONE_URL) as response:
            return response.read().decode('utf-8')
    except Exception as e:
        print(f"Error downloading root zone: {e}")
        sys.exit(1)

def parse_root_zone(content):
    """Parse the root zone file and extract TLD nameservers."""
    tlds = defaultdict(set)
    
    print("Parsing root zone file...")
    lines = content.split('\n')
    
    for line in lines:
        line = line.strip()
        if not line or line.startswith(';'):
            continue
            
        parts = line.split()
        if len(parts) >= 5 and parts[3] == 'NS':
            domain = parts[0].rstrip('.')
            nameserver = parts[4].rstrip('.')
            
            # Only process TLDs (no dots in the name)
            if '.' not in domain and domain:
                # Skip special domains
                if domain in ['ARPA', 'ROOT-SERVERS', 'LOCALHOST']:
                    continue
                tlds[domain.lower()].add(nameserver.lower())
    
    return dict(tlds)

def determine_timeout(tld):
    """Determine appropriate timeout for a TLD based on its type and location."""
    tld = tld.lower()
    
    # Common gTLDs and EU/NA ccTLDs get 1000ms
    if tld in COMMON_GTLDS or tld in EU_NA_CCTLDS:
        return 1000
    # Asia-Pacific ccTLDs get 1500ms
    elif tld in ASIA_PACIFIC_CCTLDS:
        return 1500
    # All others (Africa, South America, Middle East, etc.) get 2000ms
    else:
        return 2000

def generate_rust_code(tlds):
    """Generate Rust code for the TLD module."""
    print(f"Generating Rust code for {len(tlds)} TLDs...")
    
    # Sort TLDs for consistent output
    sorted_tlds = sorted(tlds.items())
    
    # Generate the Rust code
    rust_code = '''use phf::phf_map;

#[derive(Debug, Clone)]
pub struct TldInfo {
    pub servers: &'static [&'static str],
    pub timeout_ms: u64,
}

pub static TLD_SERVERS: phf::Map<&'static str, TldInfo> = phf_map! {
'''
    
    for tld, nameservers in sorted_tlds:
        # Sort nameservers for consistent output
        sorted_ns = sorted(nameservers)
        
        # Format nameserver array
        ns_array = ', '.join(f'"{ns}"' for ns in sorted_ns)
        
        # Determine timeout
        timeout = determine_timeout(tld)
        
        # Add entry
        rust_code += f'''    "{tld}" => TldInfo {{
        servers: &[{ns_array}],
        timeout_ms: {timeout},
    }},
'''
    
    rust_code += '''};

pub fn get_tld_info(domain: &str) -> Option<&'static TldInfo> {
    let parts: Vec<&str> = domain.split('.').collect();
    if parts.len() < 2 {
        return None;
    }
    
    let tld = parts[parts.len() - 1].to_lowercase();
    TLD_SERVERS.get(tld.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_get_tld_info() {
        assert!(get_tld_info("example.com").is_some());
        assert!(get_tld_info("test.xyz").is_some());
        assert!(get_tld_info("invalid.unknown").is_none());
        assert!(get_tld_info("invalid").is_none());
    }
    
    #[test]
    fn test_common_tlds() {
        assert!(TLD_SERVERS.get("com").is_some());
        assert!(TLD_SERVERS.get("org").is_some());
        assert!(TLD_SERVERS.get("net").is_some());
        assert!(TLD_SERVERS.get("io").is_some());
        assert!(TLD_SERVERS.get("dev").is_some());
        assert!(TLD_SERVERS.get("app").is_some());
    }
}
'''
    
    return rust_code

def write_rust_file(content):
    """Write the generated Rust code to file."""
    print(f"Writing to {OUTPUT_FILE}...")
    
    # Create backup of existing file
    if os.path.exists(OUTPUT_FILE):
        backup_file = OUTPUT_FILE + '.backup'
        os.rename(OUTPUT_FILE, backup_file)
        print(f"Backed up existing file to {backup_file}")
    
    try:
        with open(OUTPUT_FILE, 'w') as f:
            f.write(content)
        print(f"Successfully wrote {len(content)} bytes to {OUTPUT_FILE}")
    except Exception as e:
        print(f"Error writing file: {e}")
        # Restore backup if write failed
        if os.path.exists(backup_file):
            os.rename(backup_file, OUTPUT_FILE)
            print("Restored backup file")
        sys.exit(1)

def print_statistics(tlds):
    """Print statistics about the TLDs."""
    print("\n=== TLD Statistics ===")
    print(f"Total TLDs: {len(tlds)}")
    
    # Count by number of nameservers
    ns_counts = defaultdict(int)
    for tld, nameservers in tlds.items():
        ns_counts[len(nameservers)] += 1
    
    print("\nTLDs by nameserver count:")
    for count in sorted(ns_counts.keys(), reverse=True):
        print(f"  {count} nameservers: {ns_counts[count]} TLDs")
    
    # Show some examples
    print("\nExample TLDs:")
    examples = list(tlds.keys())[:10]
    for tld in examples:
        print(f"  .{tld}: {len(tlds[tld])} nameservers")
    
    # Category counts
    gtlds = sum(1 for tld in tlds if tld in COMMON_GTLDS)
    eu_na = sum(1 for tld in tlds if tld in EU_NA_CCTLDS)
    asia_pac = sum(1 for tld in tlds if tld in ASIA_PACIFIC_CCTLDS)
    other = len(tlds) - gtlds - eu_na - asia_pac
    
    print(f"\nTLD categories:")
    print(f"  Common gTLDs: {gtlds}")
    print(f"  EU/NA ccTLDs: {eu_na}")
    print(f"  Asia-Pacific ccTLDs: {asia_pac}")
    print(f"  Other: {other}")

def main():
    """Main function."""
    print(f"TLD Update Script - {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print("=" * 50)
    
    # Download root zone
    root_zone_content = download_root_zone()
    print(f"Downloaded {len(root_zone_content)} bytes")
    
    # Parse TLDs
    tlds = parse_root_zone(root_zone_content)
    
    # Generate Rust code
    rust_code = generate_rust_code(tlds)
    
    # Write to file
    write_rust_file(rust_code)
    
    # Print statistics
    print_statistics(tlds)
    
    print("\n✅ TLD update completed successfully!")
    print("\nNext steps:")
    print("1. Run 'cargo build' to compile with new TLDs")
    print("2. Run 'cargo test' to verify everything works")
    print("3. Commit the updated src/tld.rs file")

if __name__ == "__main__":
    main()