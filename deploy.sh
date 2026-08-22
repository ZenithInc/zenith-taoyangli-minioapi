#!/usr/bin/env bash
set -Eeuo pipefail

readonly kube_context='k3s-tizi'
readonly namespace='taoyangli-prod'
readonly release='tools'
readonly repository='registry.taoyangli.cn/taoyangli/tools'
readonly pull_secret='harbor-pull'
readonly project_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
readonly chart_dir="${project_dir}/chart"
readonly expected_keys_file="${project_dir}/deploy/tools-env.keys"

usage() {
    printf 'usage:\n' >&2
    printf '  %s offline-validate FULL_GIT_SHA sha256:DIGEST SECRET_NAME\n' "$0" >&2
    printf '  %s validate FULL_GIT_SHA sha256:DIGEST SECRET_NAME\n' "$0" >&2
    printf '  %s stage FULL_GIT_SHA sha256:DIGEST SECRET_NAME\n' "$0" >&2
    printf '  %s status\n' "$0" >&2
    printf '  %s rollback HELM_REVISION\n' "$0" >&2
    exit 64
}

require_tools() {
    local tool
    for tool in base64 date docker git helm jq kubectl python3 sha256sum sort; do
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
        printf 'production access is allowed only during 22:00-03:00 Beijing time\n' >&2
        exit 1
    }
}

validate_args() {
    [[ "$1" =~ ^[0-9a-f]{40}$ ]] || { printf 'image tag must be a full Git SHA\n' >&2; exit 64; }
    [[ "$2" =~ ^sha256:[0-9a-f]{64}$ ]] || { printf 'image digest is invalid\n' >&2; exit 64; }
    [[ "$3" =~ ^tools-env-[0-9a-f]{12}$ ]] || { printf 'Secret name is invalid\n' >&2; exit 64; }
}

validate_commit() {
    [[ "$(git -C "$project_dir" rev-parse HEAD)" == "$1" ]] || {
        printf 'requested SHA does not match checked-out commit\n' >&2
        exit 1
    }
    [[ -z "$(git -C "$project_dir" status --porcelain --untracked-files=all)" ]] || {
        printf 'repository worktree must be clean\n' >&2
        exit 1
    }
}

render_chart() {
    local git_sha=$1 digest=$2 secret_name=$3 destination=$4
    shift 4
    helm lint "$chart_dir"         --set-string "image.tag=${git_sha}"         --set-string "image.digest=${digest}"         --set-string "runtimeSecretName=${secret_name}" "$@"
    helm template "$release" "$chart_dir" --namespace "$namespace"         --set-string "image.tag=${git_sha}"         --set-string "image.digest=${digest}"         --set-string "runtimeSecretName=${secret_name}" "$@" > "$destination"
}

offline_validate() {
    local git_sha=$1 digest=$2 secret_name=$3
    shift 3
    validate_args "$git_sha" "$digest" "$secret_name"
    validate_commit "$git_sha"
    local rendered
    rendered=$(mktemp)
    render_chart "$git_sha" "$digest" "$secret_name" "$rendered" "$@"
    python3 -c 'import sys,yaml; list(yaml.safe_load_all(open(sys.argv[1], encoding="utf-8")))' "$rendered"
    rm -f -- "$rendered"
    printf 'offline validation passed for %s:%s@%s\n' "$repository" "$git_sha" "$digest"
}

validate_image() {
    local git_sha=$1 digest=$2 metadata architecture revision
    metadata=$(docker buildx imagetools inspect --format '{{json .Image}}' "${repository}:${git_sha}@${digest}")
    architecture=$(jq -r '.Architecture // empty' <<<"$metadata")
    revision=$(jq -r '.Config.Labels["org.opencontainers.image.revision"] // empty' <<<"$metadata")
    [[ "$architecture" == 'amd64' ]] || { printf 'image architecture is not amd64\n' >&2; exit 1; }
    [[ "$revision" == "$git_sha" ]] || { printf 'image revision label does not match Git SHA\n' >&2; exit 1; }
}

validate_cluster_secret() {
    local secret_name=$1 json keys expected digest
    kubectl --context "$kube_context" -n "$namespace" get secret "$pull_secret" >/dev/null
    json=$(kubectl --context "$kube_context" -n "$namespace" get secret "$secret_name" -o json)
    [[ "$(jq -r '.immutable // false' <<<"$json")" == 'true' ]] || {
        printf 'runtime Secret is not immutable\n' >&2
        exit 1
    }
    keys=$(jq -r '.data | keys[]' <<<"$json" | LC_ALL=C sort)
    expected=$(LC_ALL=C sort "$expected_keys_file")
    [[ "$keys" == "$expected" ]] || { printf 'cluster Secret key set does not match manifest\n' >&2; exit 1; }
    digest=$(
        jq -r '.data | to_entries | sort_by(.key)[] | [.key, .value] | @tsv' <<<"$json" |
            while IFS=$'\t' read -r key value; do
                printf '%s=' "$key"
                printf '%s' "$value" | base64 --decode
                printf '\n'
            done |
            sha256sum |
            awk '{print substr($1, 1, 12)}'
    )
    [[ "$secret_name" == "tools-env-${digest}" ]] || {
        printf 'cluster Secret content does not match its digest name\n' >&2
        exit 1
    }
    printf 'secret=%s keyset_sha256=%s\n' "$secret_name" "$(printf '%s\n' "$keys" | sha256sum | awk '{print $1}')"
}

