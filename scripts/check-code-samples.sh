python3 -m venv .venv

source .venv/bin/activate

pip install pyyaml

wget https://raw.githubusercontent.com/meilisearch/documentation/main/.code-samples.meilisearch.yaml -O reference.yaml

python3 scripts/check-code-samples.py .code-samples.meilisearch.yaml reference.yaml

rm reference.yaml
