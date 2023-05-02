{{/*
Expand the name of the chart.
*/}}
{{- define "vpnaas.name" -}}
{{- printf "%s-%s" (default .Chart.Name .Values.nameOverride) .name | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Create chart name and version as used by the chart label.
*/}}
{{- define "vpnaas.chart" -}}
{{- printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end }}

{{/*
Common labels
*/}}
{{- define "vpnaas.labels" -}}
helm.sh/chart: {{ include "vpnaas.chart" . }}
{{ include "vpnaas.selectorLabels" . }}
app.kubernetes.io/managed-by: {{ .Release.Service }}
{{- end }}

{{/*
Selector labels
*/}}
{{- define "vpnaas.selectorLabels" -}}
app.kubernetes.io/name: {{ include "vpnaas.name" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end }}

{{/*
Generates configuration for a microservice from a default
*/}}
{{- define "vpnaas.microservice-config" -}}
{{- $defaults := deepCopy .Values.microservices._default -}}
{{- $overrides := index .Values.microservices .name -}}
{{- merge
        $defaults
        $overrides
        (dict
            "name" .name
            "labels" (include "vpnaas.labels" . | fromYaml)
            "selectorLabels" (include "vpnaas.selectorLabels" . | fromYaml)
        )
    | toYaml
-}}
{{- end }}