validate_capacity() {
    local rendered=$1 temporary_dir
    temporary_dir=$(mktemp -d)
    kubectl --context "$kube_context" get nodes -o json > "${temporary_dir}/nodes.json"
    kubectl --context "$kube_context" get pods -A -o json > "${temporary_dir}/pods.json"
    python3 "${project_dir}/scripts/check-capacity.py"         --nodes "${temporary_dir}/nodes.json"         --pods "${temporary_dir}/pods.json"         --rendered "$rendered" --release "$release" --limit-percent 70
    rm -f -- "${temporary_dir}/nodes.json" "${temporary_dir}/pods.json"
    rmdir -- "$temporary_dir"
}

cluster_validate() {
    local git_sha=$1 digest=$2 secret_name=$3
    shift 3
    require_change_window
    offline_validate "$git_sha" "$digest" "$secret_name" "$@"
    validate_image "$git_sha" "$digest"
    validate_cluster_secret "$secret_name"
    local rendered
    rendered=$(mktemp)
    render_chart "$git_sha" "$digest" "$secret_name" "$rendered" "$@"
    kubectl --context "$kube_context" -n "$namespace" apply --dry-run=server -f "$rendered" >/dev/null
    validate_capacity "$rendered"
    rm -f -- "$rendered"
    printf 'cluster validation passed\n'
}

run_dependency_check() {
    local git_sha=$1 digest=$2 secret_name=$3
    local image_ref="${repository}:${git_sha}@${digest}"
    local pod_name="tools-dependency-check-${git_sha:0:12}"
    if kubectl --context "$kube_context" -n "$namespace" get pod "$pod_name" >/dev/null 2>&1; then
        printf 'dependency check Pod already exists; inspect it explicitly: %s\n' "$pod_name" >&2
        return 1
    fi
    kubectl --context "$kube_context" -n "$namespace" create -f - >/dev/null <<EOF
apiVersion: v1
kind: Pod
metadata:
  name: ${pod_name}
  labels:
    app.kubernetes.io/name: tools
    app.kubernetes.io/component: dependency-check
spec:
  restartPolicy: Never
  activeDeadlineSeconds: 300
  automountServiceAccountToken: false
  imagePullSecrets:
    - name: ${pull_secret}
  securityContext:
    runAsNonRoot: true
    runAsUser: 10001
    runAsGroup: 10001
    seccompProfile: {type: RuntimeDefault}
  containers:
    - name: dependency-check
      image: ${image_ref}
      command: ["/usr/local/bin/taoyangli-tools", "dependency-check"]
      resources:
        requests: {cpu: 100m, memory: 128Mi}
        limits: {cpu: 500m, memory: 512Mi}
      securityContext:
        allowPrivilegeEscalation: false
        readOnlyRootFilesystem: true
        capabilities: {drop: ["ALL"]}
      envFrom:
        - secretRef: {name: ${secret_name}}
      volumeMounts:
        - {name: tmp, mountPath: /tmp}
  volumes:
    - name: tmp
      emptyDir: {sizeLimit: 32Mi}
EOF
    if ! kubectl --context "$kube_context" -n "$namespace" wait         --for=jsonpath='{.status.phase}'=Succeeded "pod/$pod_name" --timeout=300s; then
        kubectl --context "$kube_context" -n "$namespace" logs "$pod_name" >&2 || true
        printf 'dependency check failed; Pod retained for diagnosis\n' >&2
        return 1
    fi
    kubectl --context "$kube_context" -n "$namespace" logs "$pod_name"
    kubectl --context "$kube_context" -n "$namespace" delete pod "$pod_name" --wait=true >/dev/null
}

stage_release() {
    local git_sha=$1 digest=$2 secret_name=$3
    local -a overrides=()
    cluster_validate "$git_sha" "$digest" "$secret_name" "${overrides[@]}"
    run_dependency_check "$git_sha" "$digest" "$secret_name"
    helm upgrade --install "$release" "$chart_dir"         --kube-context "$kube_context" --namespace "$namespace"         --set-string "image.tag=${git_sha}"         --set-string "image.digest=${digest}"         --set-string "runtimeSecretName=${secret_name}"         "${overrides[@]}" --history-max 10 --wait --timeout 10m
    kubectl --context "$kube_context" -n "$namespace" rollout status         "deployment/${release}" --timeout=5m
}

rollback_release() {
    require_change_window
    [[ "$1" =~ ^[1-9][0-9]*$ ]] || usage
    [[ "${TAOYANGLI_ROLLBACK_APPROVED:-}" == "$1" ]] || {
        printf 'set TAOYANGLI_ROLLBACK_APPROVED to the explicitly approved revision\n' >&2
        exit 1
    }
    helm rollback "$release" "$1" --kube-context "$kube_context"         --namespace "$namespace" --wait --timeout 10m
}

require_tools
case "${1:-}" in
    offline-validate)
        [[ $# -eq 4 ]] || usage
        offline_validate "$2" "$3" "$4"
        ;;
    validate)
        [[ $# -eq 4 ]] || usage
        cluster_validate "$2" "$3" "$4"
        ;;
    stage)
        [[ $# -eq 4 ]] || usage
        stage_release "$2" "$3" "$4"
        ;;
    status)
        [[ $# -eq 1 ]] || usage
        require_change_window
        helm status "$release" --kube-context "$kube_context" -n "$namespace"
        kubectl --context "$kube_context" -n "$namespace" get deployment,pod,service,ingress             -l "app.kubernetes.io/instance=${release}" -o wide
        ;;
    rollback)
        [[ $# -eq 2 ]] || usage
        rollback_release "$2"
        ;;
    *)
        usage
        ;;
esac
