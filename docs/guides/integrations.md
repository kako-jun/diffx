# Integration Guide

This comprehensive guide covers integrating `diffx` into various development workflows, CI/CD pipelines, and automation systems.

## Table of Contents

- [CI/CD Platforms](#cicd-platforms)
- [Version Control Integration](#version-control-integration) 
- [Container Ecosystems](#container-ecosystems)
- [Cloud Platforms](#cloud-platforms)
- [Monitoring & Alerting](#monitoring--alerting)
- [Development Tools](#development-tools)
- [Automation Scripts](#automation-scripts)

## CI/CD Platforms

### GitHub Actions

#### Basic Configuration Validation

```yaml
name: Configuration Validation

on:
  pull_request:
    paths:
      - 'config/**'
      - '**/*.json'
      - '**/*.yaml'
      - '**/*.yml'

jobs:
  validate-config:
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      with:
        fetch-depth: 0
    
    - name: Install diffx
      run: |
        curl -L "https://github.com/kako-jun/diffx/releases/latest/download/diffx-x86_64-unknown-linux-gnu.tar.gz" | tar -xz
        sudo mv diffx /usr/local/bin/
        diffx --version
    
    - name: Validate configuration changes
      run: |
        # Get changed files
        CHANGED_FILES=$(git diff --name-only origin/${{ github.base_ref }}...HEAD | grep -E '\.(json|yaml|yml|toml)$' || true)
        
        if [ -n "$CHANGED_FILES" ]; then
          echo "Validating changed configuration files:"
          echo "$CHANGED_FILES"
          
          for file in $CHANGED_FILES; do
            if [ -f "$file" ]; then
              echo "=== Analyzing $file ==="
              
              # Compare with base branch version
              git show origin/${{ github.base_ref }}:"$file" > /tmp/base_file 2>/dev/null || {
                echo "New file: $file"
                continue
              }
              
              # Run diffx with configuration-specific settings
              diffx /tmp/base_file "$file" \
                --ignore-keys-regex "^(timestamp|lastModified|createdAt|updatedAt|buildTime)$" \
                --output json > "/tmp/diff_${file//\//_}.json"
              
              # Check for critical changes
              if [ -s "/tmp/diff_${file//\//_}.json" ]; then
                echo "Changes detected in $file:"
                cat "/tmp/diff_${file//\//_}.json" | jq -r '.[] | 
                  if .Added then "  + \(.Added[0]): \(.Added[1])"
                  elif .Removed then "  - \(.Removed[0]): \(.Removed[1])"
                  elif .Modified then "  ~ \(.Modified[0]): \(.Modified[1]) → \(.Modified[2])"
                  elif .TypeChanged then "  ! \(.TypeChanged[0]): \(.TypeChanged[1]) → \(.TypeChanged[2]) (type changed)"
                  else . end'
                
                # Flag critical changes
                CRITICAL=$(cat "/tmp/diff_${file//\//_}.json" | jq -r '.[] | 
                  select(.Removed or .TypeChanged or 
                         (.Modified and (.Modified[0] | contains("security") or contains("database") or contains("auth"))))')
                
                if [ -n "$CRITICAL" ]; then
                  echo "⚠️ Critical changes detected in $file - requires review"
                  echo "$CRITICAL" | jq -r '.[]'
                  echo "::warning title=Critical Config Change::Critical changes detected in $file"
                fi
              else
                echo "✅ No semantic changes in $file (formatting only)"
              fi
              echo ""
            fi
          done
        else
          echo "No configuration files changed"
        fi
```

#### API Contract Testing

```yaml
name: API Contract Validation

on:
  schedule:
    - cron: '0 */4 * * *'  # Every 4 hours
  workflow_dispatch:

jobs:
  api-contract-test:
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout repository
      uses: actions/checkout@v4
    
    - name: Install diffx
      run: cargo install diffx
    
    - name: Test API contracts
      env:
        API_BASE_URL: ${{ secrets.API_BASE_URL }}
        API_KEY: ${{ secrets.API_KEY }}
      run: |
        #!/bin/bash
        set -e
        
        # Define endpoints to test
        ENDPOINTS=("users" "products" "orders" "health")
        FAILED_TESTS=()
        
        for endpoint in "${ENDPOINTS[@]}"; do
          echo "Testing $endpoint endpoint..."
          
          # Fetch current response
          curl -H "Authorization: Bearer $API_KEY" \
               -H "Accept: application/json" \
               "$API_BASE_URL/$endpoint" > "actual_$endpoint.json"
          
          # Compare with expected schema
          if diffx "tests/api_contracts/$endpoint.json" "actual_$endpoint.json" \
             --ignore-keys-regex "^(timestamp|requestId|serverId|responseTime)$" \
             --output json > "diff_$endpoint.json"; then
            echo "✅ $endpoint contract matches"
          else
            echo "❌ $endpoint contract violation detected"
            FAILED_TESTS+=("$endpoint")
            
            # Create detailed report
            echo "## $endpoint Contract Violation" >> contract_violations.md
            echo '```json' >> contract_violations.md
            cat "diff_$endpoint.json" >> contract_violations.md
            echo '```' >> contract_violations.md
            echo "" >> contract_violations.md
          fi
        done
        
        # Report results
        if [ ${#FAILED_TESTS[@]} -gt 0 ]; then
          echo "Contract violations found in: ${FAILED_TESTS[*]}"
          
          # Create GitHub issue for violations
          if [ -f contract_violations.md ]; then
            gh issue create \
              --title "API Contract Violations Detected" \
              --body-file contract_violations.md \
              --label "api,contract-violation,automation"
          fi
          
          exit 1
        else
          echo "All API contracts validated successfully"
        fi
```

### GitLab CI

#### Multi-Environment Configuration Validation

```yaml
# .gitlab-ci.yml
stages:
  - validate
  - deploy

variables:
  DIFFX_VERSION: "0.2.0"

.install_diffx: &install_diffx
  - |
    if ! command -v diffx &> /dev/null; then
      cargo install diffx --version $DIFFX_VERSION
    fi

validate_configs:
  stage: validate
  image: rust:latest
  before_script:
    - *install_diffx
  script:
    - |
      # Validate configuration consistency across environments
      ENVIRONMENTS=("development" "staging" "production")
      
      for env in "${ENVIRONMENTS[@]}"; do
        if [ "$env" != "production" ]; then
          echo "Comparing $env with production configuration..."
          
          # Compare app configs
          diffx "config/production.yaml" "config/$env.yaml" \
            --ignore-keys-regex "^(environment|host|port|replicas|resources\..*)" \
            --output json > "diff_${env}_prod.json"
          
          # Check for unexpected differences
          UNEXPECTED_DIFFS=$(cat "diff_${env}_prod.json" | jq -r '.[] | 
            select(.Added or .Removed or 
                   (.Modified and (.Modified[0] | 
                    contains("security") or contains("auth") or contains("database"))))')
          
          if [ -n "$UNEXPECTED_DIFFS" ]; then
            echo "⚠️ Unexpected configuration differences between $env and production:"
            echo "$UNEXPECTED_DIFFS" | jq -r '.'
            echo "Please review these changes for security and compatibility."
          fi
        fi
      done
  artifacts:
    reports:
      junit: config_validation_report.xml
    paths:
      - diff_*.json
    expire_in: 1 week
  only:
    changes:
      - config/**/*
```

#### Infrastructure as Code Validation

```yaml
validate_terraform:
  stage: validate
  image: hashicorp/terraform:latest
  before_script:
    - apk add --no-cache curl jq
    - *install_diffx
  script:
    - |
      # Validate Terraform plan changes
      terraform init
      terraform plan -out=tfplan
      terraform show -json tfplan > planned_changes.json
      
      # Compare with current state
      terraform show -json > current_state.json
      
      # Focus on resource changes
      diffx current_state.json planned_changes.json \
        --path "planned_values.root_module.resources" \
        --ignore-keys-regex "^(timeouts|creation_time|last_updated)" \
        --output json > terraform_diff.json
      
      # Analyze impact
      CRITICAL_CHANGES=$(cat terraform_diff.json | jq -r '.[] | 
        select(.Removed or (.Modified and (.Modified[0] | 
          contains("security_group") or contains("iam") or contains("vpc"))))')
      
      if [ -n "$CRITICAL_CHANGES" ]; then
        echo "🔴 Critical infrastructure changes detected!"
        echo "$CRITICAL_CHANGES" | jq -r '.'
        echo "Manual approval required for deployment."
        exit 1
      fi
  when: manual
  allow_failure: false
```

### Jenkins Pipeline

#### Declarative Pipeline for Configuration Management

```groovy
pipeline {
    agent any
    
    environment {
        DIFFX_VERSION = '0.2.0'
    }
    
    stages {
        stage('Setup') {
            steps {
                script {
                    // Install diffx if not present
                    sh '''
                        if ! command -v diffx &> /dev/null; then
                            curl -L "https://github.com/kako-jun/diffx/releases/latest/download/diffx-x86_64-unknown-linux-gnu.tar.gz" | tar -xz
                            sudo mv diffx /usr/local/bin/
                        fi
                        diffx --version
                    '''
                }
            }
        }
        
        stage('Validate Configuration Changes') {
            when {
                changeset "config/**"
            }
            steps {
                script {
                    def changedFiles = sh(
                        script: "git diff --name-only HEAD~1 HEAD | grep -E '\\.(json|yaml|yml|toml)\$' || true",
                        returnStdout: true
                    ).trim()
                    
                    if (changedFiles) {
                        changedFiles.split('\n').each { file ->
                            echo "Analyzing ${file}..."
                            
                            sh """
                                git show HEAD~1:${file} > old_${file} 2>/dev/null || echo '{}' > old_${file}
                                
                                diffx old_${file} ${file} \\
                                    --ignore-keys-regex "^(timestamp|version|buildNumber)\$" \\
                                    --output json > diff_${file.replaceAll('/', '_')}.json || true
                                
                                if [ -s diff_${file.replaceAll('/', '_')}.json ]; then
                                    echo "Changes detected in ${file}:"
                                    cat diff_${file.replaceAll('/', '_')}.json | jq -r '.[]'
                                else
                                    echo "No semantic changes in ${file}"
                                fi
                            """
                        }
                    }
                }
            }
            post {
                always {
                    archiveArtifacts artifacts: 'diff_*.json', allowEmptyArchive: true
                }
            }
        }
        
        stage('Deploy') {
            when {
                branch 'main'
            }
            steps {
                script {
                    // Deployment logic here
                    echo "Deploying configuration changes..."
                }
            }
        }
    }
    
    post {
        failure {
            emailext (
                subject: "Configuration Validation Failed: ${env.JOB_NAME} - ${env.BUILD_NUMBER}",
                body: "Configuration validation failed. Please check the build logs for details.",
                to: "${env.CHANGE_AUTHOR_EMAIL}"
            )
        }
    }
}
```

## Version Control Integration

### Git Hooks

#### Pre-commit Hook for Configuration Validation

```bash
#!/bin/bash
# .git/hooks/pre-commit

# Check if diffx is available
if ! command -v diffx &> /dev/null; then
    echo "Warning: diffx not found. Install with: cargo install diffx"
    exit 0
fi

# Get staged files
STAGED_FILES=$(git diff --cached --name-only --diff-filter=AM | grep -E '\.(json|yaml|yml|toml)$' || true)

if [ -z "$STAGED_FILES" ]; then
    exit 0
fi

echo "Validating staged configuration files..."

VALIDATION_FAILED=false

for file in $STAGED_FILES; do
    echo "Validating $file..."
    
    # Check if file exists in HEAD (for modifications)
    if git cat-file -e HEAD:"$file" 2>/dev/null; then
        # Compare staged version with HEAD
        git show HEAD:"$file" > /tmp/head_version
        git show :"$file" > /tmp/staged_version
        
        # Run diffx with strict validation
        if diffx /tmp/head_version /tmp/staged_version \
           --ignore-keys-regex "^(timestamp|lastModified)$" \
           --output json > /tmp/diff_output.json; then
            echo "✅ $file: No semantic changes"
        else
            echo "📝 $file: Changes detected"
            
            # Check for potentially dangerous changes
            DANGEROUS_CHANGES=$(cat /tmp/diff_output.json | jq -r '.[] | 
                select(.Removed or .TypeChanged or 
                       (.Modified and (.Modified[0] | 
                        contains("security") or contains("password") or 
                        contains("secret") or contains("key"))))')
            
            if [ -n "$DANGEROUS_CHANGES" ]; then
                echo "⚠️  WARNING: Potentially dangerous changes in $file:"
                echo "$DANGEROUS_CHANGES" | jq -r '.'
                echo ""
                read -p "Continue with commit? (y/N): " -n 1 -r
                echo
                if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                    VALIDATION_FAILED=true
                fi
            fi
        fi
        
        # Cleanup
        rm -f /tmp/head_version /tmp/staged_version /tmp/diff_output.json
    else
        echo "ℹ️  $file: New file"
    fi
done

if [ "$VALIDATION_FAILED" = true ]; then
    echo "Commit aborted due to validation concerns."
    exit 1
fi

echo "Configuration validation completed successfully."
```

#### Post-receive Hook for Deployment Validation

```bash
#!/bin/bash
# hooks/post-receive

while read oldrev newrev refname; do
    # Only process main branch
    if [ "$refname" = "refs/heads/main" ]; then
        echo "Validating deployment readiness for main branch..."
        
        # Get changed configuration files
        CHANGED_CONFIGS=$(git diff --name-only $oldrev..$newrev | grep -E 'config/.*\.(json|yaml|yml)$' || true)
        
        if [ -n "$CHANGED_CONFIGS" ]; then
            echo "Configuration changes detected:"
            echo "$CHANGED_CONFIGS"
            
            # Validate each changed config
            for config in $CHANGED_CONFIGS; do
                echo "Validating $config..."
                
                # Extract old and new versions
                git show $oldrev:$config > /tmp/old_config 2>/dev/null || echo '{}' > /tmp/old_config
                git show $newrev:$config > /tmp/new_config
                
                # Run comprehensive validation
                diffx /tmp/old_config /tmp/new_config \
                    --ignore-keys-regex "^(version|buildNumber|timestamp)$" \
                    --output json > /tmp/config_diff.json
                
                if [ -s /tmp/config_diff.json ]; then
                    # Trigger deployment pipeline
                    echo "Configuration changes require deployment update"
                    
                    # Example: Trigger Jenkins job
                    curl -X POST "$JENKINS_URL/job/deploy-config/build" \
                         --user "$JENKINS_USER:$JENKINS_TOKEN" \
                         --data-urlencode "json={\"parameter\": [{\"name\":\"config_file\", \"value\":\"$config\"}]}"
                fi
                
                rm -f /tmp/old_config /tmp/new_config /tmp/config_diff.json
            done
        fi
    fi
done
```

### Git Aliases

Add to `.gitconfig`:

```ini
[alias]
    # Compare current file with previous commit
    diffx-prev = "!f() { git show HEAD~1:\"$1\" | diffx - \"$1\"; }; f"
    
    # Compare file between two commits
    diffx-commits = "!f() { git show \"$1\":\"$3\" | diffx - <(git show \"$2\":\"$3\"); }; f"
    
    # Show semantic diff in git log
    logx = "!f() { git log --oneline \"$@\" | while read commit msg; do echo \"$commit: $msg\"; git diffx-prev HEAD~1 HEAD 2>/dev/null | head -5; echo; done; }; f"
    
    # Validate all configs before push
    validate-configs = "!find . -name '*.json' -o -name '*.yaml' -o -name '*.yml' | xargs -I {} sh -c 'echo \"Validating {}\"; diffx {} {} --output json > /dev/null && echo \"✅ {}\" || echo \"❌ {}\"'"
```

## Container Ecosystems

### Docker Integration

#### Multi-stage Build with Configuration Validation

```dockerfile
# Dockerfile
FROM rust:1.70-alpine AS diffx-builder
RUN cargo install diffx

FROM node:18-alpine AS app-builder
COPY --from=diffx-builder /usr/local/cargo/bin/diffx /usr/local/bin/

# Copy configuration files
COPY config/ ./config/
COPY config.schema.json ./

# Validate configuration during build
RUN diffx config/default.json config/production.json \
    --ignore-keys-regex "^(environment|host|port)$" \
    --output json > /tmp/config_diff.json && \
    if [ -s /tmp/config_diff.json ]; then \
        echo "Configuration validation completed"; \
        cat /tmp/config_diff.json; \
    fi

# Continue with app build...
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

FROM node:18-alpine
COPY --from=diffx-builder /usr/local/cargo/bin/diffx /usr/local/bin/
COPY --from=app-builder /app/dist ./dist
COPY --from=app-builder /app/config ./config
COPY --from=app-builder /tmp/config_diff.json ./

# Add health check that includes config validation
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD node health-check.js && diffx config/runtime.json config/expected.json --output json > /dev/null
```

#### Docker Compose with Configuration Monitoring

```yaml
# docker-compose.yml
version: '3.8'

services:
  app:
    build: .
    volumes:
      - ./config:/app/config:ro
      - config-monitor:/tmp/config-monitor
    environment:
      - CONFIG_MONITOR_ENABLED=true
    
  config-monitor:
    image: rust:1.70-alpine
    volumes:
      - ./config:/config:ro
      - config-monitor:/tmp/config-monitor
      - ./scripts:/scripts:ro
    command: |
      sh -c "
        cargo install diffx
        /scripts/config-monitor.sh
      "
    restart: unless-stopped

volumes:
  config-monitor:
```

```bash
#!/bin/bash
# scripts/config-monitor.sh

BASELINE_CONFIG="/config/baseline.json"
CURRENT_CONFIG="/config/current.json"
MONITOR_FILE="/tmp/config-monitor/status"

while true; do
    if [ -f "$CURRENT_CONFIG" ] && [ -f "$BASELINE_CONFIG" ]; then
        # Check for configuration drift
        if ! diffx "$BASELINE_CONFIG" "$CURRENT_CONFIG" \
             --ignore-keys-regex "^(timestamp|uptime|pid)$" \
             --output json > /tmp/config_drift.json; then
            
            echo "$(date): Configuration drift detected" >> "$MONITOR_FILE"
            cat /tmp/config_drift.json >> "$MONITOR_FILE"
            
            # Alert mechanism (webhook, slack, etc.)
            curl -X POST "$ALERT_WEBHOOK_URL" \
                 -H "Content-Type: application/json" \
                 -d "{\"message\": \"Configuration drift detected\", \"details\": $(cat /tmp/config_drift.json)}"
        else
            echo "$(date): Configuration stable" >> "$MONITOR_FILE"
        fi
    fi
    
    sleep 300  # Check every 5 minutes
done
```

### Kubernetes Integration

#### ConfigMap Validation

```yaml
# k8s-config-validator.yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: config-validator
spec:
  template:
    spec:
      containers:
      - name: validator
        image: rust:1.70-alpine
        command:
        - sh
        - -c
        - |
          cargo install diffx
          
          # Get current ConfigMap
          kubectl get configmap app-config -o jsonpath='{.data.config\.json}' > current_config.json
          
          # Compare with expected configuration
          diffx expected_config.json current_config.json \
            --ignore-keys-regex "^(namespace|resourceVersion|creationTimestamp)$" \
            --output json > config_validation.json
          
          if [ -s config_validation.json ]; then
            echo "Configuration validation issues found:"
            cat config_validation.json
            exit 1
          else
            echo "Configuration validation passed"
          fi
        volumeMounts:
        - name: expected-config
          mountPath: /expected_config.json
          subPath: config.json
      volumes:
      - name: expected-config
        configMap:
          name: expected-app-config
      restartPolicy: Never
  backoffLimit: 4
```

#### Helm Chart with Configuration Validation

```yaml
# templates/config-validation-job.yaml
{{- if .Values.configValidation.enabled }}
apiVersion: batch/v1
kind: Job
metadata:
  name: {{ include "myapp.fullname" . }}-config-validation
  annotations:
    "helm.sh/hook": pre-install,pre-upgrade
    "helm.sh/hook-weight": "-5"
spec:
  template:
    spec:
      containers:
      - name: config-validator
        image: {{ .Values.configValidation.image }}
        command:
        - sh
        - -c
        - |
          # Install diffx
          cargo install diffx
          
          # Validate Helm-generated config against schema
          echo '{{ .Values.config | toJson }}' > generated_config.json
          
          diffx schema_config.json generated_config.json \
            --ignore-keys-regex "{{ .Values.configValidation.ignoreKeys }}" \
            --output json > validation_result.json
          
          if [ -s validation_result.json ]; then
            echo "Helm configuration validation failed:"
            cat validation_result.json
            exit 1
          fi
        volumeMounts:
        - name: config-schema
          mountPath: /schema_config.json
          subPath: schema.json
      volumes:
      - name: config-schema
        configMap:
          name: {{ include "myapp.fullname" . }}-config-schema
      restartPolicy: Never
{{- end }}
```

## Cloud Platforms

### AWS Integration

#### Lambda Function for S3 Configuration Monitoring

```python
# lambda_function.py
import json
import boto3
import subprocess
import os
from datetime import datetime

def lambda_handler(event, context):
    s3 = boto3.client('s3')
    
    # Download diffx binary (pre-compiled for Lambda)
    if not os.path.exists('/tmp/diffx'):
        s3.download_file('my-tools-bucket', 'diffx-lambda', '/tmp/diffx')
        os.chmod('/tmp/diffx', 0o755)
    
    # Get the S3 object that triggered this function
    bucket = event['Records'][0]['s3']['bucket']['name']
    key = event['Records'][0]['s3']['object']['key']
    
    if not key.endswith(('.json', '.yaml', '.yml')):
        return {'statusCode': 200, 'body': 'Not a configuration file'}
    
    # Download current and baseline configurations
    s3.download_file(bucket, key, '/tmp/current_config')
    
    baseline_key = key.replace('current/', 'baseline/')
    try:
        s3.download_file(bucket, baseline_key, '/tmp/baseline_config')
    except:
        return {'statusCode': 200, 'body': 'No baseline configuration found'}
    
    # Run diffx comparison
    result = subprocess.run([
        '/tmp/diffx', 
        '/tmp/baseline_config', 
        '/tmp/current_config',
        '--ignore-keys-regex', '^(timestamp|lastModified|version)$',
        '--output', 'json'
    ], capture_output=True, text=True)
    
    if result.returncode != 0:
        # Configuration drift detected
        diff_data = json.loads(result.stdout) if result.stdout else []
        
        # Send to SNS for alerting
        sns = boto3.client('sns')
        message = {
            'bucket': bucket,
            'key': key,
            'timestamp': datetime.utcnow().isoformat(),
            'differences': diff_data
        }
        
        sns.publish(
            TopicArn=os.environ['SNS_TOPIC_ARN'],
            Message=json.dumps(message),
            Subject=f'Configuration Drift Detected: {key}'
        )
        
        return {
            'statusCode': 200,
            'body': json.dumps({
                'message': 'Configuration drift detected and alert sent',
                'differences': diff_data
            })
        }
    
    return {'statusCode': 200, 'body': 'No configuration drift detected'}
```

#### CloudFormation Template Validation

```yaml
# cloudformation-config-validator.yaml
AWSTemplateFormatVersion: '2010-09-09'
Description: 'Configuration validation pipeline'

Parameters:
  ConfigBucket:
    Type: String
    Description: S3 bucket containing configuration files

Resources:
  ConfigValidationRole:
    Type: AWS::IAM::Role
    Properties:
      AssumeRolePolicyDocument:
        Version: '2012-10-17'
        Statement:
          - Effect: Allow
            Principal:
              Service: codebuild.amazonaws.com
            Action: sts:AssumeRole
      ManagedPolicyArns:
        - arn:aws:iam::aws:policy/AWSCodeBuildDeveloperAccess
      Policies:
        - PolicyName: S3Access
          PolicyDocument:
            Version: '2012-10-17'
            Statement:
              - Effect: Allow
                Action:
                  - s3:GetObject
                  - s3:PutObject
                Resource: !Sub '${ConfigBucket}/*'

  ConfigValidationProject:
    Type: AWS::CodeBuild::Project
    Properties:
      ServiceRole: !GetAtt ConfigValidationRole.Arn
      Artifacts:
        Type: NO_ARTIFACTS
      Environment:
        Type: LINUX_CONTAINER
        ComputeType: BUILD_GENERAL1_SMALL
        Image: aws/codebuild/amazonlinux2-x86_64-standard:3.0
        EnvironmentVariables:
          - Name: CONFIG_BUCKET
            Value: !Ref ConfigBucket
      Source:
        Type: NO_SOURCE
        BuildSpec: |
          version: 0.2
          phases:
            install:
              runtime-versions:
                rust: 1.70
              commands:
                - cargo install diffx
            build:
              commands:
                - |
                  # Download configurations from S3
                  aws s3 cp s3://$CONFIG_BUCKET/production.json production.json
                  aws s3 cp s3://$CONFIG_BUCKET/staging.json staging.json
                  
                  # Validate consistency
                  diffx production.json staging.json \
                    --ignore-keys-regex "^(environment|host|replicas)$" \
                    --output json > validation_result.json
                  
                  # Upload results
                  aws s3 cp validation_result.json s3://$CONFIG_BUCKET/validation/
                  
                  if [ -s validation_result.json ]; then
                    echo "Configuration inconsistencies found"
                    cat validation_result.json
                    exit 1
                  fi
```

### Azure DevOps

#### Pipeline with Configuration Validation

```yaml
# azure-pipelines.yml
trigger:
  branches:
    include:
    - main
    - develop
  paths:
    include:
    - config/*

pool:
  vmImage: 'ubuntu-latest'

variables:
  DIFFX_VERSION: '0.2.0'

stages:
- stage: Validate
  displayName: 'Configuration Validation'
  jobs:
  - job: ValidateConfigs
    displayName: 'Validate Configuration Files'
    steps:
    - task: Bash@3
      displayName: 'Install diffx'
      inputs:
        targetType: 'inline'
        script: |
          curl -L "https://github.com/kako-jun/diffx/releases/latest/download/diffx-x86_64-unknown-linux-gnu.tar.gz" | tar -xz
          sudo mv diffx /usr/local/bin/
          diffx --version
    
    - task: Bash@3
      displayName: 'Validate Configuration Changes'
      inputs:
        targetType: 'inline'
        script: |
          # Get changed files in this commit
          CHANGED_FILES=$(git diff HEAD~1 HEAD --name-only | grep -E '\.(json|yaml|yml)$' || true)
          
          if [ -n "$CHANGED_FILES" ]; then
            echo "Validating changed configuration files:"
            echo "$CHANGED_FILES"
            
            for file in $CHANGED_FILES; do
              echo "=== Validating $file ==="
              
              # Get previous version
              git show HEAD~1:"$file" > "previous_$file" 2>/dev/null || echo '{}' > "previous_$file"
              
              # Compare versions
              diffx "previous_$file" "$file" \
                --ignore-keys-regex "^(buildId|timestamp|version)$" \
                --output json > "diff_$(echo $file | tr '/' '_').json"
              
              # Process results
              if [ -s "diff_$(echo $file | tr '/' '_').json" ]; then
                echo "Changes detected in $file:"
                cat "diff_$(echo $file | tr '/' '_').json" | jq -r '.[] | 
                  if .Added then "  + \(.Added[0]): \(.Added[1])"
                  elif .Removed then "  - \(.Removed[0]): \(.Removed[1])"
                  elif .Modified then "  ~ \(.Modified[0]): \(.Modified[1]) → \(.Modified[2])"
                  else . end'
              else
                echo "No semantic changes in $file"
              fi
              echo ""
            done
          else
            echo "No configuration files changed"
          fi
    
    - task: PublishBuildArtifacts@1
      displayName: 'Publish Validation Results'
      inputs:
        pathtoPublish: 'diff_*.json'
        artifactName: 'config-validation-results'
      condition: always()
```

### Google Cloud Platform

#### Cloud Function for Configuration Monitoring

```python
# main.py for Google Cloud Function
import json
import subprocess
import tempfile
import os
from google.cloud import storage
from google.cloud import pubsub_v1

def validate_config_change(event, context):
    """Triggered by a change to a Cloud Storage bucket."""
    
    file_name = event['name']
    bucket_name = event['bucket']
    
    if not file_name.endswith(('.json', '.yaml', '.yml')):
        print(f'Ignoring non-config file: {file_name}')
        return
    
    # Download diffx binary (pre-deployed in the function)
    diffx_path = '/workspace/diffx'  # Included in deployment
    
    client = storage.Client()
    bucket = client.bucket(bucket_name)
    
    # Download current file
    blob = bucket.blob(file_name)
    current_config = tempfile.NamedTemporaryFile(mode='w+b', delete=False)
    blob.download_to_filename(current_config.name)
    
    # Download baseline config
    baseline_name = file_name.replace('current/', 'baseline/')
    try:
        baseline_blob = bucket.blob(baseline_name)
        baseline_config = tempfile.NamedTemporaryFile(mode='w+b', delete=False)
        baseline_blob.download_to_filename(baseline_config.name)
    except:
        print(f'No baseline found for {file_name}')
        return
    
    # Run diffx
    result = subprocess.run([
        diffx_path,
        baseline_config.name,
        current_config.name,
        '--ignore-keys-regex', '^(timestamp|gcp_metadata)$',
        '--output', 'json'
    ], capture_output=True, text=True)
    
    # Clean up temp files
    os.unlink(current_config.name)
    os.unlink(baseline_config.name)
    
    if result.returncode != 0:
        # Configuration drift detected
        diff_data = json.loads(result.stdout) if result.stdout else []
        
        # Publish to Pub/Sub for alerting
        publisher = pubsub_v1.PublisherClient()
        topic_path = publisher.topic_path(
            os.environ['GCP_PROJECT'], 
            os.environ['PUBSUB_TOPIC']
        )
        
        message_data = {
            'bucket': bucket_name,
            'file': file_name,
            'differences': diff_data
        }
        
        publisher.publish(topic_path, json.dumps(message_data).encode('utf-8'))
        print(f'Configuration drift detected in {file_name}')
    else:
        print(f'No drift detected in {file_name}')
```

## Monitoring & Alerting

### Prometheus Integration

#### Configuration Drift Exporter

```python
#!/usr/bin/env python3
# config_drift_exporter.py

import time
import subprocess
import json
import os
from prometheus_client import start_http_server, Gauge, Counter, Info
import schedule

# Prometheus metrics
config_drift_detected = Gauge('config_drift_detected', 'Configuration drift detected', ['config_file'])
config_validation_errors = Counter('config_validation_errors_total', 'Total configuration validation errors')
config_last_check = Gauge('config_last_check_timestamp', 'Last configuration check timestamp')
config_info = Info('config_info', 'Configuration file information')

class ConfigDriftMonitor:
    def __init__(self, config_dir='/etc/app/config'):
        self.config_dir = config_dir
        self.baseline_dir = os.path.join(config_dir, 'baseline')
        self.current_dir = os.path.join(config_dir, 'current')
    
    def check_drift(self):
        """Check for configuration drift in all monitored files."""
        try:
            config_files = [f for f in os.listdir(self.current_dir) 
                          if f.endswith(('.json', '.yaml', '.yml'))]
            
            for config_file in config_files:
                current_path = os.path.join(self.current_dir, config_file)
                baseline_path = os.path.join(self.baseline_dir, config_file)
                
                if not os.path.exists(baseline_path):
                    continue
                
                # Run diffx
                result = subprocess.run([
                    'diffx',
                    baseline_path,
                    current_path,
                    '--ignore-keys-regex', '^(timestamp|pid|uptime)$',
                    '--output', 'json'
                ], capture_output=True, text=True)
                
                if result.returncode != 0:
                    # Drift detected
                    config_drift_detected.labels(config_file=config_file).set(1)
                    
                    # Log details
                    diff_data = json.loads(result.stdout) if result.stdout else []
                    print(f"Drift detected in {config_file}: {len(diff_data)} differences")
                else:
                    config_drift_detected.labels(config_file=config_file).set(0)
            
            config_last_check.set(time.time())
            
        except Exception as e:
            config_validation_errors.inc()
            print(f"Error during configuration check: {e}")

def main():
    monitor = ConfigDriftMonitor()
    
    # Schedule regular checks
    schedule.every(5).minutes.do(monitor.check_drift)
    
    # Start Prometheus metrics server
    start_http_server(8000)
    print("Configuration drift exporter started on port 8000")
    
    # Initial check
    monitor.check_drift()
    
    # Main loop
    while True:
        schedule.run_pending()
        time.sleep(1)

if __name__ == '__main__':
    main()
```

#### Grafana Dashboard Configuration

```json
{
  "dashboard": {
    "title": "Configuration Drift Monitoring",
    "panels": [
      {
        "title": "Configuration Drift Status",
        "type": "stat",
        "targets": [
          {
            "expr": "sum(config_drift_detected) by (config_file)",
            "legendFormat": "{{config_file}}"
          }
        ],
        "fieldConfig": {
          "defaults": {
            "thresholds": {
              "steps": [
                {"color": "green", "value": 0},
                {"color": "red", "value": 1}
              ]
            }
          }
        }
      },
      {
        "title": "Configuration Check Errors",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(config_validation_errors_total[5m])",
            "legendFormat": "Validation Errors/sec"
          }
        ]
      },
      {
        "title": "Last Configuration Check",
        "type": "stat",
        "targets": [
          {
            "expr": "time() - config_last_check_timestamp",
            "legendFormat": "Seconds since last check"
          }
        ]
      }
    ]
  }
}
```

### Slack Integration

#### Configuration Change Notifications

```bash
#!/bin/bash
# slack_config_notifier.sh

SLACK_WEBHOOK_URL="$1"
CONFIG_FILE="$2"
DIFF_FILE="$3"

if [ ! -f "$DIFF_FILE" ] || [ ! -s "$DIFF_FILE" ]; then
    exit 0  # No differences to report
fi

# Parse diff data
DIFF_SUMMARY=$(cat "$DIFF_FILE" | jq -r '
    group_by(keys[0]) | 
    map({
        type: .[0] | keys[0],
        count: length,
        items: map(.[keys[0]][0]) | join(", ")
    }) | 
    map("\(.type): \(.count) (\(.items))") | 
    join("\n")'
)

# Create Slack message
SLACK_MESSAGE=$(cat <<EOF
{
    "text": "Configuration Change Detected",
    "attachments": [
        {
            "color": "warning",
            "fields": [
                {
                    "title": "File",
                    "value": "$CONFIG_FILE",
                    "short": true
                },
                {
                    "title": "Timestamp",
                    "value": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
                    "short": true
                },
                {
                    "title": "Changes Summary",
                    "value": "\`\`\`$DIFF_SUMMARY\`\`\`",
                    "short": false
                }
            ],
            "actions": [
                {
                    "type": "button",
                    "text": "View Details",
                    "url": "$BUILD_URL"
                }
            ]
        }
    ]
}
EOF
)

# Send to Slack
curl -X POST -H 'Content-type: application/json' \
     --data "$SLACK_MESSAGE" \
     "$SLACK_WEBHOOK_URL"
```

## Development Tools

### VSCode Extension Integration

#### diffx Language Server Configuration

```json
// .vscode/settings.json
{
    "diffx.enabled": true,
    "diffx.configPath": ".diffx.toml",
    "diffx.ignorePatterns": [
        "^(timestamp|_.*|createdAt|updatedAt)$"
    ],
    "diffx.autoValidate": true,
    "diffx.outputFormat": "cli",
    "files.associations": {
        "*.diffx": "json",
        ".diffx": "toml"
    }
}
```

#### Tasks Configuration

```json
// .vscode/tasks.json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "diffx: Compare with baseline",
            "type": "shell",
            "command": "diffx",
            "args": [
                "config/baseline.json",
                "${file}",
                "--output", "cli"
            ],
            "group": "test",
            "presentation": {
                "echo": true,
                "reveal": "always",
                "focus": false,
                "panel": "shared"
            },
            "problemMatcher": {
                "pattern": {
                    "regexp": "^([+~!-])\\s+(.*?):\\s+(.*)$",
                    "file": 1,
                    "location": 2,
                    "message": 3
                }
            }
        },
        {
            "label": "diffx: Validate all configs",
            "type": "shell",
            "command": "find",
            "args": [
                "config/",
                "-name", "*.json",
                "-exec", "diffx", "{}", "{}.backup", ";"
            ],
            "group": "test"
        }
    ]
}
```

### IntelliJ IDEA Integration

#### External Tool Configuration

```xml
<!-- File: .idea/tools/External Tools.xml -->
<toolSet name="diffx">
  <tool name="Compare with baseline" showInMainMenu="true" showInEditor="true" showInProject="true" showInSearchPopup="true" disabled="false" useConsole="true" showConsoleOnStdOut="false" showConsoleOnStdErr="false" synchronizeAfterRun="true">
    <exec>
      <option name="COMMAND" value="diffx" />
      <option name="PARAMETERS" value="config/baseline/$FileNameWithoutExtension$.$FileExt$ $FilePath$ --output cli" />
      <option name="WORKING_DIRECTORY" value="$ProjectFileDir$" />
    </exec>
  </tool>
  <tool name="Semantic diff with git" showInMainMenu="true" showInEditor="true" showInProject="false" showInSearchPopup="false" disabled="false" useConsole="true" showConsoleOnStdOut="false" showConsoleOnStdErr="false" synchronizeAfterRun="true">
    <exec>
      <option name="COMMAND" value="bash" />
      <option name="PARAMETERS" value="-c &quot;git show HEAD~1:$FileRelativePath$ | diffx - $FilePath$&quot;" />
      <option name="WORKING_DIRECTORY" value="$ProjectFileDir$" />
    </exec>
  </tool>
</toolSet>
```

## Automation Scripts

### Comprehensive Configuration Management Script

```bash
#!/bin/bash
# config-manager.sh - Comprehensive configuration management with diffx

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_DIR="${CONFIG_DIR:-./config}"
BASELINE_DIR="${BASELINE_DIR:-./config/baseline}"
BACKUP_DIR="${BACKUP_DIR:-./config/backups}"
LOG_FILE="${LOG_FILE:-/var/log/config-manager.log}"

# Default diffx options
DIFFX_IGNORE_REGEX="${DIFFX_IGNORE_REGEX:-^(timestamp|lastModified|createdAt|updatedAt|buildTime|version)$}"
DIFFX_OUTPUT_FORMAT="${DIFFX_OUTPUT_FORMAT:-json}"

# Logging function
log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $*" | tee -a "$LOG_FILE"
}

# Check if diffx is available
check_diffx() {
    if ! command -v diffx &> /dev/null; then
        log "ERROR: diffx not found. Please install with: cargo install diffx"
        exit 1
    fi
}

# Create directory structure
setup_directories() {
    mkdir -p "$CONFIG_DIR" "$BASELINE_DIR" "$BACKUP_DIR"
    log "Directory structure created"
}

# Backup current configurations
backup_configs() {
    local backup_timestamp=$(date +'%Y%m%d_%H%M%S')
    local backup_path="$BACKUP_DIR/$backup_timestamp"
    
    mkdir -p "$backup_path"
    
    find "$CONFIG_DIR" -name "*.json" -o -name "*.yaml" -o -name "*.yml" -o -name "*.toml" | \
    while read -r config_file; do
        if [[ ! "$config_file" =~ backup ]]; then
            cp "$config_file" "$backup_path/"
            log "Backed up: $config_file"
        fi
    done
    
    log "Backup completed: $backup_path"
}

# Validate configuration against baseline
validate_config() {
    local config_file="$1"
    local baseline_file="$BASELINE_DIR/$(basename "$config_file")"
    
    if [[ ! -f "$baseline_file" ]]; then
        log "WARNING: No baseline found for $config_file"
        return 0
    fi
    
    local diff_output=$(mktemp)
    
    if diffx "$baseline_file" "$config_file" \
       --ignore-keys-regex "$DIFFX_IGNORE_REGEX" \
       --output "$DIFFX_OUTPUT_FORMAT" > "$diff_output"; then
        log "✅ $config_file: No semantic differences from baseline"
        rm "$diff_output"
        return 0
    else
        log "⚠️  $config_file: Differences detected from baseline"
        
        # Analyze differences
        local added=$(jq '[.[] | select(.Added)] | length' "$diff_output" 2>/dev/null || echo "0")
        local removed=$(jq '[.[] | select(.Removed)] | length' "$diff_output" 2>/dev/null || echo "0")
        local modified=$(jq '[.[] | select(.Modified)] | length' "$diff_output" 2>/dev/null || echo "0")
        local type_changed=$(jq '[.[] | select(.TypeChanged)] | length' "$diff_output" 2>/dev/null || echo "0")
        
        log "  Added: $added, Removed: $removed, Modified: $modified, Type changed: $type_changed"
        
        # Check for critical changes
        local critical_changes=$(jq '[.[] | select(.Removed or .TypeChanged or 
            (.Modified and (.Modified[0] | 
             contains("security") or contains("auth") or contains("password") or contains("key"))))]' "$diff_output" 2>/dev/null || echo "[]")
        
        if [[ "$critical_changes" != "[]" ]]; then
            log "🔴 CRITICAL: Potentially dangerous changes detected in $config_file"
            echo "$critical_changes" | jq -r '.[] | 
                if .Removed then "  REMOVED: \(.Removed[0])"
                elif .TypeChanged then "  TYPE_CHANGED: \(.TypeChanged[0])"
                elif .Modified then "  MODIFIED: \(.Modified[0])"
                else . end' >> "$LOG_FILE"
            
            # Save detailed diff for review
            cp "$diff_output" "$BACKUP_DIR/critical_diff_$(basename "$config_file")_$(date +'%Y%m%d_%H%M%S').json"
        fi
        
        rm "$diff_output"
        return 1
    fi
}

# Validate all configurations
validate_all() {
    local validation_failed=false
    
    log "Starting configuration validation..."
    
    find "$CONFIG_DIR" -name "*.json" -o -name "*.yaml" -o -name "*.yml" -o -name "*.toml" | \
    while read -r config_file; do
        if [[ ! "$config_file" =~ (baseline|backup) ]]; then
            if ! validate_config "$config_file"; then
                validation_failed=true
            fi
        fi
    done
    
    if [[ "$validation_failed" == "true" ]]; then
        log "❌ Configuration validation completed with issues"
        return 1
    else
        log "✅ All configurations validated successfully"
        return 0
    fi
}

# Update baseline with current configuration
update_baseline() {
    local config_file="$1"
    local baseline_file="$BASELINE_DIR/$(basename "$config_file")"
    
    cp "$config_file" "$baseline_file"
    log "Updated baseline: $baseline_file"
}

# Compare two configuration files
compare_configs() {
    local file1="$1"
    local file2="$2"
    
    log "Comparing $file1 with $file2"
    
    diffx "$file1" "$file2" \
        --ignore-keys-regex "$DIFFX_IGNORE_REGEX" \
        --output cli
}

# Generate configuration report
generate_report() {
    local report_file="${1:-config_report_$(date +'%Y%m%d_%H%M%S').html}"
    
    log "Generating configuration report: $report_file"
    
    cat > "$report_file" << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>Configuration Validation Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .summary { background: #f5f5f5; padding: 10px; border-radius: 5px; }
        .success { color: green; }
        .warning { color: orange; }
        .error { color: red; }
        pre { background: #f0f0f0; padding: 10px; overflow-x: auto; }
    </style>
</head>
<body>
    <h1>Configuration Validation Report</h1>
    <div class="summary">
        <h2>Summary</h2>
        <p>Generated: $(date)</p>
        <p>Configuration Directory: $CONFIG_DIR</p>
        <p>Baseline Directory: $BASELINE_DIR</p>
    </div>
    
    <h2>Validation Results</h2>
EOF
    
    find "$CONFIG_DIR" -name "*.json" -o -name "*.yaml" -o -name "*.yml" -o -name "*.toml" | \
    while read -r config_file; do
        if [[ ! "$config_file" =~ (baseline|backup) ]]; then
            echo "<h3>$(basename "$config_file")</h3>" >> "$report_file"
            
            local baseline_file="$BASELINE_DIR/$(basename "$config_file")"
            if [[ -f "$baseline_file" ]]; then
                local diff_output=$(mktemp)
                if diffx "$baseline_file" "$config_file" \
                   --ignore-keys-regex "$DIFFX_IGNORE_REGEX" \
                   --output json > "$diff_output"; then
                    echo '<p class="success">✅ No differences from baseline</p>' >> "$report_file"
                else
                    echo '<p class="warning">⚠️ Differences detected:</p>' >> "$report_file"
                    echo '<pre>' >> "$report_file"
                    cat "$diff_output" | jq -r '.[] | 
                        if .Added then "+ \(.Added[0]): \(.Added[1])"
                        elif .Removed then "- \(.Removed[0]): \(.Removed[1])"
                        elif .Modified then "~ \(.Modified[0]): \(.Modified[1]) → \(.Modified[2])"
                        elif .TypeChanged then "! \(.TypeChanged[0]): \(.TypeChanged[1]) → \(.TypeChanged[2]) (type changed)"
                        else . end' >> "$report_file"
                    echo '</pre>' >> "$report_file"
                fi
                rm "$diff_output"
            else
                echo '<p class="warning">⚠️ No baseline configuration found</p>' >> "$report_file"
            fi
        fi
    done
    
    echo '</body></html>' >> "$report_file"
    log "Report generated: $report_file"
}

# Main function
main() {
    local command="${1:-help}"
    
    case "$command" in
        "setup")
            setup_directories
            ;;
        "backup")
            check_diffx
            backup_configs
            ;;
        "validate")
            check_diffx
            validate_all
            ;;
        "validate-file")
            check_diffx
            if [[ -z "${2:-}" ]]; then
                echo "Usage: $0 validate-file <config-file>"
                exit 1
            fi
            validate_config "$2"
            ;;
        "update-baseline")
            if [[ -z "${2:-}" ]]; then
                echo "Usage: $0 update-baseline <config-file>"
                exit 1
            fi
            update_baseline "$2"
            ;;
        "compare")
            check_diffx
            if [[ -z "${2:-}" ]] || [[ -z "${3:-}" ]]; then
                echo "Usage: $0 compare <file1> <file2>"
                exit 1
            fi
            compare_configs "$2" "$3"
            ;;
        "report")
            check_diffx
            generate_report "${2:-}"
            ;;
        "monitor")
            check_diffx
            log "Starting configuration monitoring..."
            while true; do
                validate_all || log "⚠️ Configuration validation failed"
                sleep "${MONITOR_INTERVAL:-300}"  # Default: 5 minutes
            done
            ;;
        "help"|*)
            cat << EOF
Configuration Manager with diffx

Usage: $0 <command> [options]

Commands:
    setup                   Create directory structure
    backup                  Backup current configurations
    validate               Validate all configurations against baseline
    validate-file <file>   Validate specific configuration file
    update-baseline <file> Update baseline with current configuration
    compare <file1> <file2> Compare two configuration files
    report [file]          Generate HTML validation report
    monitor                Start continuous monitoring (use MONITOR_INTERVAL env var)
    help                   Show this help message

Environment Variables:
    CONFIG_DIR             Configuration directory (default: ./config)
    BASELINE_DIR           Baseline configuration directory (default: ./config/baseline)
    BACKUP_DIR             Backup directory (default: ./config/backups)
    LOG_FILE               Log file path (default: /var/log/config-manager.log)
    DIFFX_IGNORE_REGEX     Regex pattern for keys to ignore
    DIFFX_OUTPUT_FORMAT    Output format for diffx (default: json)
    MONITOR_INTERVAL       Monitoring interval in seconds (default: 300)

Examples:
    $0 setup
    $0 backup
    $0 validate
    $0 validate-file config/app.json
    $0 compare config/prod.json config/staging.json
    $0 report config_report.html
    MONITOR_INTERVAL=60 $0 monitor
EOF
            ;;
    esac
}

# Run main function with all arguments
main "$@"
```

この包括的な統合ガイドにより、diffx をあらゆる開発・運用環境に効果的に組み込むことができます。次は performance.md を作成しましょう。