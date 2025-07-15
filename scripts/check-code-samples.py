#!/usr/bin/env python3
import yaml
import sys

# Validate input arguments
if len(sys.argv) != 3:
    print("Usage: python compare_yaml_keys.py <your.yaml> <reference.yaml>")
    sys.exit(1)

your_path = sys.argv[1]
reference_path = sys.argv[2]

# Configured ignore/include lists
# Gotten from https://github.com/meilisearch/integration-automations/blob/main/code-samples-checkers/missing-cs-in-integration.sh
NOT_NEEDED_IN_INTEGRATION = {
    'tenant_token_guide_search_no_sdk_1',
    'updating_guide_check_version_new_authorization_header',
    'updating_guide_check_version_old_authorization_header',
    'updating_guide_get_displayed_attributes_old_authorization_header',
    'updating_guide_reset_displayed_attributes_old_authorization_header',
    'updating_guide_create_dump',
}

NOT_IN_DOCS_CODE_SAMPLES_FILE = {
    'tenant_token_guide_generate_sdk_1',
    'tenant_token_guide_search_sdk_1',
    'landing_getting_started_1',
}

# Load YAML files
with open(your_path) as f1, open(reference_path) as f2:
    your_yaml = yaml.safe_load(f1) or {}
    ref_yaml = yaml.safe_load(f2) or {}

# Collect keys
your_keys = set(your_yaml.keys())
ref_keys = set(ref_yaml.keys())

# Print results
print("❌ Incorrect:")
print("\n".join(sorted(your_keys - ref_keys - NOT_IN_DOCS_CODE_SAMPLES_FILE)))

print("\n🔁 Missing:")
print("\n".join(sorted(ref_keys - your_keys - NOT_NEEDED_IN_INTEGRATION)))
