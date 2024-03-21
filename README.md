# Rolleur de thème pour la WonderJam de l'UQAC

## Fonctionnement

1. Inséré un fichier teams.txt dans le dossier maître. Il devrait contenir une équipe par ligne. 
Ex :
```
Équipe1
Équipe2
Équipe3
...
Dernière Équipe
```
2. Lancer le programme ```cargo run```
3. Les résultats seront dans output/(re-)rolls.csv
4. Ouvrir dans Excel/Sheets, divisé par ';'(semicolon)

Les rerolls ont une condition. Ils doivent avoir 2 entrés au minimum qui sont différente de l'original. Pour s'assurer qu'un re-roll donne d'autres choix aux équipes.

## TODO

- [ ] Fichier csv pour les styles
- [x] Fichier csv pour les équipes
