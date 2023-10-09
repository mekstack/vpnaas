{{- define "common.microservice" -}}
apiVersion: apps/v1
kind: Deployment
metadata:
  name: {{ .Values.name }}
  labels:
    {{- include "common.labels" . | nindent 4 }}
spec:
  replicas: {{ .Values.replicaCount }}
  selector:
    matchLabels:
      {{- include "common.selectorLabels" . | nindent 6 }}
  template:
    metadata:
      labels:
        {{- include "common.selectorLabels" . | nindent 8 }}
    spec:
      securityContext:
        {{- toYaml .podSecurityContext | nindent 12 }}
      containers:
        - name: {{ .Values.name }}
          image: {{ printf "%s/%s:%s" .Values.repository .Values.name .Values.tag | quote }}
          imagePullPolicy: {{ .Values.pullPolicy }}
          securityContext:
            {{- toYaml .Values.containerSecurityContext | nindent 12 }}
          ports:
            {{- range .Values.containerPorts }}
            - containerPort: {{ .containerPort }}
              protocol: {{ .protocol }}
            {{- end }}
          env:
            {{- toYaml .Values.env | nindent 12 }}
---
apiVersion: v1
kind: Service
metadata:
  name: {{ .Values.name }}-http
  labels:
    {{- include "common.labels" . | nindent 4 }}
spec:
  type: ClusterIP
  ports:
    - port: 80
      targetPort: 80
      protocol: TCP
  selector:
    {{- include "common.selectorLabels" . | nindent 4 }}
{{- end}}
