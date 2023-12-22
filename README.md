# Minipng
C'est rouillé donc c'est mieux !
> Fait dans le cadre d'un TP de CSC4205 à Telecom Sudparis

## Structure
- `main.rs` &rarr; le fichier qui ouvre un fichier passé en argument et exécute les bonnes commandes pour le tester et afficher tous ses blocs.
- `error.rs` &rarr; définit l'erreur `MalformedFileError` qui est utilisée dans les autres fichiers de code source en cas d'erreur de format sur le fichier d'entrée
- `parser.rs` &rarr; le fichier dans lequel on définit un struct pour chaque type de bloc (`Header`, `Comment`, `DataBlock`, `Palette`) qui va implémenter le trait `Block`
  On a également une fonction `parse_blocks` qui, à partir d'un Vec<u8> représentant les octets du fichier minipng d'entrée et donne en sortie un tuple `(Option<Header>, Vec<Comment>, Vec<DataBlock>, Option<Palette>)` wrappé dans un Result ainsi que deux fonctions visant à vérifier l'intégrité du fichier.
- `display.rs` &rarr; on définit dans ce fichier un struct pour chaque type d'image (`BwImage`, `GsImage`, `PalImage`, `RgbImage`) qui va implémenter de trait Image, qui contient lui même une méthode display qui permettra d'afficher l'image. On va également implémenter le trait fmt::Display pour tous les blocs de type autre que DataBlock.
