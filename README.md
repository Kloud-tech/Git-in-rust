
But : reconstruire étape par étape un mini-Git compatible (objets, refs, arbres, commits…).
- `rgit init` : crée .git/
- `rgit hash-object` : compresse, stocke et renvoie le SHA-1
- `rgit cat-file` : affiche un objet
- `rgit write-tree` / `ls-tree` : arbres
- `rgit commit-tree` / `update-ref` : commits + refs
