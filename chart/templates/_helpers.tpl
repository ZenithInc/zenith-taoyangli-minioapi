{{- define "tools.name" -}}taoyangli-tools{{- end }}

{{- define "tools.fullname" -}}
{{- .Release.Name | trunc 63 | trimSuffix "-" -}}
{{- end }}

{{- define "tools.selectorLabels" -}}
app.kubernetes.io/name: {{ include "tools.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{- define "tools.labels" -}}
helm.sh/chart: {{ printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | quote }}
{{ include "tools.selectorLabels" . }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{- define "tools.image" -}}
{{- $repository := required "image.repository is required" .Values.image.repository -}}
{{- if ne $repository "registry.taoyangli.cn/taoyangli/tools" -}}{{- fail "unexpected image.repository" -}}{{- end -}}
{{- $tag := required "image.tag must be a full Git SHA" .Values.image.tag -}}
{{- if not (regexMatch "^[0-9a-f]{40}$" $tag) -}}{{- fail "image.tag must be a full lowercase Git SHA" -}}{{- end -}}
{{- $digest := required "image.digest is required" .Values.image.digest -}}
{{- if not (regexMatch "^sha256:[0-9a-f]{64}$" $digest) -}}{{- fail "image.digest must be a sha256 digest" -}}{{- end -}}
{{- printf "%s:%s@%s" $repository $tag $digest -}}
{{- end }}

{{- define "tools.runtimeSecret" -}}
{{- $name := required "runtimeSecretName is required" .Values.runtimeSecretName -}}
{{- if not (regexMatch "^tools-env-[0-9a-f]{12}$" $name) -}}{{- fail "runtimeSecretName must use the content-digest suffix" -}}{{- end -}}
{{- $name -}}
{{- end }}
