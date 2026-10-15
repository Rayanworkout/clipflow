Notre daemon écoute le clipboard, et le but est ensuite d'envoyer l'entrée au classifier qui trigger l'action.

La contrainte est que l'on souhaite un système de plugin pour les actions.

On envoie l'entrée de clipboard au classifier puis le classifier dispatch vers les plugins.

On doit donc register les plugins dans un fichier de config pour les activer ou non, et chaque type renvoyé par le classifier (url, opération ...) doit être mappé vers le plugin correspondant.

Un plugin ne peut donc pas être lié à 2 plugins il faudra le vérifier.