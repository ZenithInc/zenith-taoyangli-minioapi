#!/usr/bin/env bash
set -Eeuo pipefail

readonly kube_context='k3s-tizi'
readonly namespace='taoyangli-prod'
readonly project_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
readonly production_env='/home/jinzhi/workspace/zenith/tao/tools.production.env'
readonly expected_keys_file="${project_dir}/deploy/tools-env.keys"
readonly required_nonempty_file="${project_dir}/deploy/tools-env.required-nonempty"

usage() {
    printf 'usage: %s name|validate|apply\n' "$0" >&2
    exit 64
}

require_tools() {
    local tool
    for tool in awk base64 comm date jq kubectl readlink sha256sum sort stat; do
        command -v "$tool" >/dev/null || {
            printf 'required tool is missing: %s\n' "$tool" >&2
            exit 1
        }
    done
}

require_change_window() {
    local window_date current_hour
    window_date=$(TZ=Asia/Shanghai date -d '3 hours ago' +%F)
    current_hour=$((10#$(TZ=Asia/Shanghai date +%H)))
    [[ "${TAOYANGLI_CHANGE_APPROVED:-}" == "$window_date" ]] || {
        printf 'set TAOYANGLI_CHANGE_APPROVED to the Beijing change-window date after the user gives the start command\n' >&2
        exit 1
    }
    (( current_hour >= 22 || current_hour < 3 )) || {
        printf 'production Secret changes are allowed only during 22:00-03:00 Beijing time\n' >&2
        exit 1
    }
}

validate_env() {
    [[ -f "$production_env" && ! -L "$production_env" ]] || {
        printf 'production env is missing or is a symlink\n' >&2
        exit 1
    }
    [[ "$(stat -c '%a' "$production_env")" == '600' ]] || {
        printf 'production env must have mode 0600\n' >&2
        exit 1
    }
    [[ "$(readlink -f "$production_env")" != "$project_dir"/* ]] || {
        printf 'production env must be outside the repository\n' >&2
        exit 1
    }
    awk '
        /^[[:space:]]*($|#)/ { next }
        ! /^[A-Za-z_][A-Za-z0-9_]*=/ { invalid = 1 }
        END { exit invalid }
    ' "$production_env" || {
        printf 'production env contains unsupported syntax\n' >&2
        exit 1
    }

    local actual_keys expected_keys key_count unique_count
    actual_keys=$(awk -F= '/^[A-Za-z_][A-Za-z0-9_]*=/{print $1}' "$production_env" | LC_ALL=C sort)
    expected_keys=$(LC_ALL=C sort "$expected_keys_file")
    key_count=$(printf '%s\n' "$actual_keys" | awk 'NF { count++ } END { print count + 0 }')
    unique_count=$(printf '%s\n' "$actual_keys" | awk 'NF && !seen[$0]++ { count++ } END { print count + 0 }')
    [[ "$key_count" == "$unique_count" ]] || {
        printf 'production env contains duplicate keys\n' >&2
        exit 1
    }
    [[ "$actual_keys" == "$expected_keys" ]] || {
        printf 'production env key set does not match the committed manifest\n' >&2
        comm -3 <(printf '%s\n' "$expected_keys") <(printf '%s\n' "$actual_keys") >&2
        exit 1
    }

    local required_key
    while IFS= read -r required_key; do
        [[ -n "$required_key" ]] || continue
        awk -F= -v key="$required_key" '
            $1 == key { found = 1; if (length(substr($0, length($1) + 2)) > 0) nonempty = 1 }
            END { exit !(found && nonempty) }
        ' "$production_env" || {
            printf 'required production env key is empty: %s\n' "$required_key" >&2
            exit 1
        }
    done < "$required_nonempty_file"

    local token_mode
    token_mode=$(awk -F= '$1 == "TOOLS_TOKEN_MODE" {print substr($0, length($1) + 2)}' "$production_env")
    [[ "$token_mode" == 'shadow-readonly' || "$token_mode" == 'active' ]] || {
        printf 'TOOLS_TOKEN_MODE must be shadow-readonly or active\n' >&2
        exit 1
    }

}

secret_name() {
    local digest
    digest=$(awk '/^[A-Za-z_][A-Za-z0-9_]*=/{print}' "$production_env" | LC_ALL=C sort | sha256sum | awk '{print substr($1, 1, 12)}')
    printf 'tools-env-%s\n' "$digest"
}

keyset_hash() {
    awk -F= '/^[A-Za-z_][A-Za-z0-9_]*=/{print $1}' "$production_env" |
        LC_ALL=C sort -u |
        sha256sum |
        awk '{print $1}'
}

apply_secret() {
    require_change_window
    local name existing_json existing_digest
    name=$(secret_name)
    if kubectl --context "$kube_context" -n "$namespace" get secret "$name" >/dev/null 2>&1; then
        existing_json=$(kubectl --context "$kube_context" -n "$namespace" get secret "$name" -o json)
        [[ "$(jq -r '.immutable // false' <<<"$existing_json")" == 'true' ]] || {
            printf 'existing versioned Secret is not immutable\n' >&2
            exit 1
        }
        existing_digest=$(
            jq -r '.data | to_entries | sort_by(.key)[] | [.key, .value] | @tsv' <<<"$existing_json" |
                while IFS=$'\t' read -r key value; do
                    printf '%s=' "$key"
                    printf '%s' "$value" | base64 --decode
                    printf '\n'
                done |
                sha256sum |
                awk '{print substr($1, 1, 12)}'
        )
        [[ "$name" == "tools-env-${existing_digest}" ]] || {
            printf 'existing Secret does not match its content-digest name\n' >&2
            exit 1
        }
    else
        kubectl --context "$kube_context" -n "$namespace" create secret generic "$name"             --from-env-file="$production_env" --dry-run=client -o json |
            jq '.immutable = true' |
            kubectl --context "$kube_context" -n "$namespace" apply -f - >/dev/null
    fi
    printf 'secret_name=%s keyset_sha256=%s\n' "$name" "$(keyset_hash)"
}

[[ $# -eq 1 ]] || usage
require_tools
validate_env
case "$1" in
    name|validate)
        printf 'secret_name=%s keyset_sha256=%s\n' "$(secret_name)" "$(keyset_hash)"
        ;;
    apply)
        apply_secret
        ;;
    *)
        usage
        ;;
esac
