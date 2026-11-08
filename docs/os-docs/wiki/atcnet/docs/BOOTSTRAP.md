# Bootstrap Guide

## Eigenen Node starten
```python
from shivaos.net.atcnet import ATCNetNode
node = ATCNetNode("0.0.0.0", 4001)
node.start()
```

## Bootstrap-Node verbinden
```python
node.connect_to_bootstrap("bootstrap.atownchain.io", 5005)
```

## Node-Status prüfen
```python
print(node.get_status())
```
