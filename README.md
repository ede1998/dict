# dict
A small utility to translate words easily on the command-line.

For the translations the tool queries [dict.cc](https://www.dict.cc).
Any language supported by dict.cc is supported by this tool.

## Usage

```
$ dict rust
German >>>>>>>>>>>>>>>>>>> English
========== TRANSLATIONS ==========
to rust ........................................................................... rosten
to rust ........................................................................... einrosten
to rust ........................................................................... verrosten
rust .............................................................................. Rost {m} [Eisen, Stahl, auch Pflanzen]
rust .............................................................................. Rostfleck {m}
rust [colour] ..................................................................... Rostbraun {n}
```

If you want to translate to another language, you can pass additional parameters:

```
$ dict -l RU EN dictionary
Russian >>>>>>>>>>>>>>>>>>> English
========== TRANSLATIONS ==========
словарь {м} ................ dictionary
```

You can also list available languages.

```
$ dict info available
The following language pairs are available:
DE EN => German - English
...
EN TR => English - Turkish
```

## Languages

You can translate between German or English and a number of other languages.
The following language pairs and abbreviations are available:

### Translate between German and

| Language | Abbreviation  |
|-----------|--------------|
|Bulgarian     | BG |
|Bosnian       | BS |
|Czech         | CS |
|Danish        | DA |
|Greek         | EL |
|English       | EN |
|Esperanto     | EO |
|Spanish       | ES |
|Finnish       | FI |
|French        | FR |
|Croatian      | HR |
|Hungarian     | HU |
|Icelandic     | IS |
|Italian       | IT |
|Latin         | LA |
|Dutch         | NL |
|Norwegian     | NO |
|Polish        | PL |
|Portuguese    | PT |
|Romanian      | RO |
|Russian       | RU |
|Slovak        | SK |
|Albanian      | SQ |
|Serbian       | SR |
|Swedish       | SV |
|Turkish       | TR |

### Translate between English and

| Language | Abbreviation  |
|-----------|--------------|
|Bulgarian  | BG |
|Bosnian    | BS |
|Czech      | CS |
|Danish     | DA |
|German     | DE |
|Greek      | EL |
|Esperanto  | EO |
|Spanish    | ES |
|Finnish    | FI |
|French     | FR |
|Croatian   | HR |
|Hungarian  | HU |
|Icelandic  | IS |
|Italian    | IT |
|Latin      | LA |
|Dutch      | NL |
|Norwegian  | NO |
|Polish     | PL |
|Portuguese | PT |
|Romanian   | RO |
|Russian    | RU |
|Slovak     | SK |
|Albanian   | SQ |
|Serbian    | SR |
|Swedish    | SV |
|Turkish    | TR |
