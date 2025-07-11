# ����

Snɭ����go`diffx` h�n����������Wy�n(kJDf `diffx` �Ddj\xvyMK��gM��F�W~Y

## !�h

| ��� | .^ | b | ���ƣï� | M�� | -����� |  i( |
|------|------|---------|---------------|-------------|-------------|----------|
| **diffx** | ���ƣï | JSON/YAML/TOML/XML/INI/CSV |  |  |  | � ���� |
| diff | ƭ����� | �nƭ�� | L | L | L |  ,�jƭ��ա�� |
| jq | JSON� | JSON | �� | L | L | JSON�\ |
| yq | YAML� | YAML/JSON | �� | L | L | YAML�\ |
| daff | hb | CSV |  | L | L | CSV/����ɷ����� |
| jsondiff | JSON� | JSON |  | �� | L | JSON(� |
| deep-diff | JavaScript | JSON/�ָ��� |  | L | L | JavaScript��� |

## s0�

### vs �en `diff`

**�endiff:**
```bash
$ diff config_v1.json config_v2.json
< {
<   "name": "myapp",
<   "version": "1.0"
< }
> {
>   "version": "1.1",
>   "name": "myapp"
> }
```

**diffx:**
```bash
$ diffx config_v1.json config_v2.json
~ version: "1.0" -> "1.1"
```

**;jUD:**

| �� | �endiff | diffx |
|------|-----------|-------|
| **�** | LXMnƭ�� | ���ƣï�  |
| **���** | pj��nhWf1J | �	��!� |
| **z}** | �hWf1J | թ���Ȓ!� |
| **�ī��** | �hWf1J | թ���Ȓ!� |
| **�	�** | ƭ��	�hWfh: | �	�hWf1J |
| **M�** | Mn��� | ID�����L)(�� |
| **��b** | ƭ��diff | CLI/JSON/YAML/Unified |

**�endiff�FyM4:**
- N(ƭ��ա��
- �������
- LXMn�LŁ
- � ���njDXj�����

**diffx�FyM4:**
- -�ա��
- API���
- ���������
- � ɭ����

### vs JSON�( `jq`

**jq�c_��	:**
```bash
# �,�n_�n�jjq����
jq -n --argjson a "$(cat file1.json)" --argjson b "$(cat file2.json)" \
  'def diff(a; b): 
    if (a | type) != (b | type) then {type_changed: {from: (a | type), to: (b | type)}}
    elif a == b then empty
    elif (a | type) == "object" then
      (a + b) | to_entries | map(select(.value != a[.key] or .value != b[.key])) |
      from_entries
    else {changed: {from: a, to: b}}
    end;
  diff($a; $b)'
```

**diffx�c_4����	:**
```bash
diffx file1.json file2.json --output json
```

**�:**

| �� | jq | diffx |
|------|-------|-------|
| **�U** | ��j���	 | N����j����	 |
| **fҳ��** | %� | �K |
| **JSON(** | oD | DDH6b	 |
| **D��** | jWKչ����	 | B� |
| **M��** | K՟� | D� |
| **գ���** | Kկ�� | c�h�ѿ�� |
| **��** | ����JSON | pb |

**jq�FyM4:**
- �jJSON	�
- �����h�\
- �����Ѥ���
- JSON(������

**diffx�FyM4:**
- ����j����
- pb����LŁ
- ���ƣï�LykŁ
- -��

### vs YAML�( `yq`

**yq�c_�:**
```bash
# yqkoD��LjOK��LŁ
yq eval '. as $item ireduce ({}; . * $item)' file1.yaml file2.yaml
```

**diffx�(:**
```bash
diffx file1.yaml file2.yaml
```

**�:**

| �� | yq | diffx |
|------|-----|-------|
| **;j(** | YAML� | ���ƣï� |
| **�_�** | P��/K� | ͤƣ� |
| **b����** | YAML/JSON | 6b |
| **���ƣï�** | �� | �h |
| **-�** | jW | B� |

**yq�FyM4:**
- YAML	�
- YAMLK�n�����
- YAML<
- �jYAML�

**diffx�FyM4:**
- YAML�y
- ���b��
- -������
- ���ƣï	���

### vs CSV���( `daff`

**daffn�:**
```bash
daff data1.csv data2.csv
```

**diffxn�:**
```bash
diffx data1.csv data2.csv --array-id-key "id"
```

**�:**

| �� | daff | diffx |
|------|------|-------|
| **&�** | hb��� |  ,�j� ��� |
| **b����** | CSV/TSV | CSV�+�6b |
| **�** | HTML�� | CLI/JSON/YAML |
| **ID��** | P�� | �h���� |
| **q** | y� | N( |

**daff�FyM4:**
- �DCSV/����ɷ��\m
- hb���n�
- ExcelqLŁ
- CSVy������

**diffx�FyM4:**
- �b��
- CSV + ]n�n� ���
- APIqLŁ
- ��������

### vs `jsondiff` (Python)

**jsondiffn�:**
```python
from jsondiff import diff
import json

with open('file1.json') as f1, open('file2.json') as f2:
    diff_result = diff(json.load(f1), json.load(f2))
    print(diff_result)
```

**diffxn�:**
```bash
diffx file1.json file2.json --output json
```

**�:**

| �� | jsondiff | diffx |
|------|----------|-------|
| ** �** | Python���� | CLI��� |
| **q** | Python��� | �n �/����� |
| **b����** | JSON( | 6b |
| **�թ���** | Python� | Rust� |
| **�������** | PythonŁ | X Ф�� |
| **M��** | �,� | ئ |

**jsondiff�FyM4:**
- Pythonͤƣ֢������
- ˁ����ï
- ����Python�
- JSON(��

**diffx�FyM4:**
- ��� ���
- CLI/�����q
- ��oD�թ���LŁ
- pb����

### vs GitnD�diff

**Git diff:**
```bash
git diff HEAD~1 config.json
```

**Git diff + diffx:**
```bash
git show HEAD~1:config.json | diffx - config.json
```

**�:**

| �� | Git diff | Git + diffx |
|------|----------|-------------|
| **q** | ͤƣ� | ���� |
| **�** | L��� | ���ƣï |
| **-�** | P�� | ��� |
| **b�** | jW | B� |
| **fҳ��** | ���D | ����� |

**Gitq�:**
```bash
# .gitconfigk��
[diff "json"]
    textconv = diffx --output unified

# .gitattributesk
*.json diff=json
```

### vs  ��	����

#### JavaScript (`deep-diff`)
```javascript
const diff = require('deep-diff');
const differences = diff(obj1, obj2);
```

#### Python (`deepdiff`)
```python
from deepdiff import DeepDiff
diff = DeepDiff(dict1, dict2)
```

#### Ruby (`hashdiff`)
```ruby
require 'hashdiff'
diff = Hashdiff.diff(hash1, hash2)
```

**diffxhn�:**

| �� |  ����� | diffx |
|------|-------------|-------|
| **q** |  �ͤƣ� | CLI/� |
| **�թ���** | �	 |  �Rust	 |
| **b����** | 8X  | p |
| **�������** |  ��X | X Ф�� |
| **�** |  �ThnAPI |  �W_CLI |
| **������(** |  ��	 | ������ |

## �թ����

### �������

ƹ�ա��: 1MB JSON-�ա��

| ��� | B�sG	 | ���(� |
|------|------------|-------------|
| **diffx** | 5ms | 15MB |
| �endiff | 2ms | 8MB |
| jq�����	 | 150ms | 45MB |
| jsondiff | 80ms | 35MB |
| daff | 25ms | 20MB |

*�: ������o����n�թ���o���� k��	�*

### ������ƣ

| ա�뵤� | diffx | �endiff | jq�����	 |
|-------------|-------|-----------|-------------|
| 1KB | 1ms | 1ms | 15ms |
| 100KB | 3ms | 2ms | 45ms |
| 1MB | 5ms | 8ms | 150ms |
| 10MB | 50ms | 80ms | 1500ms |
| 100MB | 500ms | 800ms | 15s+ |

## _����ï�

### ��_�

| _� | diffx | diff | jq | yq | daff | jsondiff |
|------|-------|------|----|----|------|----------|
| **���ƣï�** |  | L | �� | �� |  |  |
| **pb** |  | L | L | L | L | L |
| **MID��** |  | L | L | L | P�� | �� |
| **c�h�գ���** |  | L | K� | K� | L | L |
| **Epsilon�** |  | L | K� | K� | L | L |
| **ѹգ���** |  | L |  |  | L | L |
| **p��b** |  | L |  |  | P�� | L |

### q_�

| _� | diffx | diff | jq | yq | daff | jsondiff |
|------|-------|------|----|----|------|----------|
| **CLI���** |  |  |  |  |  | L |
| **����** |  | L | L | L | L |  |
| **-�ա��** |  | L | L | L | L | L |
| **��	p** |  | L | L | L | L | L |
| **B����** |  |  |  |  |  | L |
| **Ѥ׵���** |  |  |  |  | P�� | L |

## (%�h

### -��
** ix�: diffx**
- ���ƣï�Ĺ
- pbL ,�
- ��k���
- գ���_�

**��: diff**����jƭ��-�(	

### APIƹ�
** ix�: diffx**
- JSON/YAML����
- ��๿��գ���n!�
- p��b
- CI/CDq

**��: jq**�jJSON�\(	

### ����
** ix�: diffx**� ���	~_o **daff**CSV́	
- �bkodiffx�x�
- �jCSV������kodaff�x�

### ������
** ix�: diff**
- LXM�LŁ
- Gitq
- ���

**diffx�(:** Package.json����n-�ա��

### ������������
** ix�: diffx**
- JSON/CSV�������
- MID��
- 'Mjա���

### DevOps/��������
** ix�: diffx**
- Kubernetes��է��YAML	
- Terraform�KJSON	
- Docker Composeա��
- -������

## �L���

### `diff` K� `diffx` x

**�������:**
```bash
diff config1.json config2.json > changes.txt
```

**�������:**
```bash
diffx config1.json config2.json --output unified > changes.txt
# ~_o���ƣï�(:
diffx config1.json config2.json > semantic_changes.txt
```

### `jq` �K� `diffx` x

**��jq�����:**
```bash
jq -n --argjson a "$(cat file1.json)" --argjson b "$(cat file2.json)" \
  'complex_diff_function($a; $b)'
```

**�����diffx:**
```bash
diffx file1.json file2.json --output json
```

###  ��	���K�

**Pythonjsondiff	:**
```python
# �
from jsondiff import diff
result = diff(data1, data2)

# �
import subprocess
result = subprocess.run(['diffx', 'file1.json', 'file2.json', '--output', 'json'], 
                       capture_output=True, text=True)
diff_data = json.loads(result.stdout)
```

## P�

`diffx` �xvyM4:
- � ���n**���ƣï�**
-  dn���gn**pb����**
- **ئjգ���**h��׷��
- **��k���j**CLI���է��
- pj������gn** �W_�\**

�n���xvyM4:
- **�endiff**:  ,�jƭ��ա������������jLXM�
- **jq/yq**: �j���	�X bny�
- **daff**: �DCSV/hb���͖
- ** �����**: y�n������ ��gn�Dq

`diffx` oƭ�����n�������� 	�n���ƣï�Ĺj�b��g*�fD~Y